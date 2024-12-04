use serde_json::Value;

fn purge_red(json_value: &Value) -> Value {
    match json_value {
        Value::Object(obj) if obj.values().any(|v| v == "red") => Value::Null,
        Value::Object(obj) => {
            Value::Object(obj.iter().map(|(k, v)| (k.clone(), purge_red(v))).collect())
        }
        Value::Array(arr) => Value::Array(arr.iter().map(purge_red).collect()),
        _ => json_value.clone(),
    }
}

fn sum_numbers(input: &str) -> isize {
    input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<isize>().unwrap())
        .sum()
}

fn part_1(input: &str) -> isize {
    sum_numbers(input)
}

fn part_2(input: &str) -> isize {
    sum_numbers(&serde_json::to_string(&purge_red(&serde_json::from_str(input).unwrap())).unwrap())
}

fn main() {
    let input = advent_of_code_2015::day_input!("12").trim();
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}
