fn get_factor_list(number: usize) -> Vec<usize> {
    let mut factors: Vec<usize> = vec![1, number];

    let mut i = 2;
    while i * i <= number {
        if number % i == 0 {
            factors.push(i);
            if i * i != number {
                factors.push(number / i);
            }
        }
        i += 1;
    }

    factors
}

fn part_1(target: usize) -> usize {
    for house_nr in 1..target {
        let factor_list = get_factor_list(house_nr);
        let factor_sum: usize = factor_list.iter().sum();
        if factor_sum >= target {
            return house_nr;
        }
    }

    return usize::MIN;
}

fn part_2(target: usize) -> usize {
    for house_nr in 1..target {
        let factor_list = get_factor_list(house_nr);
        let factor_sum: usize = factor_list.iter().filter(|&x| x * 50 >= house_nr).sum();
        if factor_sum >= target {
            return house_nr;
        }
    }

    return usize::MIN;
}

fn main() {
    let target = advent_of_code_2015::day_input!("20")
        .parse::<usize>()
        .unwrap();
    println!("Part 1: {}", part_1(target / 10));
    println!("Part 2: {}", part_2(target / 11));
}
