#[derive(Debug, Clone, Copy, Default)]
struct Item<'a> {
    name: &'a str,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl<'a> Item<'a> {
    fn from_str(s: &'a str) -> Self {
        let mut parts = s.split(',').map(|s| s.trim());
        Item {
            name: parts.next().unwrap(),
            cost: parts.next().unwrap().parse().unwrap(),
            damage: parts.next().unwrap().parse().unwrap(),
            armor: parts.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum BattleResult {
    Win,
    Loss,
}

impl std::fmt::Display for BattleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait Character {
    fn hit_points(&self) -> u32;
    fn damage(&self) -> u32;
    fn armor(&self) -> u32;
    fn kills_in_turn(&self, other: &dyn Character) -> u32 {
        let damage = (self.damage().checked_sub(other.armor()).unwrap_or(0)).max(1);
        other.hit_points().div_ceil(damage)
    }
    fn battle(&self, other: &dyn Character) -> BattleResult
    where
        Self: Sized,
    {
        if self.kills_in_turn(other) <= other.kills_in_turn(self) {
            BattleResult::Win
        } else {
            BattleResult::Loss
        }
    }
}

struct CharacterStats {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

struct Player {
    stats: CharacterStats,
    weapon: Item<'static>,
    armor: Item<'static>,
    rings: [Item<'static>; 2],
}

impl Player {
    fn new(stats: CharacterStats) -> Self {
        Player {
            stats,
            weapon: Item::default(),
            armor: Item::default(),
            rings: [Item::default(), Item::default()],
        }
    }

    fn equipment_cost(&self) -> u32 {
        self.weapon.cost + self.armor.cost + self.rings[0].cost + self.rings[1].cost
    }

    fn clear_equipment(&mut self) {
        self.weapon = Item::default();
        self.armor = Item::default();
        self.rings = [Item::default(), Item::default()];
    }
}

impl Character for Player {
    fn hit_points(&self) -> u32 {
        self.stats.hit_points
    }

    fn damage(&self) -> u32 {
        self.stats.damage + self.weapon.damage + self.rings[0].damage + self.rings[1].damage
    }

    fn armor(&self) -> u32 {
        self.stats.armor + self.armor.armor + self.rings[0].armor + self.rings[1].armor
    }
}

struct Boss {
    stats: CharacterStats,
}

impl Boss {
    fn parse_boss() -> Self {
        let parts = advent_of_code_2015::day_21_input()
            .1
            .lines()
            .map(|s| s.split(':').nth(1).unwrap().trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Boss {
            stats: CharacterStats {
                hit_points: parts[0],
                damage: parts[1],
                armor: parts[2],
            },
        }
    }
}

impl Character for Boss {
    fn hit_points(&self) -> u32 {
        self.stats.hit_points
    }

    fn damage(&self) -> u32 {
        self.stats.damage
    }

    fn armor(&self) -> u32 {
        self.stats.armor
    }
}

fn parse_shop() -> ([Item<'static>; 5], [Item<'static>; 6], [Item<'static>; 7]) {
    let mut weapons = [Item::default(); 5];
    let mut armors = [Item::default(); 6];
    let mut rings = [Item::default(); 7];

    let mut input = advent_of_code_2015::day_21_input().0.lines();
    for weapon in weapons.iter_mut() {
        *weapon = Item::from_str(input.next().unwrap());
    }

    input.next();
    for armor in armors.iter_mut().take(5) {
        *armor = Item::from_str(input.next().unwrap());
    }
    armors[5] = Item::default();

    input.next();
    for ring in rings.iter_mut().take(6) {
        *ring = Item::from_str(input.next().unwrap());
    }
    rings[6] = Item::default();

    (weapons, armors, rings)
}

fn part_1(
    player: &mut Player,
    boss: &Boss,
    weapons: &[Item<'static>; 5],
    armors: &[Item<'static>; 6],
    rings: &[Item<'static>; 7],
) {
    player.clear_equipment();
    let mut min_cost = u32::MAX;
    let mut min_equipment = [Item::default(); 4];
    for weapon in weapons.iter() {
        player.weapon = *weapon;
        for armor in armors.iter() {
            player.armor = *armor;
            for ring1 in rings.iter() {
                player.rings[0] = *ring1;
                for ring2 in rings.iter() {
                    if ring1.name == ring2.name {
                        continue;
                    }
                    player.rings[1] = *ring2;
                    match player.battle(boss) {
                        BattleResult::Win => {
                            let cost = player.equipment_cost();
                            if cost < min_cost {
                                min_cost = cost;
                                min_equipment = [
                                    player.weapon,
                                    player.armor,
                                    player.rings[0],
                                    player.rings[1],
                                ];
                            }
                        }
                        BattleResult::Loss => {}
                    }
                }
            }
        }
    }

    println!("{}", min_cost);
    for item in min_equipment.iter() {
        println!("{:?}", item);
    }
}

fn part_2(
    player: &mut Player,
    boss: &Boss,
    weapons: &[Item<'static>; 5],
    armors: &[Item<'static>; 6],
    rings: &[Item<'static>; 7],
) {
    player.clear_equipment();
    let mut max_cost = u32::MIN;
    let mut max_equipment = [Item::default(); 4];
    for weapon in weapons.iter() {
        player.weapon = *weapon;
        for armor in armors.iter() {
            player.armor = *armor;
            for ring1 in rings.iter() {
                player.rings[0] = *ring1;
                for ring2 in rings.iter() {
                    if ring1.name == ring2.name {
                        continue;
                    }
                    player.rings[1] = *ring2;
                    match player.battle(boss) {
                        BattleResult::Win => {}
                        BattleResult::Loss => {
                            let cost = player.equipment_cost();
                            if cost > max_cost {
                                max_cost = cost;
                                max_equipment = [
                                    player.weapon,
                                    player.armor,
                                    player.rings[0],
                                    player.rings[1],
                                ];
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", max_cost);
    for item in max_equipment.iter() {
        println!("{:?}", item);
    }
}

fn main() {
    let (weapons, armors, rings) = parse_shop();
    let boss = Boss::parse_boss();
    let mut player = Player::new(CharacterStats {
        hit_points: 100,
        damage: 0,
        armor: 0,
    });
    part_1(&mut player, &boss, &weapons, &armors, &rings);
    part_2(&mut player, &boss, &weapons, &armors, &rings);
}
