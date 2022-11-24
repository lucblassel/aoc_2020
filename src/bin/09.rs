fn main() {
    println!("Day 09:");
    let input = include_str!("../../inputs/09.txt");
    let mut to_check: Vec<u64> = vec![];
    let len_to_check = 25;

    let numbers: Vec<u64> = input
        .lines()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let mut wrong_num = 0;

    for num in numbers.iter() {
        if to_check.len() < len_to_check {
            to_check.push(*num);
            continue;
        }
        if !check_sum(num, &to_check[..]) {
            wrong_num = *num;
            println!("\t1) {num}");
            break;
        }
        to_check.remove(0);
        to_check.push(*num);
    }

    let mut range: Vec<u64> = vec![];

    'outer: for start in 0..numbers.len() {
        let mut candidates: Vec<u64> = vec![];
        let mut sum = 0;
        for num in numbers[start..numbers.len()].iter() {
            candidates.push(*num);
            sum += num;
            if candidates.len() >= 2 && sum == wrong_num {
                range = candidates.clone();
                break 'outer;
            }
            if sum > wrong_num {
                break;
            }
        }
    }

    println!(
        "\t2) {}",
        range.iter().min().unwrap() + range.iter().max().unwrap()
    );
}

fn check_sum(target: &u64, vec: &[u64]) -> bool {
    for val1 in vec.iter() {
        for val2 in vec.iter() {
            if val1 + val2 == *target {
                return true;
            }
        }
    }

    false
}
