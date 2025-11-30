use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionType {
    Blinded,
    Charmed,
    Deafened,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
    Exhaustion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub applied_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub duration_rounds: Option<u32>,
    pub permanent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionApplication {
    pub condition_type: ConditionType,
    pub duration_rounds: Option<u32>,
    pub permanent: bool,
}

pub struct ConditionManager {
    conditions: Vec<Condition>,
}

impl Default for ConditionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConditionManager {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    pub fn apply(&mut self, application: &ConditionApplication) {
        let now = Utc::now();
        let expires_at = if application.permanent {
            None
        } else {
            application.duration_rounds.map(|rounds| {
                // Assume 6 seconds per round
                now + chrono::Duration::seconds(rounds as i64 * 6)
            })
        };

        let condition = Condition {
            condition_type: application.condition_type,
            applied_at: now,
            expires_at,
            duration_rounds: application.duration_rounds,
            permanent: application.permanent,
        };

        // Remove existing condition of same type
        self.conditions
            .retain(|c| c.condition_type != application.condition_type);

        self.conditions.push(condition);
    }

    pub fn remove(&mut self, condition_type: ConditionType) {
        self.conditions
            .retain(|c| c.condition_type != condition_type);
    }

    pub fn has(&self, condition_type: ConditionType) -> bool {
        self.conditions
            .iter()
            .any(|c| c.condition_type == condition_type)
    }

    pub fn get_all(&self) -> &[Condition] {
        &self.conditions
    }

    pub fn expire_conditions(&mut self) {
        let now = Utc::now();
        self.conditions.retain(|c| {
            if c.permanent {
                true
            } else if let Some(expires_at) = c.expires_at {
                expires_at > now
            } else {
                true
            }
        });
    }

    pub fn clear(&mut self) {
        self.conditions.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_poisoned() {
        let mut manager = ConditionManager::new();
        manager.apply(&ConditionApplication {
            condition_type: ConditionType::Poisoned,
            duration_rounds: Some(10),
            permanent: false,
        });
        assert!(manager.has(ConditionType::Poisoned));
    }

    #[test]
    fn test_condition_stunned() {
        let mut manager = ConditionManager::new();
        manager.apply(&ConditionApplication {
            condition_type: ConditionType::Stunned,
            duration_rounds: Some(5),
            permanent: false,
        });
        assert!(manager.has(ConditionType::Stunned));
    }

    #[test]
    fn test_condition_multiple() {
        let mut manager = ConditionManager::new();
        manager.apply(&ConditionApplication {
            condition_type: ConditionType::Poisoned,
            duration_rounds: Some(10),
            permanent: false,
        });
        manager.apply(&ConditionApplication {
            condition_type: ConditionType::Stunned,
            duration_rounds: Some(5),
            permanent: false,
        });
        assert!(manager.has(ConditionType::Poisoned));
        assert!(manager.has(ConditionType::Stunned));
        assert_eq!(manager.get_all().len(), 2);
    }

    #[test]
    fn test_condition_duplicate() {
        let mut manager = ConditionManager::new();
        manager.apply(&ConditionApplication {
            condition_type: ConditionType::Poisoned,
            duration_rounds: Some(10),
            permanent: false,
        });
        manager.apply(&ConditionApplication {
            condition_type: ConditionType::Poisoned,
            duration_rounds: Some(5),
            permanent: false,
        });
        // Should only have one condition
        assert_eq!(
            manager
                .get_all()
                .iter()
                .filter(|c| c.condition_type == ConditionType::Poisoned)
                .count(),
            1
        );
    }

    #[test]
    fn test_condition_remove() {
        let mut manager = ConditionManager::new();
        manager.apply(&ConditionApplication {
            condition_type: ConditionType::Poisoned,
            duration_rounds: Some(10),
            permanent: false,
        });
        manager.remove(ConditionType::Poisoned);
        assert!(!manager.has(ConditionType::Poisoned));
    }

    #[test]
    fn test_condition_permanent() {
        let mut manager = ConditionManager::new();
        manager.apply(&ConditionApplication {
            condition_type: ConditionType::Blinded,
            duration_rounds: None,
            permanent: true,
        });
        assert!(manager.has(ConditionType::Blinded));
        manager.expire_conditions();
        // Permanent conditions should not expire
        assert!(manager.has(ConditionType::Blinded));
    }
}
