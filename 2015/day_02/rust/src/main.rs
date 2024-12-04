use std::str::Lines;

struct ChristmasBox {
    l: i32,
    w: i32,
    h: i32,
}

impl ChristmasBox {
    fn surface_areas(&self) -> [i32; 3] {
        [self.l * self.w, self.w * self.h, self.h * self.l]
    }

    fn wrapping_paper(&self) -> i32 {
        2 * self.surface_areas().iter().sum::<i32>() + self.smallest_side()
    }

    fn smallest_side(&self) -> i32 {
        *self.surface_areas().iter().min().unwrap()
    }

    fn ribbon(&self) -> i32 {
        let mut lengths = [self.l, self.w, self.h];
        lengths.sort();
        2 * (lengths[0] + lengths[1]) + self.volume()
    }

    fn volume(&self) -> i32 {
        self.l * self.w * self.h
    }
}

struct ChristmasBoxIterator<'a> {
    lines: Lines<'a>,
}

impl<'a> ChristmasBoxIterator<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines(),
        }
    }
}

impl<'a> Iterator for ChristmasBoxIterator<'a> {
    type Item = ChristmasBox;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .next()
            .map(|line| {
                line.split('x')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|v| ChristmasBox {
                l: v[0],
                w: v[1],
                h: v[2],
            })
    }
}

fn part1(input: &str) -> i32 {
    let iter = ChristmasBoxIterator::new(input);
    iter.map(|b| b.wrapping_paper()).sum()
}

fn part2(input: &str) -> i32 {
    let iter = ChristmasBoxIterator::new(input);
    iter.map(|b| b.ribbon()).sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {} square feet", part1(&input));
    println!("Part 2: {} feet", part2(&input));
}
