#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LightCoord(usize, usize);

#[derive(Debug)]
struct Instruction {
    command: Command,
    start: LightCoord,
    end: LightCoord,
}

impl Instruction {
    fn from_str(line: &str) -> Self {
        let error = || panic!("Could not parse instructions: {line}");
        let parts: Vec<&str> = line.split_whitespace().collect();

        let (command, coords_start_idx) = match parts[0] {
            "toggle" => (Command::Toggle, 1),
            "turn" => match parts[1] {
                "on" => (Command::TurnOn, 2),
                "off" => (Command::TurnOff, 2),
                _ => error(),
            },
            _ => error(),
        };

        let start_coords: Vec<usize> = parts[coords_start_idx]
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect();

        let end_coords: Vec<usize> = parts[coords_start_idx + 2]
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect();

        if start_coords.len() != 2 || end_coords.len() != 2 {
            error();
        }

        Instruction {
            command,
            start: LightCoord(start_coords[0], start_coords[1]),
            end: LightCoord(end_coords[0], end_coords[1]),
        }
    }

    fn get_coords(&self) -> Vec<LightCoord> {
        let mut coords: Vec<LightCoord> = vec![];
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                coords.push(LightCoord(x, y));
            }
        }
        coords
    }
}

fn part_1(input: &str) -> usize {
    let mut light_grid = [[false; 1000]; 1000];
    for line in input.lines() {
        let instruction = Instruction::from_str(line);
        let coords = instruction.get_coords();
        for coord in coords.into_iter() {
            light_grid[coord.0][coord.1] = match instruction.command {
                Command::TurnOn => true,
                Command::TurnOff => false,
                Command::Toggle => !light_grid[coord.0][coord.1],
            }
        }
    }

    light_grid
        .as_flattened()
        .iter()
        .filter(|light| **light)
        .count()
}

fn part_2(input: &str) -> usize {
    let mut light_grid: Vec<Vec<isize>> = vec![vec![0; 1000]; 1000];
    for line in input.lines() {
        let instruction = Instruction::from_str(line);
        let coords = instruction.get_coords();
        for coord in coords {
            light_grid[coord.0][coord.1] += match instruction.command {
                Command::TurnOn => 1,
                Command::TurnOff => -1,
                Command::Toggle => 2,
            };
            light_grid[coord.0][coord.1] = light_grid[coord.0][coord.1].max(0);
        }
    }

    light_grid
        .iter()
        .map(|n| n.iter().sum::<isize>())
        .sum::<isize>() as usize
}

fn main() {
    let input = advent_of_code_2015::day_input!("06");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
