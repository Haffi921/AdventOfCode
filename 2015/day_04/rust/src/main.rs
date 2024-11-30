use md5::{compute, Digest};

const PUZZLE_INPUT: &str = "yzbqklnj";

fn find_md5_starting_with(pattern: &str, starting_number: usize) -> (usize, Digest) {
    let mut number: usize = starting_number;
    let mut digest: Digest;
    let mut printed = false;
    loop {
        if number % 50_000 == 0 {
            if printed {
                print!("\x1B[1A\x1B[2K");
            } else {
                printed = true;
            }
            println!("Trying {number}");
        }
        digest = compute(format!("{PUZZLE_INPUT}{number}"));

        if format!("{:?}", digest).starts_with(pattern) {
            break (number, digest);
        } else {
            number += 1;
        }

        if number > 50_000_000 {
            panic!("Shouldn't go this far!!")
        }
    }
}

fn main() {
    let format_output = |number: usize, md5: Digest| {
        format!(
            "{PUZZLE_INPUT} in addition with {number} has this hash {:?}",
            md5
        )
    };

    let (number, md5) = find_md5_starting_with("00000", 0);
    println!("Part 1: {}", format_output(number, md5));

    let (number, md5) = find_md5_starting_with("000000", number);
    println!("Part 2: {}", format_output(number, md5));
}
