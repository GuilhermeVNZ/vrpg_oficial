use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Damage(i32),
    Heal(i32),
    Condition(String),
    Buff(String, i32),
    Debuff(String, i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub id: Uuid,
    pub name: String,
    pub effect_type: EffectType,
    pub target_id: Uuid,
    pub duration_rounds: Option<u32>,
    pub applied_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl Effect {
    pub fn new(
        name: String,
        effect_type: EffectType,
        target_id: Uuid,
        duration_rounds: Option<u32>,
    ) -> Self {
        let now = Utc::now();
        let expires_at = duration_rounds.map(|rounds| {
            // Assume 6 seconds per round
            now + chrono::Duration::seconds(rounds as i64 * 6)
        });

        Self {
            id: Uuid::new_v4(),
            name,
            effect_type,
            target_id,
            duration_rounds,
            applied_at: now,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn apply_damage(&self) -> Option<i32> {
        if let EffectType::Damage(amount) = self.effect_type {
            Some(amount)
        } else {
            None
        }
    }

    pub fn apply_heal(&self) -> Option<i32> {
        if let EffectType::Heal(amount) = self.effect_type {
            Some(amount)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_creation() {
        let target_id = Uuid::new_v4();
        let effect = Effect::new(
            "Fire Damage".to_string(),
            EffectType::Damage(10),
            target_id,
            Some(3),
        );

        assert_eq!(effect.name, "Fire Damage");
        assert_eq!(effect.target_id, target_id);
        assert!(effect.expires_at.is_some());
    }

    #[test]
    fn test_effect_damage() {
        let target_id = Uuid::new_v4();
        let effect = Effect::new("Fire".to_string(), EffectType::Damage(15), target_id, None);

        assert_eq!(effect.apply_damage(), Some(15));
    }

    #[test]
    fn test_effect_heal() {
        let target_id = Uuid::new_v4();
        let effect = Effect::new("Healing".to_string(), EffectType::Heal(20), target_id, None);

        assert_eq!(effect.apply_heal(), Some(20));
    }
}
