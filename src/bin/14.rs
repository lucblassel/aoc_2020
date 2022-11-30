use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Day 14:");
    let input = include_str!("../../inputs/14.txt");
    let regex = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    let (mut mask_0, mut mask_1): (u64, u64) = (0, 0);
    let mut masks_x: Vec<(u64, u64)> = vec![];
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            let (_, mask_s) = line.split_at(7);
            (mask_0, mask_1, _) = parse_masks(mask_s);
            continue;
        };
        let captures = regex.captures(line).unwrap();
        let address: u64 = captures["address"].parse().unwrap();
        let mut value: u64 = captures["value"].parse().unwrap();

        value |= mask_1;
        value &= !mask_0;

        memory.insert(address, value);
    }

    println!("\t1) {}", memory.values().into_iter().sum::<u64>());

    memory = HashMap::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            let (_, mask_s) = line.split_at(7);
            (_, mask_1, masks_x) = parse_masks(mask_s);
            continue;
        };

        let captures = regex.captures(line).unwrap();
        let mut address: u64 = captures["address"].parse().unwrap();
        let value: u64 = captures["value"].parse().unwrap();

        address |= mask_1;

        let addresses: Vec<u64> = masks_x
            .iter()
            .map(|(mask_0, mask_1)| {
                let mut address = address;
                address |= mask_1;
                address &= !mask_0;

                address
            })
            .collect();

        for address in addresses {
            memory.insert(address, value);
        }
    }

    println!("\t2) {}", memory.values().into_iter().sum::<u64>());
}

fn parse_masks(mask_input: &str) -> (u64, u64, Vec<(u64, u64)>) {
    let (mut mask_0, mut mask_1) = (0, 0);
    let mut masks_x: Vec<(u64, u64)> = vec![(0, 0)];

    for char in mask_input.chars() {
        mask_1 <<= 1;
        mask_0 <<= 1;
        masks_x = masks_x.iter().map(|(v1, v2)| (v1 << 1, v2 << 1)).collect();
        match char {
            '0' => mask_0 |= 1,
            '1' => mask_1 |= 1,
            'X' => {
                let mut new_masks: Vec<(u64, u64)> = vec![];
                for (mask_0, mask_1) in masks_x.iter() {
                    let new_mask_0 = mask_0;
                    let new_mask_1 = mask_1;

                    new_masks.push((new_mask_0 | 1, *new_mask_1));
                    new_masks.push((*new_mask_0, new_mask_1 | 1));
                }
                masks_x = new_masks;
            }
            _ => unreachable!("Unknown character {char} in mask definition!"),
        }
    }

    (mask_0, mask_1, masks_x)
}
