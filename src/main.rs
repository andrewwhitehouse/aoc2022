use std::fs;

mod day1;

fn solve_day1() {
    let input = fs::read_to_string("input/day1.txt").expect("failed to read day1 input");
    let batches = day1::parse(input);
    println!("Day 1 Part 1 {}", day1::max_sum(batches.clone()));
    println!("Day 1 Part 2 {}", day1::sum_of_top_three(batches));
}

pub fn main() {
    solve_day1();
}
