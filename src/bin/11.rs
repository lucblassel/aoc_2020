#[derive(Clone, PartialEq)]
enum Seat {
    Free,
    Occupied,
}

fn show_board(board: &Vec<Vec<Option<Seat>>>) -> String {
    let mut repr = String::from("");
    for row in board {
        for val in row {
            match val {
                Some(Seat::Free) => repr.push('L'),
                Some(Seat::Occupied) => repr.push('#'),
                None => repr.push('.'),
            }
        }
        repr.push('\n');
    }
    repr
}

fn main() {
    println!("Day 11:");
    let input = include_str!("../../inputs/11.txt");
    let mut board: Vec<Vec<Option<Seat>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    'L' => Some(Seat::Free),
                    '#' => Some(Seat::Occupied),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut board2 = board.clone();

    let mut first_iter = true;
    let mut changes = 0;

    while changes != 0 || first_iter {
        let (new_board, changes_) = get_new_board(&board, 4, true);
        changes = changes_;
        board = new_board;
        first_iter = false;
        // println!("{}", show_board(&board))
    }

    let mut count1: u32 = 0;
    for row in board {
        for val in row.iter().flatten() {
            if *val == Seat::Occupied {
                count1 += 1
            }
        }
    }
    println!("\t1): {count1}");

    let mut first_iter = true;
    let mut changes = 0;

    while changes != 0 || first_iter {
        let (new_board, changes_) = get_new_board(&board2, 5, false);
        changes = changes_;
        board2 = new_board;
        first_iter = false;
        // println!("{}", show_board(&board2))
    }

    let mut count2: u32 = 0;
    for row in board2 {
        for val in row.iter().flatten() {
            if *val == Seat::Occupied {
                count2 += 1
            }
        }
    }
    println!("\t2): {count2}");
}

fn get_neighbour_indices(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut indices: Vec<(usize, usize)> = vec![];

    let x_range = if x > 0 { x - 1..=x + 1 } else { x..=x + 1 };

    for x_ in x_range {
        let y_range = if y > 0 { y - 1..=y + 1 } else { y..=y + 1 };

        for y_ in y_range {
            if x_ == x && y_ == y {
                continue;
            }
            if x_ < max_x && y_ < max_y {
                indices.push((x_, y_));
            }
        }
    }

    indices
}

fn get_visible_indices(x: i32, y: i32, board: &Vec<Vec<Option<Seat>>>) -> Vec<(usize, usize)> {
    let mut indices: Vec<(usize, usize)> = vec![];

    let x_dirs = [-1, 0, 1];
    let y_dirs = [-1, 0, 1];

    for x_dir in x_dirs {
        for y_dir in y_dirs {
            if x_dir == 0 && y_dir == 0 {
                continue;
            }

            let mut x_look = x;
            let mut y_look = y;
            loop {
                x_look += x_dir;
                y_look += y_dir;
                if x_look < 0
                    || x_look >= board.len() as i32
                    || y_look < 0
                    || y_look >= board[0].len() as i32
                {
                    break;
                }
                match board[x_look as usize][y_look as usize] {
                    None => continue,
                    Some(_) => {
                        indices.push((x_look as usize, y_look as usize));
                        break;
                    }
                }
            }
        }
    }

    indices
}

fn get_new_board(
    board: &Vec<Vec<Option<Seat>>>,
    max_neighbours: i32,
    adjacent: bool,
) -> (Vec<Vec<Option<Seat>>>, u32) {
    let mut new_board = board.clone();
    let mut changes: u32 = 0;

    for (x, row) in board.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            let mut neighbours = 0;
            let indices = if adjacent {
                get_neighbour_indices(x, y, board.len(), board[0].len())
            } else {
                get_visible_indices(x as i32, y as i32, board)
            };
            for (x_, y_) in indices {
                match board[x_][y_] {
                    Some(Seat::Occupied) => neighbours += 1,
                    Some(Seat::Free) | None => (),
                }
            }
            match val {
                Some(Seat::Free) if neighbours == 0 => {
                    changes += 1;
                    new_board[x][y] = Some(Seat::Occupied);
                }
                Some(Seat::Occupied) if neighbours >= max_neighbours => {
                    changes += 1;
                    new_board[x][y] = Some(Seat::Free);
                }
                seat => new_board[x][y] = seat.clone(),
            };
        }
    }

    (new_board, changes)
}
