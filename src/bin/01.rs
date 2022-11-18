use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/01.txt");

    let numbers = input.lines().map(|num| str::parse::<i32>(num).unwrap());

    let mut pb1 = numbers
        .clone()
        .tuple_combinations::<(i32, i32)>()
        .filter(|(num1, num2)| num1 + num2 == 2020);

    if let Some((num1, num2)) = pb1.next() {
        println!("{num1} * {num2} = {}", num1 * num2);
    }

    let mut pb2 = numbers
        .tuple_combinations::<(i32, i32, i32)>()
        .filter(|(num1, num2, num3)| num1 + num2 + num3 == 2020);

    if let Some((num1, num2, num3)) = pb2.next() {
        println!("{num1} * {num2} * {num3} = {}", num1 * num2 * num3);
    }
}
