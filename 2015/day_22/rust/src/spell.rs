use core::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spell {
    pub spell_type: SpellType,
    pub mana_cost: i32,
    pub damage: i32,
    pub heal: i32,
    pub duration: i32,
}

impl Spell {
    pub fn all_spells() -> Vec<Spell> {
        vec![
            Spell {
                spell_type: SpellType::MagicMissile,
                mana_cost: 53,
                damage: 4,
                heal: 0,
                duration: 0,
            },
            Spell {
                spell_type: SpellType::Drain,
                mana_cost: 73,
                damage: 2,
                heal: 2,
                duration: 0,
            },
            Spell {
                spell_type: SpellType::Shield,
                mana_cost: 113,
                damage: 0,
                heal: 0,
                duration: 6,
            },
            Spell {
                spell_type: SpellType::Poison,
                mana_cost: 173,
                damage: 3,
                heal: 0,
                duration: 6,
            },
            Spell {
                spell_type: SpellType::Recharge,
                mana_cost: 229,
                damage: 0,
                heal: 0,
                duration: 5,
            },
        ]
    }
}

impl Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.spell_type == SpellType::MagicMissile {
            write!(f, "Magic Missile")
        } else {
            write!(f, "{:?}", self.spell_type)
        }
    }
}
