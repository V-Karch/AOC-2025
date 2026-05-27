fn is_valid_id(id: &str) -> bool {
    let length = id.len();
    let middle = length / 2;

    id[..middle] != id[middle..]
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

            if !is_valid_id(&id_str) {
                invalid_ids.push(id_num);
            }
        }
    }

    let sum: i64 = invalid_ids.iter().sum();

    println!("Part 1: {}", sum);
}
