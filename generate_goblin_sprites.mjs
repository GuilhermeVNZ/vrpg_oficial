import fs from 'fs';
import path from 'path';

async function main() {
    // Read API Key from .env
    let apiKey = '';
    try {
        const envContent = fs.readFileSync('.env', 'utf8');
        const match = envContent.match(/GEMINI_API_KEY=(.*)/);
        if (match) {
            apiKey = match[1].trim();
        }
    } catch (e) {
        console.error('Error reading .env:', e);
        process.exit(1);
    }

    if (!apiKey) {
        console.error('GEMINI_API_KEY not found in .env');
        process.exit(1);
    }

    console.log(`Found API Key: ${apiKey.substring(0, 5)}...`);

    // Load reference images
    const refImage01 = fs.readFileSync('assets-and-models/sprites/monsters/goblin01/idle_01.png');
    const refImage06 = fs.readFileSync('assets-and-models/sprites/monsters/goblin01/idle_06.png');

    const refImage01Base64 = refImage01.toString('base64');
    const refImage06Base64 = refImage06.toString('base64');

    // Enhanced prompts with more natural movement descriptions
    const frames = [
        {
            filename: "idle_07.png",
            prompt: "Dungeons and Dragons goblin warrior character in dark fantasy anime illustration style, viewed from top-down angle. This is frame 7 of a 9-frame idle breathing animation loop. The goblin is standing on a circular stone pedestal base. MOVEMENT: Continuing from the exhale in frame 6, the goblin's head tilts slightly to the right as if scanning the area, shoulders begin to rise as breath starts again, right arm (holding scimitar) lifts subtly upward about 15 degrees, left arm (holding wooden shield) shifts outward slightly. The movement should feel like a warrior staying alert and ready. Maintain the same goblin design: green skin, large pointed ears, rusty iron helmet, tattered leather armor, crude scimitar and wooden shield. Solid dark grey background, NO checkered pattern. Tabletop miniature aesthetic with painted details.",
            useRef: refImage06Base64
        },
        {
            filename: "idle_08.png",
            prompt: "Dungeons and Dragons goblin warrior character in dark fantasy anime illustration style, viewed from top-down angle. This is frame 8 of a 9-frame idle breathing animation loop. The goblin is standing on a circular stone pedestal base. MOVEMENT: Peak of the breathing cycle - chest fully expanded, head turned about 20 degrees to the right (looking to the side), both shoulders raised high, scimitar arm extended outward more noticeably (weapon tip pointing slightly away from body), shield arm pulled closer to body in a protective stance. This is the most dynamic frame showing alertness and readiness. Maintain the same goblin design: green skin, large pointed ears, rusty iron helmet, tattered leather armor, crude scimitar and wooden shield. Solid dark grey background, NO checkered pattern. Tabletop miniature aesthetic with painted details.",
            useRef: refImage06Base64
        },
        {
            filename: "idle_09.png",
            prompt: "Dungeons and Dragons goblin warrior character in dark fantasy anime illustration style, viewed from top-down angle. This is frame 9 of a 9-frame idle breathing animation loop. The goblin is standing on a circular stone pedestal base. MOVEMENT: Transition frame returning to neutral - head rotating back toward center (about 10 degrees from forward), shoulders beginning to drop, chest still slightly expanded but starting to relax, scimitar arm lowering back toward resting position, shield settling back to defensive stance. This frame bridges back to frame 1 to create a smooth loop. Maintain the same goblin design: green skin, large pointed ears, rusty iron helmet, tattered leather armor, crude scimitar and wooden shield. Solid dark grey background, NO checkered pattern. Tabletop miniature aesthetic with painted details.",
            useRef: refImage01Base64
        }
    ];

    const outputDir = path.join('assets-and-models', 'sprites', 'monsters', 'goblin01');

    // Using Imagen 4.0 Ultra model (Nano Banana Pro - working version)
    // Note: imagen-4.0-generate-preview-06-06 returns empty response
    // imagen-4.0-ultra-generate-preview-06-06 works correctly
    const modelName = "imagen-4.0-ultra-generate-preview-06-06";

    for (const frame of frames) {
        console.log(`\nGenerating ${frame.filename}...`);
        console.log(`Prompt: ${frame.prompt.substring(0, 100)}...`);

        try {
            const requestBody = {
                instances: [{
                    prompt: frame.prompt,
                    image: {
                        bytesBase64Encoded: frame.useRef
                    }
                }],
                parameters: {
                    sampleCount: 1,
                    aspectRatio: "1:1",
                    mode: "edit",
                    editMode: "inpaint-insert"
                }
            };

            const response = await fetch(
                `https://generativelanguage.googleapis.com/v1beta/models/${modelName}:predict?key=${apiKey}`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(requestBody)
                }
            );

            if (!response.ok) {
                const errorText = await response.text();
                console.error(`API Error (${response.status}):`, errorText.substring(0, 300));
                continue;
            }

            const data = await response.json();

            let imageData = null;
            if (data.predictions && data.predictions[0]) {
                imageData = data.predictions[0].bytesBase64Encoded || data.predictions[0].image?.bytesBase64Encoded;
            }

            if (imageData) {
                const buffer = Buffer.from(imageData, 'base64');
                const outputPath = path.join(outputDir, frame.filename);
                fs.writeFileSync(outputPath, buffer);
                console.log(`✓ Saved ${frame.filename} (${(buffer.length / 1024).toFixed(1)} KB)`);
            } else {
                console.error(`✗ No image data in response for ${frame.filename}`);
                console.log('Response structure:', JSON.stringify(data).substring(0, 200));
            }

        } catch (error) {
            console.error(`✗ Failed to generate ${frame.filename}:`, error.message);
        }
    }

    console.log('\n✓ Generation complete!');
}

main();
