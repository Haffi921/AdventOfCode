struct FloorIterator<'a> {
    input: &'a str,
    index: usize,
    floor: i32,
}

impl<'a> FloorIterator<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            floor: 0,
        }
    }
}

impl<'a> Iterator for FloorIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() {
            return None;
        }
        self.input
            .get(self.index..)
            .unwrap()
            .chars()
            .next()
            .inspect(|_| self.index += 1)
            .map(|c| {
                self.floor += if c == '(' { 1 } else { -1 };
                self.floor
            })
    }
}

fn part1(input: &str) -> i32 {
    FloorIterator::new(input).last().unwrap()
}

fn part2(input: &str) -> usize {
    FloorIterator::new(input)
        .position(|floor| floor == -1)
        .unwrap()
        + 1
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
