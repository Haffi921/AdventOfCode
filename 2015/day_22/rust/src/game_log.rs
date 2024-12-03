use crate::effect::Effect;
use crate::game_state::{BossStats, GameDifficulty, GameState, PlayerStats, TurnResult};
use crate::spell::{Spell, SpellType};

fn get_start_of_turn_log(game_state: &GameState, turn: &str, player: &PlayerStats) -> String {
    let mut log = String::new();
    log.push_str(&format!("\n-- {} --\n", turn));
    log.push_str(&format!(
        "- Player has {} hit points, {} armor, {} mana ({} mana spent)\n",
        game_state.player.hp, player.armor, game_state.player.mana, game_state.mana_spent
    ));
    log.push_str(&format!("- Boss has {} hit points\n", game_state.boss.hp));
    if game_state.difficulty == GameDifficulty::Hard {
        log.push_str("Player loses 1 hp.\n");
    }
    log
}

fn get_effects_log(
    effects: &Vec<Effect>,
    mut player: PlayerStats,
    mut boss: BossStats,
) -> (String, PlayerStats, BossStats) {
    player.armor = 0;
    let effects_log = effects
        .iter()
        .map(|e| match e.spell_type {
            SpellType::Shield => {
                player.armor = 7;
                format!(
                    "Shield's timer is now {}.\n{}",
                    e.turns_remaining - 1,
                    if e.turns_remaining == 1 {
                        "Shield wears off.\n"
                    } else {
                        ""
                    }
                )
            }
            SpellType::Poison => {
                boss.hp -= 3;
                format!(
                    "Poison deals 3 damage (Boss hp: {}); its timer is now {}.\n{}",
                    boss.hp,
                    e.turns_remaining - 1,
                    if e.turns_remaining == 1 {
                        "Poison wears off.\n"
                    } else {
                        ""
                    }
                )
            }
            SpellType::Recharge => {
                player.mana += 101;
                format!(
                    "Recharge provides 101 mana ({} total); its timer is now {}.\n{}",
                    player.mana,
                    e.turns_remaining - 1,
                    if e.turns_remaining == 1 {
                        "Recharge wears off.\n"
                    } else {
                        ""
                    }
                )
            }
            _ => String::new(),
        })
        .collect::<Vec<_>>()
        .join("");
    (effects_log, player, boss)
}

fn get_spell_log(spell: &Spell, player: &PlayerStats, boss: &BossStats) -> String {
    let mut log = String::new();
    log.push_str(&format!(
        "Player casts {} (costs {} mana).\n",
        spell, spell.mana_cost
    ));
    if spell.duration == 0 {
        if spell.damage > 0 {
            log.push_str(&format!(
                "{} deals {} damage (Boss hp: {}).\n",
                spell,
                spell.damage,
                (boss.hp - spell.damage).max(0)
            ));
        }
        if spell.heal > 0 {
            log.push_str(&format!(
                "{} heals for {} (Player hp: {}).\n",
                spell, spell.heal, player.hp
            ));
        }
    }
    log
}

pub fn create_game_log(
    spell_history: &Vec<Spell>,
    starting_player: PlayerStats,
    starting_boss: BossStats,
    difficulty: GameDifficulty,
) -> String {
    let mut log = String::new();
    let mut replay_state = GameState::new(starting_player, starting_boss, difficulty);

    for spell in spell_history {
        let (effects_log, player, boss) = get_effects_log(
            &replay_state.active_effects,
            replay_state.player.clone(),
            replay_state.boss.clone(),
        );
        log.push_str(&get_start_of_turn_log(
            &replay_state,
            "Player turn",
            &player,
        ));
        let spell_log = get_spell_log(spell, &player, &boss);

        match replay_state.make_player_turn(&spell).unwrap() {
            TurnResult::BossKilledByEffects => {
                log.push_str(&effects_log);
                log.push_str(&format!(
                    "This kills the boss, and the player wins! 🎉 Total mana spent: {}\n",
                    replay_state.mana_spent
                ));
                return log;
            }
            TurnResult::BossKilledBySpell => {
                log.push_str(&effects_log);
                log.push_str(&spell_log);
                log.push_str(&format!(
                    "This kills the boss, and the player wins! 🎉 Total mana spent: {}\n",
                    replay_state.mana_spent
                ));
                return log;
            }
            TurnResult::PlayerKilled => {
                log.push_str(&format!("This kills the player, and the boss wins.\n"));
                return log;
            }
            _ => {
                log.push_str(&effects_log);
                log.push_str(&spell_log);
            }
        }

        let (effects_log, player, boss) = get_effects_log(
            &replay_state.active_effects,
            replay_state.player.clone(),
            replay_state.boss.clone(),
        );
        log.push_str(&get_start_of_turn_log(&replay_state, "Boss turn", &player));
        log.push_str(&effects_log);

        let boss_turn_result = replay_state.make_boss_turn().unwrap();
        if boss_turn_result == TurnResult::BossKilledByEffects {
            log.push_str(&format!(
                "This kills the boss, and the player wins! 🎉 Total mana spent: {}\n",
                replay_state.mana_spent
            ));
            return log;
        }

        log.push_str(&format!(
            "Boss attacks for {} damage! ({} - {} armor; Player hp: {})\n",
            (boss.damage - player.armor).max(1),
            boss.damage,
            player.armor,
            player.hp - (boss.damage - player.armor).max(1)
        ));

        if boss_turn_result == TurnResult::PlayerKilled {
            log.push_str("This kills the player, and the boss wins.\n");
            return log;
        }
    }

    log
}
