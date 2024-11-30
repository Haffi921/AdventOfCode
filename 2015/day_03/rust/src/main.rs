use std::{collections::HashSet, fs, str::Chars};

enum Direction {
    North,
    South,
    East,
    West,
}

struct DirectionIterator<'a> {
    chars: Chars<'a>,
}

impl<'a> DirectionIterator<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
        }
    }
}

impl<'a> Iterator for DirectionIterator<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|c| match c {
            '>' => Direction::East,
            '^' => Direction::North,
            '<' => Direction::West,
            'v' => Direction::South,
            _ => unreachable!(),
        })
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct House(i32, i32);

impl House {
    fn get_next_house(&self, direction: Direction) -> House {
        match direction {
            Direction::North => House(self.0, self.1 + 1),
            Direction::South => House(self.0, self.1 - 1),
            Direction::East => House(self.0 + 1, self.1),
            Direction::West => House(self.0 - 1, self.1),
        }
    }
}

type HouseSet = HashSet<House>;

struct Santa {
    house: House,
    house_map: HouseSet,
}

impl Santa {
    fn new() -> Self {
        let mut house_map = HouseSet::new();
        house_map.insert(House(0, 0));
        Self {
            house: House(0, 0),
            house_map,
        }
    }

    fn move_direction(&mut self, direction: Direction) {
        self.house = self.house.get_next_house(direction);
        self.house_map.insert(self.house);
    }
}

fn part_1(input: &str) -> usize {
    let mut santa = Santa::new();
    for direction in DirectionIterator::new(input) {
        santa.move_direction(direction);
    }
    santa.house_map.len()
}

fn part_2(input: &str) -> usize {
    let mut santa = Santa::new();
    let mut robo_santa = Santa::new();
    for (i, direction) in DirectionIterator::new(input).enumerate() {
        if i % 2 == 0 {
            santa.move_direction(direction);
        } else {
            robo_santa.move_direction(direction);
        }
    }
    let mut combined_house_map: HouseSet = santa.house_map.clone();
    for house in robo_santa.house_map {
        combined_house_map.insert(house);
    }
    combined_house_map.len()
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
