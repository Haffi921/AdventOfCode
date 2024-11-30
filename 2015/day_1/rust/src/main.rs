use std::fs;

fn main() {
    let input = fs::read_to_string("../input_1.txt").unwrap();
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }
    println!("Floor: {}", floor);
}
