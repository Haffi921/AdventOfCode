#[derive(Default)]
struct SueProperties {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl<'a> SueProperties {
    fn from_str(string: &'a str) -> Self {
        let properties = string
            .split_once(": ")
            .map(|(_, properties)| properties.split(", ").collect::<Vec<&str>>())
            .unwrap()
            .iter()
            .map(|property| property.split_once(": ").unwrap())
            .collect::<Vec<(&'a str, &'a str)>>();

        let mut sue = Self {
            ..Default::default()
        };

        for (key, value) in properties {
            match key {
                "children" => sue.children = Some(value.parse().unwrap()),
                "cats" => sue.cats = Some(value.parse().unwrap()),
                "samoyeds" => sue.samoyeds = Some(value.parse().unwrap()),
                "pomeranians" => sue.pomeranians = Some(value.parse().unwrap()),
                "akitas" => sue.akitas = Some(value.parse().unwrap()),
                "vizslas" => sue.vizslas = Some(value.parse().unwrap()),
                "goldfish" => sue.goldfish = Some(value.parse().unwrap()),
                "trees" => sue.trees = Some(value.parse().unwrap()),
                "cars" => sue.cars = Some(value.parse().unwrap()),
                "perfumes" => sue.perfumes = Some(value.parse().unwrap()),
                _ => unreachable!(),
            }
        }

        sue
    }

    fn matches_part_1(&self, mfcsam: &SueProperties) -> bool {
        if let Some(children) = self.children {
            if children != mfcsam.children.unwrap() {
                return false;
            }
        }

        if let Some(cats) = self.cats {
            if cats != mfcsam.cats.unwrap() {
                return false;
            }
        }

        if let Some(samoyeds) = self.samoyeds {
            if samoyeds != mfcsam.samoyeds.unwrap() {
                return false;
            }
        }

        if let Some(pomeranians) = self.pomeranians {
            if pomeranians != mfcsam.pomeranians.unwrap() {
                return false;
            }
        }

        if let Some(akitas) = self.akitas {
            if akitas != mfcsam.akitas.unwrap() {
                return false;
            }
        }

        if let Some(vizslas) = self.vizslas {
            if vizslas != mfcsam.vizslas.unwrap() {
                return false;
            }
        }

        if let Some(goldfish) = self.goldfish {
            if goldfish != mfcsam.goldfish.unwrap() {
                return false;
            }
        }

        if let Some(trees) = self.trees {
            if trees != mfcsam.trees.unwrap() {
                return false;
            }
        }

        if let Some(cars) = self.cars {
            if cars != mfcsam.cars.unwrap() {
                return false;
            }
        }

        if let Some(perfumes) = self.perfumes {
            if perfumes != mfcsam.perfumes.unwrap() {
                return false;
            }
        }

        true
    }

    fn matches_part_2(&self, mfcsam: &SueProperties) -> bool {
        if let Some(children) = self.children {
            if children != mfcsam.children.unwrap() {
                return false;
            }
        }

        if let Some(cats) = self.cats {
            if cats <= mfcsam.cats.unwrap() {
                return false;
            }
        }

        if let Some(samoyeds) = self.samoyeds {
            if samoyeds != mfcsam.samoyeds.unwrap() {
                return false;
            }
        }

        if let Some(pomeranians) = self.pomeranians {
            if pomeranians >= mfcsam.pomeranians.unwrap() {
                return false;
            }
        }

        if let Some(akitas) = self.akitas {
            if akitas != mfcsam.akitas.unwrap() {
                return false;
            }
        }

        if let Some(vizslas) = self.vizslas {
            if vizslas != mfcsam.vizslas.unwrap() {
                return false;
            }
        }

        if let Some(goldfish) = self.goldfish {
            if goldfish >= mfcsam.goldfish.unwrap() {
                return false;
            }
        }

        if let Some(trees) = self.trees {
            if trees <= mfcsam.trees.unwrap() {
                return false;
            }
        }

        if let Some(cars) = self.cars {
            if cars != mfcsam.cars.unwrap() {
                return false;
            }
        }

        if let Some(perfumes) = self.perfumes {
            if perfumes != mfcsam.perfumes.unwrap() {
                return false;
            }
        }

        true
    }
}

fn part_1(mfcsam: &SueProperties, sues: &[SueProperties]) -> String {
    for (i, sue) in sues.iter().enumerate() {
        if sue.matches_part_1(&mfcsam) {
            return format!("Sue {} matches", i + 1);
        }
    }

    return "No matching Sue".to_string();
}

fn part_2(mfcsam: &SueProperties, sues: &[SueProperties]) {
    for (i, sue) in sues.iter().enumerate() {
        if sue.matches_part_2(&mfcsam) {
            println!("Sue {} matches", i + 1);
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let sues = input
        .lines()
        .map(SueProperties::from_str)
        .collect::<Vec<_>>();

    let mfcsam = SueProperties {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    println!("{}", part_1(&mfcsam, &sues));
    part_2(&mfcsam, &sues);
}
