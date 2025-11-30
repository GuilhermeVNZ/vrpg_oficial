//! Integration with Rules5e Service
//!
//! This module provides HTTP client integration with the rules5e-service
//! for resolving combat actions, skill checks, and dice rolls.

use crate::error::{OrchestratorError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Rules5e Service client
pub struct Rules5eClient {
    client: Client,
    base_url: String,
}

impl Rules5eClient {
    /// Create a new Rules5e client
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }

    /// Default client pointing to localhost:3001
    pub fn default() -> Self {
        Self::new("http://localhost:3001".to_string())
    }

    /// Roll dice
    pub async fn roll_dice(
        &self,
        expression: &str,
        seed: Option<u64>,
        advantage: Option<bool>,
        disadvantage: Option<bool>,
    ) -> Result<RollResponse> {
        let request = RollRequest {
            expression: expression.to_string(),
            seed,
            advantage,
            disadvantage,
        };

        let response = self
            .client
            .post(&format!("{}/roll", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!("Rules5e roll request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(OrchestratorError::ServiceError(format!(
                "Rules5e roll failed with status {}: {}",
                status, text
            )));
        }

        let result: RollResponse = response.json().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to parse roll response: {}", e))
        })?;

        Ok(result)
    }

    /// Resolve an attack
    pub async fn resolve_attack(
        &self,
        attack_bonus: i32,
        target_ac: i32,
        advantage: Option<bool>,
        disadvantage: Option<bool>,
        seed: Option<u64>,
    ) -> Result<AttackResponse> {
        let request = AttackRequest {
            attack_bonus,
            target_ac,
            advantage,
            disadvantage,
            seed,
        };

        let response = self
            .client
            .post(&format!("{}/attack", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!("Rules5e attack request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(OrchestratorError::ServiceError(format!(
                "Rules5e attack failed with status {}: {}",
                status, text
            )));
        }

        let result: AttackResponse = response.json().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to parse attack response: {}", e))
        })?;

        Ok(result)
    }

    /// Calculate damage
    pub async fn calculate_damage(
        &self,
        damage_expression: &str,
        damage_type: &str,
        seed: Option<u64>,
    ) -> Result<DamageResponse> {
        let request = DamageRequest {
            damage_expression: damage_expression.to_string(),
            damage_type: damage_type.to_string(),
            seed,
        };

        let response = self
            .client
            .post(&format!("{}/damage", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!("Rules5e damage request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(OrchestratorError::ServiceError(format!(
                "Rules5e damage failed with status {}: {}",
                status, text
            )));
        }

        let result: DamageResponse = response.json().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to parse damage response: {}", e))
        })?;

        Ok(result)
    }

    /// Perform a skill check
    pub async fn skill_check(
        &self,
        skill: &str,
        ability_modifier: i32,
        proficiency_bonus: i32,
        has_proficiency: bool,
        dc: i32,
        advantage: Option<bool>,
        disadvantage: Option<bool>,
        seed: Option<u64>,
    ) -> Result<SkillCheckResponse> {
        let request = SkillCheckRequest {
            skill: skill.to_string(),
            ability_modifier,
            proficiency_bonus,
            has_proficiency,
            dc,
            advantage,
            disadvantage,
            seed,
        };

        let response = self
            .client
            .post(&format!("{}/skills/check", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!(
                    "Rules5e skill check request failed: {}",
                    e
                ))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(OrchestratorError::ServiceError(format!(
                "Rules5e skill check failed with status {}: {}",
                status, text
            )));
        }

        let result: SkillCheckResponse = response.json().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to parse skill check response: {}", e))
        })?;

        Ok(result)
    }
}

// Request/Response types matching rules5e-service API

#[derive(Debug, Serialize, Deserialize)]
struct RollRequest {
    expression: String,
    seed: Option<u64>,
    advantage: Option<bool>,
    disadvantage: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollResponse {
    pub result: RollResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollResult {
    pub total: i32,
    pub natural: Option<i32>,
    pub breakdown: String,
    pub advantage_used: Option<bool>,
    pub disadvantage_used: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttackRequest {
    attack_bonus: i32,
    target_ac: i32,
    advantage: Option<bool>,
    disadvantage: Option<bool>,
    seed: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackResponse {
    pub hit: bool,
    pub critical: bool,
    pub attack_roll: i32,
    pub natural_roll: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DamageRequest {
    damage_expression: String,
    damage_type: String,
    seed: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageResponse {
    pub total_damage: i32,
    pub damage_type: String,
    pub breakdown: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SkillCheckRequest {
    skill: String,
    ability_modifier: i32,
    proficiency_bonus: i32,
    has_proficiency: bool,
    dc: i32,
    advantage: Option<bool>,
    disadvantage: Option<bool>,
    seed: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillCheckResponse {
    pub success: bool,
    pub roll_total: i32,
    pub natural_roll: i32,
    pub dc: i32,
    pub margin: i32,
}
