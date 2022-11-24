use std::collections::HashMap;

fn main() {
    println!("Day 10:");
    let input = include_str!("../../inputs/10.txt");

    let mut differences: HashMap<u32, u32> = HashMap::new();
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut joltages: Vec<u32> = input
        .lines()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    joltages.push(0); // Socket jolts
    joltages.sort();

    let device_jolts = joltages.last().unwrap() + 3;
    joltages.push(device_jolts);

    for i in 0..(joltages.len() - 1) {
        differences
            .entry(joltages[i + 1] - joltages[i])
            .and_modify(|count| *count += 1)
            .or_insert(1);
        let mut children: Vec<u32> = vec![];
        for val in joltages[i + 1..].iter() {
            if val - joltages[i] <= 3 {
                children.push(*val);
            }
        }
        graph.insert(joltages[i], children);
    }

    println!(
        "\t1) {}",
        differences.get(&1).unwrap() * differences.get(&3).unwrap()
    );

    let mut cache: HashMap<u32, u64> = HashMap::new();
    let paths = check_path(0, &graph, device_jolts, &mut cache);

    println!("\t2) {paths}");
}

fn check_path(start: u32, graph: &HashMap<u32, Vec<u32>>, end: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    let mut total = 0;

    if start == end {
        return 1;
    }

    let children = graph.get(&start).unwrap();
    for child in children {
        match cache.get(child) {
            Some(val) => total += val,
            None => {
                let paths = check_path(*child, graph, end, cache);
                cache.insert(*child, paths);
                total += paths;
            }
        }
    }

    total
}
