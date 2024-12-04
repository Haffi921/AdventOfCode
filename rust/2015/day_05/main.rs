fn is_part_1_nice_string(s: &str) -> bool {
    fn is_char_vowel(c: char) -> bool {
        "aeiou".contains(c)
    }

    fn has_disallowed_strings(s: &str) -> bool {
        ["ab", "cd", "pq", "xy"]
            .iter()
            .any(|&disallowed| s.contains(disallowed))
    }

    if has_disallowed_strings(s) {
        return false;
    }

    let mut vowel_count = usize::default();
    let mut prev_char = char::default();
    let mut has_double_char = false;

    for c in s.chars() {
        has_double_char = has_double_char || prev_char == c;
        prev_char = c;
        if is_char_vowel(c) {
            vowel_count += 1
        };

        if vowel_count >= 3 && has_double_char {
            return true;
        }
    }
    false
}

fn is_part_2_nice_string(s: &str) -> bool {
    let mut has_repeating_pair = false;
    let mut letter_sandwich = false;
    for i in 0..s.len() - 1 {
        if !has_repeating_pair {
            let pair = s.get(i..i + 2).unwrap_or_default();
            has_repeating_pair = s.get(i + 2..).unwrap_or_default().contains(pair);
        }

        if !letter_sandwich && s.chars().nth(i) == s.chars().nth(i + 2) {
            letter_sandwich = true;
        }

        if has_repeating_pair && letter_sandwich {
            return true;
        }
    }
    false
}

fn nice_strings<F>(input: &str, predicate: F) -> usize
where
    F: Fn(&str) -> bool,
{
    input.lines().filter(|s| predicate(s)).count()
}

fn main() {
    let input = advent_of_code_2015::day_input!("05");
    println!(
        "Part 1: {} strings",
        nice_strings(&input, is_part_1_nice_string)
    );
    println!(
        "Part 2: {} strings",
        nice_strings(&input, is_part_2_nice_string)
    )
}
