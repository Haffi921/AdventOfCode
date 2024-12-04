fn look_say(string: &str) -> String {
    if string.len() == 0 {
        return String::from("");
    }
    let mut new_string = String::with_capacity(string.len() * 2);
    let mut chars = string.chars();
    let mut prev_char = chars.next().unwrap();
    let mut count: u32 = 1;

    for c in chars {
        if prev_char == c {
            count += 1;
        } else {
            new_string.push_str(&format!("{count}{prev_char}"));
            prev_char = c;
            count = 1;
        }
    }

    new_string.push_str(&format!("{count}{prev_char}"));

    new_string
}

fn part_1(input: &str) -> usize {
    let mut result = input.to_string();
    for _ in 0..40 {
        result = look_say(&result);
    }
    result.len()
}

fn part_2(input: &str) -> usize {
    let mut result = input.to_string();
    for _ in 0..50 {
        result = look_say(&result);
    }
    result.len()
}

fn main() {
    let input = advent_of_code_2015::day_input!("10").trim();
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}
