use std::fs;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn file_to_lines(path: &str) -> Vec<String> {
    let content: String = fs::read_to_string(path).expect("Failed to read file");

    return content.lines().map(String::from).collect();
}

fn main() {
    let day_1_lines: Vec<String> = file_to_lines("inputs/day_1/input.txt");
    let day_2_lines: Vec<String> = file_to_lines("inputs/day_2/input.txt");
    let day_3_lines: Vec<String> = file_to_lines("inputs/day_3/input.txt");
    let day_4_lines: Vec<String> = file_to_lines("inputs/day_4/input.txt");
    let day_5_lines: Vec<String> = file_to_lines("inputs/day_5/input.txt");
    println!("Day 1:");
    day_1::part_one(&day_1_lines);
    day_1::part_two(&day_1_lines);
    println!("Day 2:");
    day_2::part_one(&day_2_lines);
    day_2::part_two(&day_2_lines);
    println!("Day 3:");
    day_3::part_one(&day_3_lines);
    day_3::part_two(&day_3_lines);
    println!("Day 4:");
    day_4::part_one(&day_4_lines);
    day_4::part_two(&day_4_lines);
    println!("Day 5:");
    day_5::part_one(&day_5_lines);
}
