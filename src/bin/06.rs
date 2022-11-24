fn main() {
    println!("Day 06:");
    let input = include_str!("../../inputs/06.txt");

    let mut counts: Vec<[i32; 26]> = vec![];
    let mut sizes: Vec<i32> = vec![];

    let mut count: [i32; 26] = [0; 26];
    let mut size = 0;
    for line in input.lines() {
        if line.is_empty() {
            counts.push(count);
            sizes.push(size);
            count = [0; 26];
            size = 0;
            continue;
        }
        for char in line.chars() {
            let idx = (char as usize) - 97;
            count[idx] += 1;
        }
        size += 1;
    }
    counts.push(count);
    sizes.push(size);

    let c1: usize = counts
        .iter()
        .map(|count| count.iter().filter(|&x| *x > 0).count())
        .sum();

    let c2: usize = counts.iter()
        .zip(sizes.iter())
        .map(|(count, size)| count.iter().filter(|&x| *x == *size).count())
        .sum();

    println!("\t1) {c1}");
    println!("\t2) {c2}");
}
