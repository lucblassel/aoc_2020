use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    println!("Day 15");
    let input = include_str!("../../inputs/15.txt");
    let numbers: Vec<usize> = input.split(',').map(|num| num.parse().unwrap()).collect();

    let mut said_last = 0;
    let mut tracker: HashMap<usize, (usize, usize)> = HashMap::new();

    for (i, number) in numbers.iter().enumerate() {
        tracker.insert(*number, (i + 1, i + 1));
        said_last = *number;
    }

    for turn in (numbers.len() + 1)..=30000000 {
        let (second_to_last, last) = tracker.get(&said_last).unwrap();
        if second_to_last == last {
            said_last = 0; // Say 0
        } else {
            said_last = last - second_to_last;
        }

        let turns = match tracker.entry(said_last) {
            Entry::Vacant(entry) => entry.insert((turn, turn)),
            Entry::Occupied(entry) => entry.into_mut(),
        };

        turns.0 = turns.1;
        turns.1 = turn;

        if turn == 2020 {
            println!("\t1) {said_last}");
        }
    }

    println!("\t2) {said_last}");
}