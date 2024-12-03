use crate::spell::{Spell, SpellType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    pub spell_type: SpellType,
    pub turns_remaining: i32,
}

impl Effect {
    pub fn new(spell: &Spell) -> Option<Self> {
        if spell.duration > 0 {
            Some(Effect {
                spell_type: spell.spell_type,
                turns_remaining: spell.duration,
            })
        } else {
            None
        }
    }
}
