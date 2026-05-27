fn is_valid_id_part1(id: &str) -> bool {
    let length = id.len();
    let middle = length / 2;

    id[..middle] != id[middle..]
}

fn is_valid_id_part2(id: &str) -> bool {
    let len = id.len();

    for pattern_len in 1..=len / 2 {
        // Pattern length must divide evenly
        if len % pattern_len != 0 {
            continue;
        }

        let pattern = &id[..pattern_len];
        let repetitions = len / pattern_len;

        // Entire string is repeated pattern
        if pattern.repeat(repetitions) == id {
            return false;
        }
    }

    true
}

pub fn part_one(lines: &Vec<String>) {
    let line = &lines[0]; // Should only be one line for this

    let mut invalid_ids: Vec<i64> = Vec::new();

    for number_range in line.split(',') {
        if number_range.is_empty() {
            continue;
        }

        let mut parts = number_range.split('-');

        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();

        for id_num in start..=end {
            let id_str = id_num.to_string();

            if !is_valid_id_part1(&id_str) {
                invalid_ids.push(id_num);
            }
        }
    }

    let sum: i64 = invalid_ids.iter().sum();

    println!("Part 1: {}", sum);
}

pub fn part_two(lines: &Vec<String>) {
    let line = &lines[0]; // Input is one long line

    let mut invalid_ids: Vec<i64> = Vec::new();

    for number_range in line.split(',') {
        if number_range.is_empty() {
            continue;
        }

        let mut parts = number_range.split('-');

        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();

        for id_num in start..=end {
            let id_str = id_num.to_string();

            if !is_valid_id_part2(&id_str) {
                invalid_ids.push(id_num);
            }
        }
    }

    let sum: i64 = invalid_ids.iter().sum();

    println!("Part 2: {}", sum);
}
