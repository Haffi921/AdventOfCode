use std::collections::{HashMap, HashSet};

type Molecule<'a> = &'a str;
type Replacement<'a> = (&'a str, &'a str);
type ReplacementMap<'a> = HashMap<&'a str, Vec<&'a str>>;

fn apply_replacement(molecule: Molecule, (key, value): Replacement, index: usize) -> String {
    let mut generated_molecule = String::from(molecule);
    generated_molecule.replace_range(index..index + key.len(), value);
    generated_molecule
}

fn get_unique_molecules(replacements: &ReplacementMap, molecule: Molecule) -> HashSet<String> {
    let mut unique_molecules = HashSet::new();

    for (key, values) in replacements {
        for i in 0..molecule.len().checked_sub(key.len() - 1).unwrap_or(0) {
            if molecule[i..].starts_with(key) {
                for value in values {
                    unique_molecules.insert(apply_replacement(molecule, (key, value), i));
                }
            }
        }
    }

    unique_molecules
}

fn part_1(replacements: &ReplacementMap, molecule: Molecule) -> usize {
    get_unique_molecules(replacements, molecule).len()
}

fn part_2(replacements: &ReplacementMap, molecule: Molecule) -> usize {
    // Create reverse map where values are keys and keys are values
    let reverse_map: ReplacementMap = replacements
        .iter()
        .flat_map(|(key, values)| values.iter().map(|value| (*value, vec![*key])))
        .fold(HashMap::new(), |mut acc, (value, key)| {
            acc.entry(value).or_default().extend(key);
            acc
        });

    let mut current_molecules = HashSet::from([molecule.to_string()]);
    let mut steps = 0;

    while !current_molecules.contains(&"e".to_string()) {
        steps += 1;
        let new_molecules: HashSet<String> = current_molecules
            .iter()
            .flat_map(|molecule| get_unique_molecules(&reverse_map, molecule))
            .collect();

        if new_molecules.is_empty() {
            panic!("No solution found");
        }

        current_molecules = new_molecules
            .iter()
            .filter(|m| m.len() == new_molecules.iter().min_by_key(|m| m.len()).unwrap().len())
            .cloned()
            .take(7)
            .fold(HashSet::new(), |mut acc, m| {
                acc.insert(m.clone());
                acc
            });
    }

    steps
}

fn main() {
    let input = advent_of_code_2015::day_input!("19");
    let molecule: Molecule = input.lines().last().unwrap();
    let replacements: ReplacementMap = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once(" => ").unwrap())
        .fold(HashMap::new(), |mut acc: ReplacementMap, (key, value)| {
            acc.entry(key).or_default().push(value);
            acc
        });

    println!("Part 1: {}", part_1(&replacements, molecule));
    println!("Part 2: {}", part_2(&replacements, molecule));
}
