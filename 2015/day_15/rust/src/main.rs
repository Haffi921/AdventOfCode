// First, let's create a struct to represent an ingredient
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn from_str(s: &str) -> Self {
        let parts = s
            .split_whitespace()
            .map(|s| s.trim_end_matches(','))
            .collect::<Vec<_>>();
        Self {
            capacity: parts[2].parse().unwrap(),
            durability: parts[4].parse().unwrap(),
            flavor: parts[6].parse().unwrap(),
            texture: parts[8].parse().unwrap(),
            calories: parts[10].parse().unwrap(),
        }
    }
}

fn calculate_score(ingredients: &[Ingredient], amounts: &[i32]) -> i64 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;

    for (ingredient, &amount) in ingredients.iter().zip(amounts.iter()) {
        capacity += ingredient.capacity * amount;
        durability += ingredient.durability * amount;
        flavor += ingredient.flavor * amount;
        texture += ingredient.texture * amount;
    }

    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);

    capacity as i64 * durability as i64 * flavor as i64 * texture as i64
}

fn calculate_calories(ingredients: &[Ingredient], amounts: &[i32]) -> i32 {
    ingredients
        .iter()
        .zip(amounts.iter())
        .map(|(i, a)| i.calories * a)
        .sum()
}

fn part_1(ingredients: &[Ingredient]) -> String {
    let mut best_score = 0;

    for amount1 in 0..=100 {
        for amount2 in 0..=100 - amount1 {
            for amount3 in 0..=100 - amount1 - amount2 {
                let amount4 = 100 - amount1 - amount2 - amount3;

                let score = calculate_score(ingredients, &[amount1, amount2, amount3, amount4]);
                best_score = best_score.max(score);
            }
        }
    }

    format!("Best possible score: {}", best_score)
}

fn part_2(ingredients: &[Ingredient]) -> String {
    let mut best_score = 0;

    for amount1 in 0..=100 {
        for amount2 in 0..=100 - amount1 {
            for amount3 in 0..=100 - amount1 - amount2 {
                let amount4 = 100 - amount1 - amount2 - amount3;

                if calculate_calories(ingredients, &[amount1, amount2, amount3, amount4]) == 500 {
                    let score = calculate_score(ingredients, &[amount1, amount2, amount3, amount4]);
                    best_score = best_score.max(score);
                }
            }
        }
    }

    format!("Best possible score with 500 calories: {}", best_score)
}

fn main() {
    let input = include_str!("../../input.txt");
    let ingredients = input.lines().map(Ingredient::from_str).collect::<Vec<_>>();
    println!("Part 1: {}", part_1(&ingredients));
    println!("Part 2: {}", part_2(&ingredients));
}
