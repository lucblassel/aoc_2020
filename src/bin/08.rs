use regex::Regex;
use std::collections::HashSet;

fn main() {
    println!("Day 08:");
    let input = include_str!("../../inputs/08.txt");
    let regex = Regex::new(r"(nop|acc|jmp) (.+)").unwrap();

    let instructions: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            regex
                .captures_iter(line)
                .map(|cap| {
                    (
                        cap.get(1).unwrap().as_str(),
                        cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    )
                })
                .next()
                .unwrap()
        })
        .collect();

    let (acc1, _) = execute_prog(&instructions);
    println!("\t1) {acc1}");

    for (cursor, instruction) in instructions.iter().enumerate() {
        match instruction {
            ("acc", _) => continue,
            _ => {
                let (acc, halted) = execute_with_switch(&instructions, cursor);
                if halted {
                    println!("\t2) {acc}");
                    break;
                }
            }
        }
    }
}

fn execute_prog(instructions: &Vec<(&str, i32)>) -> (i32, bool) {
    let mut cursor: i32 = 0;
    let mut acc = 0;
    let mut executed: HashSet<i32> = HashSet::new();

    while (cursor as usize) < instructions.len() {
        if executed.contains(&cursor) {
            return (acc, false);
        }
        executed.insert(cursor);

        match instructions[cursor as usize] {
            ("nop", _) => cursor += 1,
            ("jmp", num) => cursor += num,
            ("acc", num) => {
                cursor += 1;
                acc += num;
            }
            _ => unreachable!(),
        }
    }

    (acc, true)
}

fn execute_with_switch(instructions: &Vec<(&str, i32)>, switch_index: usize) -> (i32, bool) {
    let mut cursor: i32 = 0;
    let mut acc = 0;
    let mut executed: HashSet<i32> = HashSet::new();

    while (cursor as usize) < instructions.len() {
        if executed.contains(&cursor) {
            return (acc, false);
        }
        executed.insert(cursor);

        match instructions[cursor as usize] {
            ("nop", num) => {
                if cursor as usize == switch_index {
                    cursor += num;
                } else {
                    cursor += 1;
                }
            }
            ("jmp", num) => {
                if cursor as usize == switch_index {
                    cursor += 1;
                } else {
                    cursor += num;
                }
            }
            ("acc", num) => {
                cursor += 1;
                acc += num;
            }
            _ => unreachable!(),
        }
    }

    (acc, true)
}
