use std::{collections::HashMap, ops::Add, u16};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Location<'a> {
    name: &'a str,
}

#[derive(Debug)]
struct Distance<'a> {
    locations: [&'a str; 2],
    distance: u16,
}

#[derive(Debug)]
struct Graph<'a> {
    locations: HashMap<&'a str, Location<'a>>,
    distances: Vec<Distance<'a>>,
}

#[derive(Clone, Copy)]
enum RouteType {
    Shortest,
    Longest,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph {
            locations: HashMap::new(),
            distances: Vec::new(),
        }
    }

    fn from_str(input: &'a str) -> Result<Self, String> {
        let mut graph = Graph::new();
        for line in input.lines() {
            graph.add_from_str(line)?;
        }
        Ok(graph)
    }

    fn add_location(&mut self, name: &'a str) {
        self.locations.insert(name, Location { name });
    }

    fn add_distance(&mut self, from: &'a str, to: &'a str, distance: u16) {
        if !self.locations.contains_key(from) {
            self.add_location(from);
        }
        if !self.locations.contains_key(to) {
            self.add_location(to);
        }

        self.distances.push(Distance {
            locations: [from, to],
            distance,
        });
    }

    fn add_from_str(&mut self, input: &'a str) -> Result<(), String> {
        let (from, to, distance) = input
            .split_once(" = ")
            .and_then(|(locations, distance)| {
                locations
                    .split_once(" to ")
                    .map(|(to, from)| (to, from, distance))
            })
            .ok_or_else(|| format!("Error parsing line: {input}"))?;

        let distance = distance
            .parse::<u16>()
            .map_err(|_| format!("Error parsing distance: {distance}"))?;

        self.add_distance(from, to, distance);

        Ok(())
    }

    fn get_distance(&self, from: &'a str, to: &'a str) -> u16 {
        self.distances
            .iter()
            .find(|distance| distance.locations.contains(&from) && distance.locations.contains(&to))
            .map(|distance| distance.distance)
            .unwrap_or_else(|| panic!("Distance not in list!"))
    }

    fn find_route(
        &self,
        route_type: RouteType,
        from: Option<&'a str>,
        unvisited_locations: Vec<&'a str>,
    ) -> u16 {
        if unvisited_locations.len() == 0 {
            return 0;
        }

        let mut best_distance = match route_type {
            RouteType::Shortest => u16::MAX,
            RouteType::Longest => u16::MIN,
        };
        for to in unvisited_locations.iter() {
            let remaining_locations = unvisited_locations
                .clone()
                .into_iter()
                .filter(|loc| loc != to)
                .collect();
            let current_distance = from
                .map(|from| self.get_distance(from, to))
                .unwrap_or_default()
                .add(self.find_route(route_type, Some(to), remaining_locations));
            best_distance = match route_type {
                RouteType::Shortest => current_distance.min(best_distance),
                RouteType::Longest => current_distance.max(best_distance),
            };
        }

        best_distance
    }
}

fn part_1(graph: &Graph) -> u16 {
    graph.find_route(
        RouteType::Shortest,
        None,
        graph.locations.keys().cloned().collect(),
    )
}

fn part_2(graph: &Graph) -> u16 {
    graph.find_route(
        RouteType::Longest,
        None,
        graph.locations.keys().cloned().collect(),
    )
}

fn main() -> Result<(), String> {
    let input = include_str!("../../input.txt");
    let graph = Graph::from_str(input)?;
    println!("Part 1: {}", part_1(&graph));
    println!("Part 2: {}", part_2(&graph));
    Ok(())
}
