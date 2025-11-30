use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Persona {
    DungeonMaster,
    Npc(String),
    PlayerIa(String),
    Monster(String),
    Narrator,
}

impl Persona {
    pub fn name(&self) -> &str {
        match self {
            Persona::DungeonMaster => "Dungeon Master",
            Persona::Npc(name) => name,
            Persona::PlayerIa(name) => name,
            Persona::Monster(name) => name,
            Persona::Narrator => "Narrator",
        }
    }
}
