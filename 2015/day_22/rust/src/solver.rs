use crate::game_state::{GameState, TurnResult};
use crate::spell::Spell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct GameStateWrapper(GameState);

impl Ord for GameStateWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.mana_spent.cmp(&self.0.mana_spent)
    }
}

impl PartialOrd for GameStateWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_minimum_mana_solution(initial_state: GameState) -> Option<GameState> {
    let mut queue = BinaryHeap::new();
    queue.push(GameStateWrapper(initial_state));
    let spells = Spell::all_spells();
    let mut discarded_states = 0;

    while let Some(GameStateWrapper(current_state)) = queue.pop() {
        for spell in &spells {
            let mut new_state = current_state.clone();
            match new_state.make_turn(spell) {
                Ok(TurnResult::BossKilledByEffects) | Ok(TurnResult::BossKilledBySpell) => {
                    println!("\nDiscarded {} states", discarded_states);
                    println!("Paths left: {}", queue.len());
                    return Some(new_state);
                }
                Ok(TurnResult::Ongoing) => {
                    queue.push(GameStateWrapper(new_state));
                }
                Ok(TurnResult::PlayerKilled) | Err(_) => discarded_states += 1,
            }
        }
    }
    None
}
