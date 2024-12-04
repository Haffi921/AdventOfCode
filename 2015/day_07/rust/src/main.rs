use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Input<'a> {
    Wire(&'a str),
    Value(u16),
}

impl<'a> Input<'a> {
    fn from_str(s: &'a str) -> Input {
        s.parse::<u16>()
            .map(Input::Value)
            .unwrap_or_else(|_| Input::Wire(s))
    }

    fn evaluate(
        &self,
        wires: &HashMap<&str, Wire<'a>>,
        cache: &mut HashMap<&'a str, u16>,
    ) -> Option<u16> {
        match self {
            Input::Value(v) => Some(*v),
            Input::Wire(w) => wires.get(w).and_then(|g| g.evaluate(wires, cache)),
        }
    }
}

#[derive(Debug, Clone)]
enum Gate<'a> {
    Direct(Input<'a>),
    And(Input<'a>, Input<'a>),
    Or(Input<'a>, Input<'a>),
    Not(Input<'a>),
    Rshift(Input<'a>, u16),
    Lshift(Input<'a>, u16),
}

#[derive(Debug, Clone)]
struct Wire<'a> {
    gate: Gate<'a>,
    output: &'a str,
}

impl<'a> Wire<'a> {
    fn from_str(line: &'a str) -> Result<Self, String> {
        let (gate_str, output) = line
            .split_once(" -> ")
            .ok_or_else(|| format!("Invalid wire format: {}", line))?;

        let parts: Vec<&str> = gate_str.split_whitespace().collect();

        let gate = match parts.len() {
            1 => Gate::Direct(Input::from_str(parts[0])),
            2 if parts[0] == "NOT" => Gate::Not(Input::from_str(parts[1])),
            3 => match parts[1] {
                "AND" => Gate::And(Input::from_str(parts[0]), Input::from_str(parts[2])),
                "OR" => Gate::Or(Input::from_str(parts[0]), Input::from_str(parts[2])),
                "RSHIFT" => Gate::Rshift(
                    Input::from_str(parts[0]),
                    parts[2]
                        .parse::<u16>()
                        .map_err(|_| format!("Invalid shift value: {}", parts[2]))?,
                ),
                "LSHIFT" => Gate::Lshift(
                    Input::from_str(parts[0]),
                    parts[2]
                        .parse::<u16>()
                        .map_err(|_| format!("Invalid shift value: {}", parts[2]))?,
                ),
                _ => return Err(format!("Invalid operation: {}", parts[1])),
            },
            _ => return Err(format!("Invalid instruction format: {}", line)),
        };
        Ok(Wire { gate, output })
    }

    fn evaluate(
        &self,
        wires: &HashMap<&str, Wire<'a>>,
        cache: &mut HashMap<&'a str, u16>,
    ) -> Option<u16> {
        if let Some(value) = cache.get(self.output) {
            return Some(*value);
        }

        let value = match &self.gate {
            Gate::Direct(input) => input.evaluate(wires, cache),
            Gate::And(a, b) => {
                let v1 = a.evaluate(wires, cache)?;
                let v2 = b.evaluate(wires, cache)?;
                Some(v1 & v2)
            }
            Gate::Or(a, b) => {
                let v1 = a.evaluate(wires, cache)?;
                let v2 = b.evaluate(wires, cache)?;
                Some(v1 | v2)
            }
            Gate::Not(a) => a.evaluate(wires, cache).map(|v| !v),
            Gate::Rshift(a, b) => a.evaluate(wires, cache).map(|v| v >> b),
            Gate::Lshift(a, b) => a.evaluate(wires, cache).map(|v| v << b),
        };

        if let Some(value) = value {
            cache.insert(self.output, value);
        }

        value
    }
}

fn part_1(wires: &HashMap<&str, Wire>) -> u16 {
    let mut cache = HashMap::<&str, u16>::new();
    wires.get("a").unwrap().evaluate(wires, &mut cache).unwrap()
}

fn part_2(wires: &HashMap<&str, Wire>, a_value: u16) -> u16 {
    let mut cache = HashMap::<&str, u16>::new();
    cache.insert("b", a_value);
    wires.get("a").unwrap().evaluate(wires, &mut cache).unwrap()
}

fn main() {
    let wires = include_str!("../../input.txt")
        .lines()
        .filter_map(|line| Wire::from_str(line).ok())
        .map(|wire| (wire.output, wire))
        .collect::<HashMap<&str, Wire>>();
    let a_value = part_1(&wires);
    println!("{}", a_value);
    println!("{}", part_2(&wires, a_value));
}
