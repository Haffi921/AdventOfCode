mod effect;
mod game_log;
mod game_state;
mod solver;
mod spell;

use game_log::create_game_log;
use game_state::{BossStats, GameDifficulty, GameState, PlayerStats};
use solver::find_minimum_mana_solution;

use std::sync::LazyLock;
static BOSS_STATS: LazyLock<(i32, i32)> = LazyLock::new(|| parse_boss_stats());

fn play_game(player_stats: PlayerStats, boss_stats: BossStats, difficulty: GameDifficulty) {
    let initial_state = GameState::new(player_stats, boss_stats, difficulty);
    match find_minimum_mana_solution(initial_state) {
        Some(winning_state) => {
            println!("Minimum mana required: {}", winning_state.mana_spent);
            println!("\nWinning game log:");
            println!(
                "{}",
                create_game_log(
                    &winning_state.spell_history,
                    player_stats,
                    boss_stats,
                    difficulty
                )
            );
        }
        None => println!("No solution found!"),
    }
}

fn parse_boss_stats() -> (i32, i32) {
    advent_of_code_2015::day_input!("22")
        .split_once("\n")
        .map(|(hp, damage)| {
            (
                hp.split_once("Hit Points: ").unwrap().1.trim(),
                damage.split_once("Damage: ").unwrap().1.trim(),
            )
        })
        .map(|(hp, damage)| (hp.parse().unwrap(), damage.parse().unwrap()))
        .unwrap()
}

fn main() {
    let player_stats = PlayerStats {
        hp: 50,
        armor: 0,
        mana: 500,
    };
    let boss_stats = BossStats {
        hp: BOSS_STATS.0,
        damage: BOSS_STATS.1,
    };

    play_game(player_stats, boss_stats, GameDifficulty::Normal);
    play_game(player_stats, boss_stats, GameDifficulty::Hard);
}
