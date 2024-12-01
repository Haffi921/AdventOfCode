use std::str::Lines;

struct InputString<'a> {
    string: &'a str,
}

impl<'a> InputString<'a> {
    fn input_length(&self) -> usize {
        self.string.len()
    }

    fn in_memory_length(&self) -> usize {
        let mut chars = self.string.trim_matches('"').chars();
        let mut length = 0;
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('\\') => length += 1,
                    Some('"') => length += 1,
                    Some('x') => {
                        chars.next();
                        chars.next();
                        length += 1;
                    }
                    _ => length += 1,
                }
            } else {
                length += 1;
            }
        }
        length
    }

    fn escape_length(&self) -> usize {
        self.string.escape_default().count() + 2
    }
}

fn part_1(input_strings: Lines) -> usize {
    input_strings
        .map(|s| InputString { string: s })
        .map(|input_string| input_string.input_length() - input_string.in_memory_length())
        .sum()
}

fn part_2(input_strings: Lines) -> usize {
    input_strings
        .map(|s| InputString { string: s })
        .map(|input_string| input_string.escape_length() - input_string.input_length())
        .sum()
}

fn main() {
    let input_strings = include_str!("../../input.txt").lines();

    println!("Part 1: {}", part_1(input_strings.clone()));
    println!("Part 2: {}", part_2(input_strings.clone()));
}
