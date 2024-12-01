use std::{
    collections::HashMap,
    ops::{Add, Div, Mul},
};

const SECONDS: u16 = 2503;

struct Reindeer<'a> {
    name: &'a str,
    speed: u16,
    flying_time: u16,
    resting_time: u16,
}

impl<'a> Reindeer<'a> {
    fn new((name, speed, flying_time, resting_time): (&'a str, u16, u16, u16)) -> Self {
        Reindeer {
            name,
            speed,
            flying_time,
            resting_time,
        }
    }

    fn from_str(input: &'a str) -> Self {
        Self::new(
            input
                .split_once(" can fly ")
                .map(|(name, rest)| {
                    let (speed, rest) = rest.split_once(" km/s for ").unwrap();
                    let (flying_time, resting_time) = rest
                        .split_once(" seconds, but then must rest for ")
                        .unwrap();
                    (
                        name,
                        speed
                            .parse()
                            .unwrap_or_else(|e| panic!("Invalid speed: {}", e)),
                        flying_time
                            .parse()
                            .unwrap_or_else(|e| panic!("Invalid flying time: {}", e)),
                        resting_time
                            .trim_end_matches(" seconds.")
                            .parse()
                            .unwrap_or_else(|e| panic!("Invalid resting time: {}", e)),
                    )
                })
                .unwrap_or_else(|| panic!("Invalid reindeer: {}", input)),
        )
    }

    fn flying_distance_after(&self, seconds: u16) -> u16 {
        let cycle_time = self.flying_time + self.resting_time;
        self.speed
            .mul(self.flying_time)
            .mul(seconds.div(cycle_time))
            .add(self.speed.mul(self.flying_time.min(seconds % cycle_time)))
    }
}

fn part_1(reindeers: &[Reindeer<'static>]) -> (&'static str, u16) {
    reindeers
        .iter()
        .map(|r| (r.name, r.flying_distance_after(SECONDS)))
        .max_by_key(|(_, distance)| *distance)
        .unwrap()
}

fn part_2(reindeers: &[Reindeer<'static>]) -> (&'static str, u16) {
    let mut scores = HashMap::new();
    for second in 1..=SECONDS {
        let max_distance = reindeers
            .iter()
            .map(|r| r.flying_distance_after(second))
            .max()
            .unwrap();
        reindeers
            .iter()
            .filter(|r| r.flying_distance_after(second) == max_distance)
            .for_each(|r| *scores.entry(r.name).or_insert(0) += 1);
    }
    scores.into_iter().max_by_key(|(_, score)| *score).unwrap()
}

fn main() {
    let input = include_str!("../../input.txt");
    let reindeers = input.lines().map(Reindeer::from_str).collect::<Vec<_>>();
    println!("{:?}", part_1(&reindeers));
    println!("{:?}", part_2(&reindeers));
}
