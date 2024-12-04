struct ValidPassword {
    bytes: [u8; 8],
}

impl ValidPassword {
    fn new(current_password: &str) -> Self {
        if !current_password.chars().all(|c| c.is_ascii_lowercase()) {
            panic!("Password must contain only lowercase letters");
        }

        ValidPassword {
            bytes: current_password
                .as_bytes()
                .try_into()
                .expect("Passwords have to be exactly 8 characters long"),
        }
    }

    fn next(&mut self) -> String {
        loop {
            self.increment();
            if self.is_valid() {
                return self.to_string();
            }
        }
    }

    fn to_string(&self) -> String {
        self.bytes.iter().map(|b| *b as char).collect()
    }

    fn increment(&mut self) {
        for i in (0..self.bytes.len()).rev() {
            self.bytes[i] += 1;
            if self.bytes[i] <= b'z' {
                break;
            } else {
                self.bytes[i] = b'a';
            }
        }
    }

    fn is_valid(&self) -> bool {
        self.contains_a_straight()
            && !self.contains_forbidden_letters()
            && self.contains_two_pairs()
    }

    fn contains_a_straight(&self) -> bool {
        self.bytes
            .windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
    }

    fn contains_forbidden_letters(&self) -> bool {
        self.bytes.contains(&b'i') || self.bytes.contains(&b'o') || self.bytes.contains(&b'l')
    }

    fn contains_two_pairs(&self) -> bool {
        self.bytes
            .windows(2)
            .enumerate()
            .filter(|(_, w)| w[0] == w[1])
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
            .windows(2)
            .any(|w| w[1] - w[0] > 1)
    }
}

fn main() {
    let input = advent_of_code_2015::day_input!("11").trim();
    let mut password = ValidPassword::new(input);
    println!("Part 1: {}", password.next());
    println!("Part 2: {}", password.next());
}
