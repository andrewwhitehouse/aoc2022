use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day8;

fn solve_day1() {
    let input = fs::read_to_string("input/day1.txt").expect("failed to read day1 input");
    let batches = day1::parse(input);
    println!("Day 1 Part 1 {}", day1::max_sum(batches.clone()));
    println!("Day 1 Part 2 {}", day1::sum_of_top_three(batches));
}

fn solve_day2() {
    let input = fs::read_to_string("input/day2.txt").expect("failed to read day1 input");
    let choices = day2::parse(input.clone());
    println!("Day 2 Part 1 {}", day2::game_score(choices));
    println!(
        "Day 2 Part 2 {}",
        day2::game_score_part2(day2::parse_part2(input))
    );
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("failed to read input file")
}

fn solve_day3() {
    let input = read_file("input/day3.txt");
    println!("Day 3 Part 1 {}", day3::solve_part1(input.clone()));
    println!("Day 3 Part 2 {}", day3::solve_part2(input));
}

fn solve_day4() {
    let input = read_file("input/day4.txt");
    println!("Day 4 Part 1 {}", day4::solve_part1(input.clone()));
    println!("Day 4 Part 2 {}", day4::solve_part2(input.clone()));
}

fn solve_day5() {
    let input = read_file("input/day5.txt");
    println!("Day 5 Part 1 {}", day5::solve_part1(input.clone()));
    println!("Day 5 Part 2 {}", day5::solve_part2(input));
}

fn solve_day6() {
    let input = read_file("input/day6.txt");
    println!("Day 6 Part 1 {}", day6::solve_part1(input.clone()));
    println!("Day 6 Part 2 {}", day6::solve_part2(input));
}

fn solve_day8() {
    let input = read_file("input/day8.txt");
    println!("Day 8 Part 1 {}", day8::solve_part1(input.clone()));
    println!("Day 8 Part 2 {}", day8::solve_part2(input));
}

pub fn main() {
    solve_day1();
    solve_day2();
    solve_day3();
    solve_day4();
    solve_day5();
    solve_day6();
    solve_day8();
}
