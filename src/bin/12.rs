use regex::Regex;

fn main() {
    println!("Day 12:");
    let input = include_str!("../../inputs/12.txt");
    let regex = Regex::new(r"([A-Z])(\d+)").unwrap();

    let instructions: Vec<(&str, i64)> = input
        .lines()
        .map(|line| {
            let caps = regex.captures(line).unwrap();
            let op = caps.get(1).unwrap().as_str();
            let val = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            (op, val)
        })
        .collect();

    let (mut ship_x, mut ship_y) = (0, 0);
    let mut ship_angle: f64 = 0.;

    for (op, val) in instructions.iter() {
        match *op {
            "N" => ship_y += val,
            "S" => ship_y -= val,
            "E" => ship_x += val,
            "W" => ship_x -= val,
            "L" => ship_angle = (ship_angle + *val as f64) % 360.,
            "R" => ship_angle = (ship_angle - *val as f64) % 360.,
            "F" => {
                let radians = ship_angle.to_radians();
                ship_x += (radians.cos() as i64) * val;
                ship_y += (radians.sin() as i64) * val;
            }
            _ => unreachable!("Unknown instruction: {op}"),
        }
    }

    println!("\t1): {}", ship_x.abs() + ship_y.abs());

    (ship_x, ship_y) = (0, 0);
    let (mut wp_x, mut wp_y) = (10, 1);

    for (op, val) in instructions.iter() {
        match *op {
            "N" => wp_y += val,
            "S" => wp_y -= val,
            "E" => wp_x += val,
            "W" => wp_x -= val,
            "L" | "R" => {
                let angle = if *op == "R" { 360 - val } else { *val };
                (wp_x, wp_y) = match angle {
                    90 => (-wp_y, wp_x),
                    180 => (-wp_x, -wp_y),
                    270 => (wp_y, -wp_x),
                    _ => (wp_x, wp_y),
                };
            }
            "F" => {
                ship_x += val * wp_x;
                ship_y += val * wp_y;
            }
            _ => unreachable!("Unknown instruction: {op}"),
        }
    }

    println!("\t2): {}", ship_x.abs() + ship_y.abs());
}
