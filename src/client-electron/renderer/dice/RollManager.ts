import * as THREE from 'three';
import * as CANNON from 'cannon-es';
import { DiceFactory } from './DiceFactory';
import { DiceType } from './DiceConfig';

/** Simple notation parser for expressions like "2d20+3" */
function parseNotation(notation: string): { count: number; type: DiceType; modifier: number } {
    const match = notation.trim().match(/(\d*)d(\d+)([+-]\d+)?/i);
    if (!match) throw new Error(`Invalid dice notation: ${notation}`);
    const count = parseInt(match[1] || '1', 10);
    const faces = parseInt(match[2]!, 10);
    const typeMap: Record<number, DiceType> = {
        4: 'd4',
        6: 'd6',
        8: 'd8',
        10: 'd10',
        12: 'd12',
        20: 'd20',
        100: 'd100',
    };
    const type = typeMap[faces];
    if (!type) throw new Error(`Unsupported dice faces: ${faces}`);
    const modifier = match[3] ? parseInt(match[3]!, 10) : 0;
    return { count, type, modifier };
}

/** Result of a single dice roll */
export interface DiceResult {
    value: number;
    faceIndex: number;
}

/** Overall roll result, handling advantage/disadvantage */
export interface RollResult {
    dice: DiceResult[];
    final: number; // after advantage/disadvantage and modifier
    advantage?: boolean;
    disadvantage?: boolean;
    modifier: number;
}

export class RollManager {
    private scene: THREE.Scene;
    private world: CANNON.World;
    private factory: DiceFactory;
    private diceInstances: { mesh: THREE.Mesh; body: CANNON.Body }[] = [];
    private rolling: boolean = false;
    private settlementResolver: (() => void) | null = null;

    constructor(scene: THREE.Scene) {
        this.scene = scene;
        this.world = new CANNON.World();
        this.world.gravity.set(0, 0, -50); // Reduced gravity for better visibility (approx 5g)
        this.world.allowSleep = true; // Allow bodies to sleep so they can settle
        // Use GSSolver for iteration control
        this.world.solver = new CANNON.GSSolver();
        (this.world.solver as any).iterations = 14;
        // contact materials
        const diceMaterial = new CANNON.Material('dice');
        const groundMaterial = new CANNON.Material('ground');
        const diceGroundContact = new CANNON.ContactMaterial(diceMaterial, groundMaterial, {
            friction: 0.1, // Increased friction for better rolling
            restitution: 0.5,
        });
        const diceDiceContact = new CANNON.ContactMaterial(diceMaterial, diceMaterial, {
            friction: 0.1,
            restitution: 0.5, // Reduced restitution to prevent excessive bouncing
        });
        this.world.addContactMaterial(diceGroundContact);
        this.world.addContactMaterial(diceDiceContact);
        this.factory = new DiceFactory();
    }

    /** Create ground plane and surrounding containment walls for dice to stay within bounds. */
    addGround(options?: { transparent?: boolean }) {
        // Ground plane
        const groundGeo = new THREE.PlaneGeometry(20, 20);
        const groundMat = options?.transparent
            ? new THREE.ShadowMaterial({ opacity: 0.5 })
            : new THREE.MeshStandardMaterial({ color: 0x222222, roughness: 0.8 });
        const groundMesh = new THREE.Mesh(groundGeo, groundMat);
        groundMesh.rotation.x = -Math.PI / 2;
        groundMesh.receiveShadow = true;
        this.scene.add(groundMesh);

        const groundShape = new CANNON.Plane();
        const groundBody = new CANNON.Body({ mass: 0, shape: groundShape, material: new CANNON.Material('ground') });
        groundBody.quaternion.setFromEuler(-Math.PI / 2, 0, 0);
        this.world.addBody(groundBody);

        // Containment walls (four thin boxes surrounding the ground)
        const wallThickness = 0.5;
        const wallHeight = 20; // Increased wall height to prevent escapes
        const halfSize = 10; // half of ground size (20/2)
        const wallMaterial = new THREE.MeshStandardMaterial({ color: 0x111111, transparent: true, opacity: 0 }); // invisible
        const wallMaterialPhys = new CANNON.Material('wall');

        const createWall = (position: CANNON.Vec3, rotationY: number) => {
            const wallGeo = new THREE.BoxGeometry(20, wallThickness, wallHeight);
            const wallMesh = new THREE.Mesh(wallGeo, wallMaterial);
            wallMesh.position.set(position.x, position.y, position.z);
            wallMesh.rotation.y = rotationY;
            wallMesh.receiveShadow = true;
            this.scene.add(wallMesh);

            const halfExtents = new CANNON.Vec3(10, wallThickness / 2, wallHeight / 2);
            const wallShape = new CANNON.Box(halfExtents);
            const wallBody = new CANNON.Body({ mass: 0, shape: wallShape, material: wallMaterialPhys });
            wallBody.position.set(position.x, position.y, position.z);
            wallBody.quaternion.setFromEuler(0, rotationY, 0);
            this.world.addBody(wallBody);
        };

        // Front wall (positive Y)
        createWall(new CANNON.Vec3(0, halfSize + wallThickness / 2, wallHeight / 2), 0);
        // Back wall (negative Y)
        createWall(new CANNON.Vec3(0, -halfSize - wallThickness / 2, wallHeight / 2), 0);
        // Right wall (positive X)
        createWall(new CANNON.Vec3(halfSize + wallThickness / 2, 0, wallHeight / 2), Math.PI / 2);
        // Left wall (negative X)
        createWall(new CANNON.Vec3(-halfSize - wallThickness / 2, 0, wallHeight / 2), Math.PI / 2);
    }

    /** Roll dice according to a notation string. Returns a promise that resolves when dice settle. */
    async roll(notation: string, options?: { advantage?: boolean; disadvantage?: boolean }): Promise<RollResult> {
        this.rolling = true; // Ensure rolling is true immediately
        const { count, type, modifier } = parseNotation(notation);
        // Clear previous dice
        this.clearDice();

        // Create dice instances
        for (let i = 0; i < count; i++) {
            const { mesh, body } = await this.factory.createDice(type, this.scene, this.world);
            // Position dice: Spawn high up (z=10) and slightly spread out
            mesh.position.set((Math.random() - 0.5) * 5, (Math.random() - 0.5) * 5, 10 + i * 1.5);
            body.position.set(mesh.position.x, mesh.position.y, mesh.position.z);

            // Apply throw force: Random horizontal spin and slight downward push
            // Force is Impulse (Change in Momentum). Mass is 1.
            const force = new CANNON.Vec3(
                (Math.random() - 0.5) * 10,  // Random X push
                (Math.random() - 0.5) * 10,  // Random Y push
                -5                           // Slight downward push to start fall
            );
            const torque = new CANNON.Vec3(
                (Math.random() - 0.5) * 20,
                (Math.random() - 0.5) * 20,
                (Math.random() - 0.5) * 20
            );

            body.applyImpulse(force, new CANNON.Vec3(0, 0, 0));
            body.applyTorque(torque);
            this.diceInstances.push({ mesh, body });
            console.log(`[RollManager] Created die ${i + 1}/${count} at position:`, mesh.position, 'with force:', force);
        }

        console.log('[RollManager] All dice created, waiting for settlement...');
        // Run simulation until all dice settle
        await this.waitForSettlement();
        console.log('[RollManager] Dice settled!');

        // Detect results
        const diceResults: DiceResult[] = this.diceInstances.map(inst => {
            const up = new THREE.Vector3(0, 0, 1);
            const shape = (inst.body.shapes[0]! as CANNON.ConvexPolyhedron);
            let maxDot = -Infinity;
            let topFace = 0;
            for (let i = 0; i < shape.faceNormals.length; i++) {
                const normal = shape.faceNormals[i]!.clone();
                inst.body.quaternion.vmult(normal, normal);
                const threeNormal = new THREE.Vector3(normal.x, normal.y, normal.z);
                const dot = threeNormal.dot(up);
                if (dot > maxDot) {
                    maxDot = dot;
                    topFace = i;
                }
            }
            // Face index to value mapping – assuming sequential 1..faces
            const value = topFace + 1;
            return { value, faceIndex: topFace };
        });

        // Compute final value with advantage/disadvantage
        let final = diceResults.reduce((sum, r) => sum + r.value, 0) + modifier;
        if (options?.advantage && diceResults.length >= 2) {
            final = Math.max(diceResults[0]!.value, diceResults[1]!.value) + modifier;
        } else if (options?.disadvantage && diceResults.length >= 2) {
            final = Math.min(diceResults[0]!.value, diceResults[1]!.value) + modifier;
        }

        return { dice: diceResults, final, advantage: options?.advantage, disadvantage: options?.disadvantage, modifier };
    }

    /** Public method to update physics - call this from your render loop */
    update(deltaTime: number = 1 / 60) {
        if (!this.rolling) return;

        // Step physics
        this.world.step(deltaTime);

        // Sync meshes with physics bodies
        for (const d of this.diceInstances) {
            d.mesh.position.copy(d.body.position as any);
            d.mesh.quaternion.copy(d.body.quaternion as any);
        }

        // Check if all dice have settled
        const allSleeping = this.diceInstances.every(d => d.body.sleepState === CANNON.Body.SLEEPING);

        // Debug: Log sleep states periodically
        if (Math.random() < 0.01) { // 1% chance each frame
            console.log('[RollManager] Dice sleep states:', this.diceInstances.map(d => ({
                sleeping: d.body.sleepState === CANNON.Body.SLEEPING,
                velocity: d.body.velocity.length(),
                angularVel: d.body.angularVelocity.length(),
                position: { x: d.body.position.x.toFixed(2), y: d.body.position.y.toFixed(2), z: d.body.position.z.toFixed(2) }
            })));
        }

        if (allSleeping && this.settlementResolver) {
            console.log('[RollManager] All dice sleeping, resolving settlement');
            this.rolling = false;
            this.settlementResolver();
            this.settlementResolver = null;
        }
    }

    /** Check if dice are currently rolling */
    get isRolling(): boolean {
        return this.rolling;
    }

    /** Wait for dice to settle - relies on external update() calls */
    private async waitForSettlement(): Promise<void> {
        this.rolling = true;
        return new Promise(resolve => {
            this.settlementResolver = resolve;
        });
    }

    /** Remove dice meshes and bodies from scene/world */
    private clearDice() {
        for (const d of this.diceInstances) {
            this.scene.remove(d.mesh);
            this.world.removeBody(d.body);
        }
        this.diceInstances = [];
    }
}
