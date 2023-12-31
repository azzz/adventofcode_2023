use std::fs::read_to_string;

mod game;
mod parser;

fn main() {
    solve_part1();
    solve_part2();
}

fn solve_part1() {
    let rule = game::Set(12, 13, 14);

    let sum: u32 = read_to_string("input.txt").
        unwrap().
        lines().
        map(|line| parser::parse_game(line).unwrap().1 ).
        filter(|g| g.is_valid(&rule).is_ok() ).
        map(|g| g.id()).sum();

    println!("PART 1: {}", sum)
}

fn solve_part2() {
    let sum: u32 = read_to_string("input.txt").
        unwrap().
        lines().
        map(|line| parser::parse_game(line).unwrap().1.power() ).
        sum();

    println!("PART 2: {}", sum)
}