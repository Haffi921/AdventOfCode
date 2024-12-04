use crate::effect::Effect;
use crate::spell::{Spell, SpellType};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameDifficulty {
    Normal,
    Hard,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlayerStats {
    pub hp: i32,
    pub armor: i32,
    pub mana: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BossStats {
    pub hp: i32,
    pub damage: i32,
}

#[derive(Clone, PartialEq, Eq)]
pub struct GameState {
    pub player: PlayerStats,
    pub boss: BossStats,
    pub active_effects: Vec<Effect>,
    pub spell_history: Vec<Spell>,
    pub mana_spent: i32,
    pub difficulty: GameDifficulty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TurnResult {
    BossKilledByEffects,
    BossKilledBySpell,
    PlayerKilled,
    Ongoing,
}

impl GameState {
    pub fn new(player: PlayerStats, boss: BossStats, difficulty: GameDifficulty) -> Self {
        Self {
            player,
            boss,
            active_effects: Vec::new(),
            spell_history: Vec::new(),
            mana_spent: 0,
            difficulty,
        }
    }

    pub fn apply_effects(&mut self) {
        self.player.armor = 0;

        for effect in &mut self.active_effects {
            match effect.spell_type {
                SpellType::Shield => {
                    self.player.armor = 7;
                }
                SpellType::Poison => {
                    self.boss.hp -= 3;
                }
                SpellType::Recharge => {
                    self.player.mana += 101;
                }
                _ => {}
            }
            effect.turns_remaining -= 1;
        }

        self.active_effects.retain(|e| e.turns_remaining > 0);
    }

    pub fn can_cast(&self, spell: &Spell) -> bool {
        if spell.mana_cost > self.player.mana {
            return false;
        }

        if spell.duration > 0 {
            !self
                .active_effects
                .iter()
                .any(|e| e.spell_type == spell.spell_type)
        } else {
            true
        }
    }

    pub fn cast_spell(&mut self, spell: &Spell) -> Result<(), &'static str> {
        if !self.can_cast(spell) {
            return Err("Cannot cast this spell");
        }

        self.player.mana -= spell.mana_cost;
        self.mana_spent += spell.mana_cost;
        if spell.duration == 0 {
            if spell.damage > 0 {
                self.boss.hp -= spell.damage;
            }
            if spell.heal > 0 {
                self.player.hp += spell.heal;
            }
        } else {
            if let Some(effect) = Effect::new(spell) {
                self.active_effects.push(effect);
            }
        }

        Ok(())
    }

    pub fn is_player_dead(&self) -> bool {
        self.player.hp <= 0
    }

    pub fn is_boss_dead(&self) -> bool {
        self.boss.hp <= 0
    }

    pub fn make_player_turn(&mut self, spell: &Spell) -> Result<TurnResult, &'static str> {
        if self.difficulty == GameDifficulty::Hard {
            self.player.hp -= 1;
            if self.is_player_dead() {
                return Ok(TurnResult::PlayerKilled);
            }
        }

        self.apply_effects();
        if self.is_boss_dead() {
            return Ok(TurnResult::BossKilledByEffects);
        }

        self.cast_spell(spell)?;

        if self.is_boss_dead() {
            return Ok(TurnResult::BossKilledBySpell);
        }

        Ok(TurnResult::Ongoing)
    }

    pub fn make_boss_turn(&mut self) -> Result<TurnResult, &'static str> {
        self.apply_effects();
        if self.is_boss_dead() {
            return Ok(TurnResult::BossKilledByEffects);
        }

        let damage = (self.boss.damage - self.player.armor).max(1);
        self.player.hp -= damage;

        if self.is_player_dead() {
            Ok(TurnResult::PlayerKilled)
        } else {
            Ok(TurnResult::Ongoing)
        }
    }

    pub fn make_turn(&mut self, spell: &Spell) -> Result<TurnResult, &'static str> {
        self.spell_history.push(spell.clone());
        self.make_player_turn(spell)?;
        self.make_boss_turn()
    }
}
