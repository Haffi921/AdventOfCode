#[derive(Clone)]
struct PackageGroup {
    packages: Vec<usize>,
}

impl PackageGroup {
    fn find_groups(
        packages: &[usize],
        target_weight: usize,
        num_groups: usize,
    ) -> Option<Vec<PackageGroup>> {
        if num_groups == 1 {
            let group = PackageGroup {
                packages: packages.to_vec(),
            };
            return if group.weight() == target_weight {
                Some(vec![group])
            } else {
                None
            };
        }

        let possible_groups = PackageGroup::possible_groups(packages, target_weight);

        for group in possible_groups {
            let remaining_packages: Vec<usize> = packages
                .iter()
                .copied()
                .filter(|package| !group.packages.contains(package))
                .collect();

            if let Some(mut sub_groups) =
                Self::find_groups(&remaining_packages, target_weight, num_groups - 1)
            {
                sub_groups.insert(0, group);
                return Some(sub_groups);
            }
        }

        None
    }

    fn possible_groups(packages: &[usize], target_weight: usize) -> Vec<PackageGroup> {
        fn find_combinations(
            groups: &mut Vec<PackageGroup>,
            current_group: &mut PackageGroup,
            remaining_packages: &[usize],
            target_weight: usize,
        ) {
            let current_sum: usize = current_group.weight();

            if current_sum == target_weight {
                groups.push(current_group.clone());
                return;
            }

            if current_sum > target_weight || remaining_packages.is_empty() {
                return;
            }

            for i in 0..remaining_packages.len() {
                current_group.packages.push(remaining_packages[i]);
                find_combinations(
                    groups,
                    current_group,
                    &remaining_packages[i + 1..],
                    target_weight,
                );
                current_group.packages.pop();
            }
        }

        let mut groups = Vec::new();
        let mut current_group = PackageGroup {
            packages: Vec::new(),
        };
        find_combinations(&mut groups, &mut current_group, packages, target_weight);

        groups
    }

    fn weight(&self) -> usize {
        self.packages.iter().sum()
    }

    fn quantum_entanglement(&self) -> u128 {
        let mut entanglement: u128 = 1;
        for package in self.packages.iter() {
            entanglement *= *package as u128;
        }
        entanglement
    }
}

fn find_quantum_entanglement(packages: &[usize], nr_groups: usize) -> u128 {
    let target_weight = packages.iter().sum::<usize>() / nr_groups;
    let mut possible_first_groups = PackageGroup::possible_groups(&packages, target_weight);
    possible_first_groups.sort_by(|a, b| a.quantum_entanglement().cmp(&b.quantum_entanglement()));

    for first_group in possible_first_groups.iter() {
        let remaining_packages = packages
            .iter()
            .copied()
            .filter(|package| !first_group.packages.contains(&package))
            .collect::<Vec<usize>>();

        if let Some(_) =
            PackageGroup::find_groups(&remaining_packages, target_weight, nr_groups - 1)
        {
            return first_group.quantum_entanglement();
        }
    }

    panic!("No solution found");
}

fn main() {
    let packages = advent_of_code_2015::day_input!("24")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("Part 1: {}", find_quantum_entanglement(&packages, 3));
    println!("Part 2: {}", find_quantum_entanglement(&packages, 4));
}
