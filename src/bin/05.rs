use std::ops::Range;

fn main() {
    let input = include_str!("../../inputs/05.txt");

    let mut seats: Vec<i32> = input.lines().map(get_seat).collect();

    seats.sort();

    let mut prev_seat = -1;

    for seat in seats {
        if prev_seat != -1 && seat - prev_seat > 1 {
            println!("My Seat: {}", seat - 1);
            break;
        }
        prev_seat = seat;
    }
}

fn get_half(range: Range<i32>, upper: bool) -> Range<i32> {
    let mid = range.start + (range.end - range.start) / 2;
    if upper {
        mid..range.end
    } else {
        range.start..mid
    }
}

fn get_seat(steps: &str) -> i32 {
    let mut rows = 0..128;
    let mut seats = 0..8;

    for c in steps.chars() {
        match c {
            'F' => rows = get_half(rows, false),
            'B' => rows = get_half(rows, true),
            'L' => seats = get_half(seats, false),
            'R' => seats = get_half(seats, true),
            _ => unreachable!(),
        }
    }

    rows.start * 8 + seats.start
}
