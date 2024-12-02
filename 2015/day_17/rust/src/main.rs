const TARGET_SUM: usize = 150;

fn find_combinations(
    remaining_containers: &[usize],
    current_sum: usize,
    current_combination: &mut Vec<usize>,
    combinations: &mut Vec<Vec<usize>>,
) {
    if current_sum == TARGET_SUM {
        combinations.push(current_combination.clone());
        return;
    }

    if remaining_containers.is_empty() || current_sum > TARGET_SUM {
        return;
    }

    for (i, &c) in remaining_containers.iter().enumerate() {
        if current_sum + c <= TARGET_SUM {
            current_combination.push(c);
            find_combinations(
                &remaining_containers[i + 1..],
                current_sum + c,
                current_combination,
                combinations,
            );
            current_combination.pop();
        }
    }
}

fn part_1(containers: &[usize]) -> usize {
    let mut combinations = Vec::new();
    find_combinations(containers, 0, &mut Vec::new(), &mut combinations);
    combinations.len()
}

fn part_2(containers: &[usize]) -> usize {
    let mut combinations = Vec::new();
    find_combinations(containers, 0, &mut Vec::new(), &mut combinations);
    combinations
        .iter()
        .filter(|c| c.len() == combinations.iter().map(|c| c.len()).min().unwrap())
        .count()
}

fn main() {
    let containers = include_str!("../../input.txt")
        .lines()
        .map(|container| container.parse::<usize>().expect("Invalid container size"))
        .collect::<Vec<_>>();

    println!("{}", part_1(&containers));
    println!("{}", part_2(&containers));
}
