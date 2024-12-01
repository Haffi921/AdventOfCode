use std::collections::HashMap;

type Person<'a> = &'a str;

type Happiness = i16;

#[derive(Debug)]
struct Seating<'a> {
    people: Vec<Person<'a>>,
    happinesses: HashMap<[Person<'a>; 2], Happiness>,
}

impl<'a> Seating<'a> {
    fn new() -> Self {
        Seating {
            people: Vec::new(),
            happinesses: HashMap::new(),
        }
    }

    fn from_str(input: &'a str) -> Result<Self, String> {
        let mut seating = Seating::new();
        for line in input.lines() {
            seating.add_from_str(line)?;
        }
        Ok(seating)
    }

    fn add_person(&mut self, name: Person<'a>) {
        if !self.people.contains(&name) {
            self.people.push(name);
            self.people.sort();
        }
    }

    fn add_happiness(&mut self, people: [Person<'a>; 2], change: i16) {
        if people[0] == people[1] {
            return;
        }

        self.add_person(people[0]);
        self.add_person(people[1]);

        let mut people = people.clone();
        people.sort();
        if let Some(happiness) = self.happinesses.get_mut(&people) {
            *happiness += change;
        } else {
            self.happinesses.insert(people, change);
        }
    }

    fn add_from_str(&mut self, input: &'a str) -> Result<(), String> {
        let (person_a, person_b, change) = input
            .split_once(" would ")
            .and_then(|(person_a, rest)| {
                rest.split_once(" happiness units by sitting next to ")
                    .map(|(change, person_b)| (person_a, person_b.trim_end_matches('.'), change))
            })
            .and_then(|(person_a, person_b, change)| {
                let change = match change.split_once(char::is_whitespace) {
                    Some(("gain", change)) => change.parse::<i16>().unwrap(),
                    Some(("lose", change)) => -change.parse::<i16>().unwrap(),
                    _ => return None,
                };
                Some((person_a, person_b, change))
            })
            .ok_or_else(|| format!("Error parsing line: {input}"))?;

        self.add_happiness([person_a, person_b], change);

        Ok(())
    }

    fn get_happiness_change(&self, people: [&'a str; 2]) -> i16 {
        let mut people = people.clone();
        people.sort();

        self.happinesses
            .get(&people)
            .copied()
            .unwrap_or_else(|| panic!("Happiness not in list! {:?}", people))
    }

    fn find_seating(
        &self,
        (start_person, current_person): (Person<'a>, Person<'a>),
        unseated_people: Vec<Person<'a>>,
    ) -> i16 {
        if unseated_people.len() == 0 {
            let happiness = self.get_happiness_change([current_person, start_person]);
            return happiness;
        }

        let mut best_happiness = i16::MIN;

        for person in unseated_people.iter() {
            let remaining_people: Vec<Person<'a>> = unseated_people
                .clone()
                .into_iter()
                .filter(|p| p != person)
                .collect();
            let current_happiness = self.get_happiness_change([current_person, person])
                + self.find_seating((start_person, person), remaining_people);
            best_happiness = current_happiness.max(best_happiness);
        }

        best_happiness
    }
}

fn part_1(seating: &Seating) -> i16 {
    seating.find_seating(
        (seating.people[0], seating.people[0]),
        seating.people[1..].to_vec(),
    )
}

fn part_2(seating: &mut Seating) -> i16 {
    let people = seating.people.clone();
    seating.add_person("Me");
    for person in people.iter() {
        seating.add_happiness([person, "Me"], 0);
    }
    seating.find_seating(
        (seating.people[0], seating.people[0]),
        seating.people[1..].to_vec(),
    )
}

fn main() -> Result<(), String> {
    let input = include_str!("../../input.txt");
    let mut seating = Seating::from_str(input)?;
    println!("Part 1: {}", part_1(&seating));
    println!("Part 2: {}", part_2(&mut seating));
    Ok(())
}
