use std::collections::HashMap;

#[derive(Default, Debug)]
struct Register {
    value: usize,
}

struct Program<'a> {
    registers: HashMap<&'a str, Register>,
    instructions: Vec<&'a str>,
}

impl<'a> Program<'a> {
    fn new(instructions: &Vec<&'a str>) -> Self {
        Self {
            registers: HashMap::new(),
            instructions: instructions.clone(),
        }
    }

    fn run(&mut self) {
        let mut line: usize = 0;
        while line < self.instructions.len() {
            let (instruction, params) = self.instructions[line]
                .split_once(" ")
                .map(|(i, p)| (i.trim(), p.trim()))
                .unwrap();

            match instruction {
                "hlf" => {
                    let register = self.registers.entry(params).or_default();
                    register.value /= 2;
                    line += 1;
                }
                "tpl" => {
                    let register = self.registers.entry(params).or_default();
                    register.value *= 3;
                    line += 1;
                }
                "inc" => {
                    let register = self.registers.entry(params).or_default();
                    register.value += 1;
                    line += 1;
                }
                "jmp" => {
                    let offset = params.parse::<isize>().unwrap();
                    line = offset.checked_add(line as isize).unwrap_or_default() as usize;
                }
                "jie" => {
                    let (register_name, offset) = params
                        .split_once(", ")
                        .map(|(r, o)| (r.trim(), o.trim().parse::<isize>().unwrap()))
                        .unwrap();
                    let register = self.registers.entry(register_name).or_default();
                    if register.value % 2 == 0 {
                        line = offset.checked_add(line as isize).unwrap_or_default() as usize;
                    } else {
                        line += 1;
                    }
                }
                "jio" => {
                    let (register_name, offset) = params
                        .split_once(", ")
                        .map(|(r, o)| (r.trim(), o.trim().parse::<isize>().unwrap()))
                        .unwrap();
                    let register = self.registers.entry(register_name).or_default();
                    if register.value == 1 {
                        line = offset.checked_add(line as isize).unwrap_or_default() as usize;
                    } else {
                        line += 1;
                    }
                }
                _ => panic!("Invalid instruction: {}", self.instructions[line]),
            }
        }
    }
}

fn part_1(instructions: &Vec<&str>) -> usize {
    let mut program = Program::new(instructions);
    program.run();
    program.registers.entry("b").or_default().value
}

fn part_2(instructions: &Vec<&str>) -> usize {
    let mut program = Program::new(instructions);
    program.registers.entry("a").or_default().value = 1;
    program.run();
    program.registers.entry("b").or_default().value
}

fn main() {
    let instructions = include_str!("../../input.txt")
        .lines()
        .collect::<Vec<&str>>();

    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));
}
