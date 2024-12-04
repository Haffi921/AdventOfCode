fn calculate_next_value(previous_value: u64) -> u64 {
    const MULTIPLYER: u64 = 252533;
    const DIVIDER: u64 = 33554393;

    (previous_value * MULTIPLYER) % DIVIDER
}

fn coord_value(row: u32, column: u32) -> u64 {
    const INITIAL_VALUE: u64 = 20151125;

    let diagonal_nr = row + column - 1;
    let number_of_calculations = ((diagonal_nr * (diagonal_nr - 1)) / 2) + column;

    let mut value = INITIAL_VALUE;
    for _ in 2..=number_of_calculations {
        value = calculate_next_value(value);
    }
    value
}

fn parse_input() -> (u32, u32) {
    include_str!("../../input.txt")
        .trim_start_matches(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
        )
        .trim_end_matches(".")
        .split_once(", column ")
        .map(|(row, column)| (row.parse().unwrap(), column.parse().unwrap()))
        .unwrap()
}

fn main() {
    let (row, column) = parse_input();
    println!("{}", coord_value(row, column));
}
