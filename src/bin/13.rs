fn main() {
    println!("Day 13:");
    let mut lines = include_str!("../../inputs/13.txt").lines();
    let time: u64 = lines.next().unwrap().parse().unwrap();
    let bus_s = lines.next().unwrap();
    let buses: Vec<&str> = bus_s.split(',').collect();

    let (wait, line): (u64, u64) = buses
        .iter()
        .filter(|x| x != &&"x")
        .map(|val| val.parse().unwrap())
        .map(|bus| (bus - (time % bus), bus))
        .min()
        .unwrap();

    println!("\t1) {}", wait * line);

    let constraints: Vec<(i64, i64)> = buses
        .iter()
        .enumerate()
        .filter(|(_, val)| val != &&"x")
        .map(|(i, val)| (i as i64, val.parse().unwrap()))
        .collect();

    let mut timestamp = constraints[0].1;
    let mut step = 1;

    for (offset, bus) in constraints.iter() {
        while (timestamp + offset) % bus != 0 {
            timestamp += step;
        }
        step *= bus;
    }

    println!("\t2) {timestamp}")
}
