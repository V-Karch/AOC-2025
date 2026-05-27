use std::fs;

mod day_1;

fn file_to_lines(path: &str) -> Vec<String> {
    let content: String = fs::read_to_string(path).expect("Failed to read file");

    return content.lines().map(String::from).collect();
}

fn main() {
    let day_1_lines: Vec<String> = file_to_lines("inputs/day_1/input.txt");
    day_1::part_one(&day_1_lines);
    day_1::part_two(&day_1_lines);
}
