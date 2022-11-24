fn main() {
    let input = include_str!("../../inputs/03.txt");

    let lines: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let height = lines.len();
    let width = lines[0].len();

    let angles: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees: Vec<i32> = vec![];

    for (x_step, y_step) in angles {
        let (mut cur_x, mut cur_y) = (0, 0);

        let mut count = 0;

        while cur_y < (height - 1) {
            cur_x += x_step;
            if cur_x >= width {
                cur_x -= width;
            }
            cur_y += y_step;

            if lines[cur_y][cur_x] == '#' {
                count += 1;
            }
        }
        trees.push(count)
    }

    let mult: i32 = trees.iter().product();
    println!("1) {}", trees[1]);
    println!("2) {mult}");
}
