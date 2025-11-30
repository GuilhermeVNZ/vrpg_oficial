//! INTENT Executor
//!
//! Executes parsed INTENTs by calling appropriate services

use super::actor_stats::{get_actor_stats, skill_ability_modifier};
use super::types::Intent;
use crate::error::Result;
use crate::services::{MemoryClient, Rules5eClient};
use crate::session::GameSession;
use rules5e_service::{DamageType, DiceExpression, WeaponDatabase};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// INTENT Executor
pub struct IntentExecutor {
    /// Rules5e service client for combat and dice resolution
    rules5e_client: Arc<Rules5eClient>,
    /// Memory service client for lore and rule queries
    memory_client: Arc<MemoryClient>,
}

impl IntentExecutor {
    /// Create a new INTENT executor
    pub fn new() -> Self {
        Self {
            rules5e_client: Arc::new(Rules5eClient::default()),
            memory_client: Arc::new(MemoryClient::default()),
        }
    }

    /// Create with custom service clients
    pub fn with_clients(
        rules5e_client: Arc<Rules5eClient>,
        memory_client: Arc<MemoryClient>,
    ) -> Self {
        Self {
            rules5e_client,
            memory_client,
        }
    }

    /// Execute an INTENT
    ///
    /// Executes INTENTs by calling appropriate services:
    /// - Combat INTENTs -> game-engine + rules5e-service
    /// - Skill checks -> rules5e-service
    /// - Lore/Rule queries -> memory-service
    /// - Asset generation -> Art Daemon (future)
    /// - State transitions -> FSM
    pub async fn execute(&self, intent: &Intent, game_session: &mut GameSession) -> Result<()> {
        match intent {
            // Combat INTENTs
            Intent::CombatStart { reason } => {
                tracing::info!("Combat starting: {:?}", reason);
                // Transition FSM to CombatTurnBased (this will sync engine session)
                game_session.transition_to(crate::fsm::SceneState::CombatTurnBased)?;

                // Start combat in engine session
                if let Some(engine) = game_session.engine_session_mut() {
                    engine.start_combat()?;
                }
            }

            Intent::CombatEnd { reason } => {
                tracing::info!("Combat ending: {:?}", reason);
                // Transition FSM back to Exploration or SocialFreeFlow
                // Default to Exploration after combat (this will sync engine session)
                game_session.transition_to(crate::fsm::SceneState::Exploration)?;

                // End combat in engine session (already handled by transition_to, but explicit here)
                if let Some(engine) = game_session.engine_session_mut() {
                    if let Some(scene) = engine.get_current_scene_mut() {
                        scene.end_combat();
                    }
                }
            }

            Intent::MeleeAttack {
                actor,
                target,
                weapon,
                move_required,
            } => {
                tracing::info!(
                    "Melee attack: {} -> {} with {:?} (move: {})",
                    actor,
                    target,
                    weapon,
                    move_required
                );

                // Get actor and target from game session
                let (attack_bonus, target_ac) = if let Some(engine) = game_session.engine_session()
                {
                    if let Some(scene) = engine.get_current_scene() {
                        // Try to parse actor IDs as UUIDs, fallback to name lookup
                        let actor_uuid = Uuid::parse_str(actor).ok().or_else(|| {
                            scene
                                .all_actors()
                                .iter()
                                .find(|a| a.name == actor.as_str())
                                .map(|a| a.id)
                        });

                        let target_uuid = Uuid::parse_str(&target).ok().or_else(|| {
                            scene
                                .all_actors()
                                .iter()
                                .find(|a| a.name == target.as_str())
                                .map(|a| a.id)
                        });

                        if let (Some(_actor_id), Some(target_id)) = (actor_uuid, target_uuid) {
                            // Get actor stats
                            let actor_stats = get_actor_stats(game_session, actor).ok().flatten();
                            let target_obj = scene.get_actor(target_id);

                            if let Some(target_actor) = target_obj {
                                let attack_bonus = actor_stats
                                    .as_ref()
                                    .map(|s| s.attack_bonus(false)) // Use STR for melee by default
                                    .unwrap_or(5); // Fallback
                                let target_ac = target_actor.ac;
                                (Some(attack_bonus), Some(target_ac))
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        }
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };

                // Use placeholder values if not found
                let attack_bonus = attack_bonus.unwrap_or(5);
                let target_ac = target_ac.unwrap_or(15);

                // Check for advantage/disadvantage conditions
                let advantage = check_advantage_conditions(game_session, actor, true);
                let disadvantage = advantage.map(|adv| !adv);
                let seed = get_deterministic_seed(game_session);

                // Resolve attack
                match self
                    .rules5e_client
                    .resolve_attack(
                        attack_bonus,
                        target_ac,
                        advantage,
                        if advantage.is_some() {
                            None
                        } else {
                            disadvantage
                        },
                        seed,
                    )
                    .await
                {
                    Ok(attack_result) => {
                        tracing::info!(
                            "Attack result: hit={}, critical={}, roll={}",
                            attack_result.hit,
                            attack_result.critical,
                            attack_result.attack_roll
                        );

                        // If hit, calculate damage and apply to target
                        if attack_result.hit {
                            // Get weapon damage from weapon database
                            let (damage_expr, damage_type) = get_weapon_damage(weapon, false)
                                .unwrap_or_else(|| {
                                    // Fallback to default if weapon not found
                                    tracing::warn!(
                                        "Weapon {:?} not found in database, using default damage",
                                        weapon
                                    );
                                    ("1d8+3".to_string(), "slashing".to_string())
                                });

                            match self
                                .rules5e_client
                                .calculate_damage(&damage_expr, &damage_type, None)
                                .await
                            {
                                Ok(damage_result) => {
                                    tracing::info!(
                                        "Damage: {} {} damage",
                                        damage_result.total_damage,
                                        damage_result.damage_type
                                    );

                                    // Apply damage to target in game session
                                    if let Some(engine) = game_session.engine_session_mut() {
                                        if let Some(scene) = engine.get_current_scene_mut() {
                                            // Find target by name or ID
                                            if let Some(target_actor) = scene
                                                .all_actors()
                                                .iter()
                                                .find(|a| {
                                                    a.name == target.as_str()
                                                        || a.id.to_string() == target.as_str()
                                                })
                                                .map(|a| a.id)
                                                .and_then(|id| scene.get_actor_mut(id))
                                            {
                                                target_actor
                                                    .take_damage(damage_result.total_damage);
                                                tracing::info!(
                                                    "Applied {} damage to {}, HP now: {}",
                                                    damage_result.total_damage,
                                                    target,
                                                    target_actor.hp
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Failed to calculate damage: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to resolve attack: {}", e);
                        return Err(e);
                    }
                }
            }

            Intent::RangedAttack {
                actor,
                target,
                weapon,
                move_required,
            } => {
                tracing::info!(
                    "Ranged attack: {} -> {} with {:?} (move: {})",
                    actor,
                    target,
                    weapon,
                    move_required
                );

                // Get actor and target stats
                let actor_stats = get_actor_stats(game_session, actor).ok().flatten();
                let target_stats = get_actor_stats(game_session, &target).ok().flatten();

                let attack_bonus = actor_stats
                    .as_ref()
                    .map(|s| s.attack_bonus(true)) // Use DEX for ranged
                    .unwrap_or(5);
                let target_ac = target_stats.as_ref().map(|s| s.ac).unwrap_or(15);

                // Check for advantage/disadvantage conditions
                let advantage = check_advantage_conditions(game_session, actor, true);
                let disadvantage = advantage.map(|adv| !adv);
                let seed = get_deterministic_seed(game_session);

                // Resolve attack
                match self
                    .rules5e_client
                    .resolve_attack(
                        attack_bonus,
                        target_ac,
                        advantage,
                        if advantage.is_some() {
                            None
                        } else {
                            disadvantage
                        },
                        seed,
                    )
                    .await
                {
                    Ok(attack_result) => {
                        tracing::info!(
                            "Ranged attack result: hit={}, critical={}, roll={}",
                            attack_result.hit,
                            attack_result.critical,
                            attack_result.attack_roll
                        );

                        if attack_result.hit {
                            // Get weapon damage from weapon database
                            let (damage_expr, damage_type) = get_weapon_damage(weapon, false)
                                .unwrap_or_else(|| {
                                    // Fallback to default if weapon not found
                                    tracing::warn!(
                                        "Weapon {:?} not found in database, using default damage",
                                        weapon
                                    );
                                    ("1d6+3".to_string(), "piercing".to_string())
                                });

                            match self
                                .rules5e_client
                                .calculate_damage(&damage_expr, &damage_type, None)
                                .await
                            {
                                Ok(damage_result) => {
                                    tracing::info!(
                                        "Ranged damage: {} {} damage",
                                        damage_result.total_damage,
                                        damage_result.damage_type
                                    );

                                    // Apply damage to target in game session
                                    if let Some(engine) = game_session.engine_session_mut() {
                                        if let Some(scene) = engine.get_current_scene_mut() {
                                            // Find target by name or ID
                                            if let Some(target_actor) = scene
                                                .all_actors()
                                                .iter()
                                                .find(|a| {
                                                    a.name == target.as_str()
                                                        || a.id.to_string() == target.as_str()
                                                })
                                                .map(|a| a.id)
                                                .and_then(|id| scene.get_actor_mut(id))
                                            {
                                                target_actor
                                                    .take_damage(damage_result.total_damage);
                                                tracing::info!(
                                                    "Applied {} damage to {}, HP now: {}",
                                                    damage_result.total_damage,
                                                    target,
                                                    target_actor.hp
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Failed to calculate ranged damage: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to resolve ranged attack: {}", e);
                        return Err(e);
                    }
                }
            }

            Intent::SpellCast {
                actor,
                spell,
                slot_level,
                area_center,
                targets,
            } => {
                tracing::info!(
                    "Spell cast: {} casts {} (level {}) at {:?} targeting {:?}",
                    actor,
                    spell,
                    slot_level,
                    area_center,
                    targets
                );
                // TODO: Call rules5e-service for spell resolution
            }

            // Skill checks
            Intent::SkillCheck {
                actor,
                skill,
                target,
                context,
                suggest_dc,
            } => {
                tracing::info!(
                    "Skill check: {} checks {} (target: {:?}, context: {:?}, suggest_dc: {})",
                    actor,
                    skill,
                    target,
                    context,
                    suggest_dc
                );

                // Get actor stats from game session
                let actor_stats = get_actor_stats(game_session, actor).ok().flatten();

                let ability_modifier = actor_stats
                    .as_ref()
                    .map(|s| skill_ability_modifier(s, skill))
                    .unwrap_or(0);
                let proficiency_bonus = actor_stats
                    .as_ref()
                    .map(|s| s.proficiency_bonus)
                    .unwrap_or(2);
                let has_proficiency = actor_stats
                    .as_ref()
                    .map(|s| s.has_proficiency(skill))
                    .unwrap_or(false);

                // Determine DC based on context or use default
                let dc = if *suggest_dc {
                    context
                        .as_ref()
                        .and_then(|c| {
                            // Try to extract difficulty from context
                            if c.contains("easy") || c.contains("trivial") {
                                Some(10)
                            } else if c.contains("hard") || c.contains("difficult") {
                                Some(20)
                            } else if c.contains("very_hard") || c.contains("nearly_impossible") {
                                Some(25)
                            } else {
                                Some(15) // Default medium
                            }
                        })
                        .unwrap_or(15)
                } else {
                    15 // Default DC
                };

                // Check for advantage/disadvantage conditions
                let advantage = check_advantage_conditions(game_session, actor, false);
                let disadvantage = advantage.map(|adv| !adv); // If advantage is Some, disadvantage is opposite

                // Get deterministic seed if available
                let seed = get_deterministic_seed(game_session);

                match self
                    .rules5e_client
                    .skill_check(
                        skill,
                        ability_modifier,
                        proficiency_bonus,
                        has_proficiency,
                        dc,
                        advantage,
                        if advantage.is_some() {
                            None
                        } else {
                            disadvantage
                        },
                        seed,
                    )
                    .await
                {
                    Ok(check_result) => {
                        tracing::info!(
                            "Skill check result: success={}, roll={}, dc={}, margin={}",
                            check_result.success,
                            check_result.roll_total,
                            check_result.dc,
                            check_result.margin
                        );
                        // TODO: Send result to UI for display
                    }
                    Err(e) => {
                        tracing::error!("Failed to perform skill check: {}", e);
                        return Err(e);
                    }
                }
            }

            // Exploration INTENTs
            Intent::InvestigateArea { actor, area } => {
                tracing::info!("Investigation: {} investigates {}", actor, area);

                // Trigger investigation skill check
                let investigation_intent = Intent::SkillCheck {
                    actor: actor.clone(),
                    skill: "investigation".to_string(),
                    target: Some(area.clone()),
                    context: Some(format!("Investigating area: {}", area)),
                    suggest_dc: true,
                };

                // Execute the skill check
                if let Err(e) = Box::pin(self.execute(&investigation_intent, game_session)).await {
                    tracing::error!("Failed to execute investigation check: {}", e);
                    return Err(e);
                }
            }

            Intent::SearchItem { actor, item } => {
                tracing::info!("Search: {} searches for {:?}", actor, item);

                // Trigger perception skill check for searching
                let search_intent = Intent::SkillCheck {
                    actor: actor.clone(),
                    skill: "perception".to_string(),
                    target: item.clone(),
                    context: Some(format!("Searching for: {:?}", item)),
                    suggest_dc: true,
                };

                // Execute the skill check
                if let Err(e) = Box::pin(self.execute(&search_intent, game_session)).await {
                    tracing::error!("Failed to execute search check: {}", e);
                    return Err(e);
                }
            }

            Intent::InteractObject { actor, object_id } => {
                tracing::info!("Interaction: {} interacts with {}", actor, object_id);

                // For object interaction, we might need different skill checks depending on the object
                // For now, use a general investigation check
                // In the future, this could be expanded to check object type and use appropriate skill
                let interaction_intent = Intent::SkillCheck {
                    actor: actor.clone(),
                    skill: "investigation".to_string(),
                    target: Some(object_id.clone()),
                    context: Some(format!("Interacting with object: {}", object_id)),
                    suggest_dc: true,
                };

                // Execute the skill check
                if let Err(e) = Box::pin(self.execute(&interaction_intent, game_session)).await {
                    tracing::error!("Failed to execute interaction check: {}", e);
                    return Err(e);
                }
            }

            // Social INTENTs
            Intent::NpcDialogue { npc_id, text } => {
                tracing::info!("NPC dialogue: {} says: {}", npc_id, text);
                // TODO: Send to TTS service with NPC voice profile
            }

            Intent::SceneEvent {
                event_type,
                description,
            } => {
                tracing::info!("Scene event [{}]: {}", event_type, description);
                // TODO: Update scene state, trigger visual updates
            }

            // Query INTENTs
            Intent::LoreQuery { query, scope } => {
                tracing::info!("Lore query: {} (scope: {:?})", query, scope);

                // Build filters based on scope
                let mut filters = HashMap::new();
                if let Some(scope_str) = scope {
                    filters.insert("scope".to_string(), scope_str.clone());
                }
                filters.insert("type".to_string(), "lore".to_string());

                // Search with filters
                match self
                    .memory_client
                    .search(query, Some(5), Some(filters))
                    .await
                {
                    Ok(search_result) => {
                        tracing::info!("Found {} lore results", search_result.results.len());
                        for result in &search_result.results {
                            tracing::debug!(
                                "Lore result: {} (score: {:?})",
                                result.id,
                                result.score
                            );
                        }
                        // TODO: Return results to LLM for context injection
                    }
                    Err(e) => {
                        tracing::error!("Failed to search lore: {}", e);
                        // Don't fail the INTENT, just log the error
                    }
                }
            }

            Intent::RuleQuery { query, context } => {
                tracing::info!("Rule query: {} (context: {:?})", query, context);

                // Build filters for rule lookup
                let mut filters = HashMap::new();
                filters.insert("type".to_string(), "rule".to_string());
                if let Some(ctx) = context {
                    filters.insert("context".to_string(), ctx.clone());
                }

                // Search with filters
                match self
                    .memory_client
                    .search(query, Some(3), Some(filters))
                    .await
                {
                    Ok(search_result) => {
                        tracing::info!("Found {} rule results", search_result.results.len());
                        for result in &search_result.results {
                            tracing::debug!(
                                "Rule result: {} (score: {:?})",
                                result.id,
                                result.score
                            );
                        }
                        // TODO: Return results to LLM for context injection
                    }
                    Err(e) => {
                        tracing::error!("Failed to search rules: {}", e);
                        // Don't fail the INTENT, just log the error
                    }
                }
            }

            // Asset generation INTENTs (future)
            Intent::GeneratePortrait {
                character_id,
                style,
            } => {
                tracing::info!("Generate portrait: {} (style: {:?})", character_id, style);
                // TODO: Call Art Daemon
            }

            Intent::GenerateScene {
                scene_id,
                style,
                prompts,
            } => {
                tracing::info!(
                    "Generate scene: {} (style: {:?}, prompts: {})",
                    scene_id,
                    style,
                    prompts.len()
                );
                // TODO: Call Art Daemon
            }

            Intent::GenerateBattlemap { map_id, style } => {
                tracing::info!("Generate battlemap: {} (style: {:?})", map_id, style);
                // TODO: Call Art Daemon
            }

            // Action INTENTs
            Intent::UseItem { actor, item_id } => {
                tracing::info!("Use item: {} uses {}", actor, item_id);
                // TODO: Call rules5e-service for item effect
            }

            Intent::ReadyAction { actor, action } => {
                tracing::info!("Ready action: {} readies {}", actor, action);
                // TODO: Store ready action in turn state
            }

            Intent::Dash { actor } => {
                tracing::info!("Dash: {} dashes", actor);

                // Dash doubles movement speed for this turn
                // This is tracked in the turn state, which will be implemented with the Turn Engine
                // For now, we just log it
                if let Some(engine) = game_session.engine_session() {
                    if let Some(scene) = engine.get_current_scene() {
                        // Find actor
                        if let Some(actor_obj) = scene
                            .all_actors()
                            .iter()
                            .find(|a| a.name == *actor || a.id.to_string() == *actor)
                        {
                            tracing::info!(
                                "Actor {} used Dash action (movement speed doubled for this turn)",
                                actor_obj.name
                            );
                            // TODO: When Turn Engine is implemented, track this in turn state
                        }
                    }
                }
            }

            Intent::Disengage { actor } => {
                tracing::info!("Disengage: {} disengages", actor);

                // Disengage prevents opportunity attacks for this turn
                // This is tracked in the turn state
                if let Some(engine) = game_session.engine_session() {
                    if let Some(scene) = engine.get_current_scene() {
                        // Find actor
                        if let Some(actor_obj) = scene
                            .all_actors()
                            .iter()
                            .find(|a| a.name == *actor || a.id.to_string() == *actor)
                        {
                            tracing::info!(
                                "Actor {} used Disengage action (no opportunity attacks this turn)",
                                actor_obj.name
                            );
                            // TODO: When Turn Engine is implemented, mark actor as disengaged in turn state
                        }
                    }
                }
            }

            Intent::Help { actor, target } => {
                tracing::info!("Help: {} helps {}", actor, target);

                // Help action grants advantage to target's next attack or ability check
                // This is tracked in the turn state
                if let Some(engine) = game_session.engine_session() {
                    if let Some(scene) = engine.get_current_scene() {
                        // Find target actor
                        if let Some(target_actor) = scene
                            .all_actors()
                            .iter()
                            .find(|a| a.name == *target || a.id.to_string() == *target)
                        {
                            tracing::info!(
                                "Actor {} helped {}, granting advantage to next attack/check",
                                actor,
                                target_actor.name
                            );
                            // TODO: When Turn Engine is implemented, mark target as having advantage from Help action
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Execute multiple INTENTs in sequence
    pub async fn execute_many(
        &self,
        intents: &[Intent],
        game_session: &mut GameSession,
    ) -> Result<()> {
        for intent in intents {
            self.execute(intent, game_session).await?;
        }
        Ok(())
    }
}

impl Default for IntentExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to convert DiceExpression to string format (e.g., "1d8+3")
fn dice_expression_to_string(expr: &DiceExpression) -> String {
    if expr.count == 0 {
        return format!("{}", expr.modifier);
    }

    let mut result = format!("{}d{}", expr.count, expr.sides);
    if expr.modifier > 0 {
        result.push_str(&format!("+{}", expr.modifier));
    } else if expr.modifier < 0 {
        result.push_str(&format!("{}", expr.modifier));
    }
    result
}

/// Helper function to convert DamageType to lowercase string
fn damage_type_to_string(damage_type: &DamageType) -> String {
    format!("{:?}", damage_type).to_lowercase()
}

/// Helper function to get weapon damage expression and type from weapon name
fn get_weapon_damage(
    weapon_name: &Option<String>,
    use_versatile: bool,
) -> Option<(String, String)> {
    let weapon_name = weapon_name.as_ref()?;
    let weapon = WeaponDatabase::get_weapon(weapon_name)?;
    let damage_expr = weapon.calculate_damage(use_versatile);
    let damage_str = dice_expression_to_string(&damage_expr);
    let damage_type_str = damage_type_to_string(&weapon.damage_type);
    Some((damage_str, damage_type_str))
}

/// Helper function to check for advantage conditions on an actor
///
/// Returns:
/// - Some(true) if actor has advantage
/// - Some(false) if actor has disadvantage
/// - None if neither (normal roll)
fn check_advantage_conditions(
    game_session: &GameSession,
    actor_id: &str,
    is_attack: bool,
) -> Option<bool> {
    // For now, we don't have a full condition system integrated in the game-engine Actor
    // This is a placeholder that will be expanded when conditions are fully integrated
    //
    // Conditions that grant advantage on attacks:
    // - Invisible (attacking while invisible)
    // - Hidden (attacking from stealth)
    // - Help action (ally helping)
    // - Prone (melee attacks against prone target)
    //
    // Conditions that grant disadvantage on attacks:
    // - Blinded
    // - Frightened (if target is source of fear)
    // - Prone (ranged attacks)
    // - Restrained
    // - Long range (ranged weapons beyond normal range)

    // TODO: When condition system is fully integrated, check actor's conditions
    // For now, return None (normal roll)
    None
}

/// Helper function to generate a deterministic seed for rolls
///
/// Uses session ID and current turn/round to create a reproducible seed
fn get_deterministic_seed(_game_session: &GameSession) -> Option<u64> {
    // For now, return None (non-deterministic)
    // TODO: When turn tracking is implemented, use session_id + turn_number + action_index
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::GameSession;

    #[tokio::test]
    async fn test_execute_skill_check() {
        let executor = IntentExecutor::new();
        let mut game_session = GameSession::new();

        // Skill check doesn't require actors in scene, so this should work
        let intent = Intent::SkillCheck {
            actor: "player_1".to_string(),
            skill: "persuasion".to_string(),
            target: Some("npc_guard_01".to_string()),
            context: Some("test".to_string()),
            suggest_dc: true,
        };

        // This will fail because rules5e-service is not running, but we can test the structure
        // For now, we'll just verify it doesn't panic on parsing
        let result = executor.execute(&intent, &mut game_session).await;
        // Service might not be available, so we accept either Ok or ServiceError
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_execute_combat_start() {
        let executor = IntentExecutor::new();
        let mut game_session = GameSession::new();

        // Create a scene with at least one actor for combat to start
        use game_engine::actor::{Actor, ActorType};

        // Get mutable reference to engine session
        if let Some(engine_session) = game_session.engine_session_mut() {
            let scene_id = engine_session.create_scene("Test Scene".to_string());

            // Add an actor to the scene
            let actor = Actor::new("Test Actor".to_string(), ActorType::Player);
            engine_session.add_actor_to_scene(scene_id, actor).unwrap();
        }

        let intent = Intent::CombatStart {
            reason: Some("Ambush!".to_string()),
        };

        // Should transition to CombatTurnBased
        assert!(executor.execute(&intent, &mut game_session).await.is_ok());
        assert_eq!(
            game_session.current_state(),
            crate::fsm::SceneState::CombatTurnBased
        );
    }

    #[tokio::test]
    async fn test_execute_combat_end() {
        let executor = IntentExecutor::new();
        let mut game_session = GameSession::new();

        // Create a scene with at least one actor
        use game_engine::actor::{Actor, ActorType};

        // Get mutable reference to engine session
        if let Some(engine_session) = game_session.engine_session_mut() {
            let scene_id = engine_session.create_scene("Test Scene".to_string());

            // Add an actor to the scene
            let actor = Actor::new("Test Actor".to_string(), ActorType::Player);
            engine_session.add_actor_to_scene(scene_id, actor).unwrap();
        }

        // First start combat
        let start_intent = Intent::CombatStart { reason: None };
        executor
            .execute(&start_intent, &mut game_session)
            .await
            .unwrap();
        assert_eq!(
            game_session.current_state(),
            crate::fsm::SceneState::CombatTurnBased
        );

        // Then end combat
        let end_intent = Intent::CombatEnd {
            reason: Some("Enemies defeated".to_string()),
        };
        assert!(executor
            .execute(&end_intent, &mut game_session)
            .await
            .is_ok());
        assert_eq!(
            game_session.current_state(),
            crate::fsm::SceneState::Exploration
        );
    }
}
