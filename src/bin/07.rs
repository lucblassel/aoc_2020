use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Day 07:");
    let input = include_str!("../../inputs/07.txt");

    let parent_re = Regex::new(r"^(.+) bags contain").unwrap();
    let child_re = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();

    let mut containers: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();
    let mut containees: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();

    for line in input.lines() {
        let parent = parent_re.captures(line).unwrap().get(1).unwrap().as_str();
        for cap in child_re.captures_iter(line) {
            let count = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color = cap.get(2).unwrap().as_str();

            containers
                .entry(parent)
                .or_insert(vec![])
                .push((color, count));
            containees
                .entry(color)
                .or_insert(vec![])
                .push((parent, count));
        }
    }

    let mut to_visit: Vec<&str> = vec!["shiny gold"];
    let mut visited: HashSet<&str> = HashSet::new();
    while !to_visit.is_empty() {
        match to_visit.pop() {
            None => break,
            Some(name) => {
                containees
                    .entry(name)
                    .or_default()
                    .iter()
                    .map(|(name, _)| *name)
                    .map(|name| {
                        visited.insert(name);
                        to_visit.push(name);
                    })
                    .count();
            }
        }
    }

    println!(
        "\t1) {}",
        visited.len()
    );

    println!(
        "\t2) {}",
        count_contained("shiny gold", &containers) - 1
    )
}

fn count_contained(name: &str, containers: &HashMap<&str, Vec<(&str, i32)>>) -> i32 {
    let mut count = 1;

    if let Some(contained) = containers.get(name) {
        for (c_name, c_count) in contained {
            count += (*c_count) * count_contained(c_name, containers)
        }
    } else {
        return 1;
    }

    count
}
