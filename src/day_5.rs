fn is_in_range(id: u64, parts: &(u64, u64)) -> bool {
    return id >= parts.0 && id <= parts.1;
}

pub fn part_one(lines: &Vec<String>) {
    let split_index = lines
        .iter()
        .position(|s| s.is_empty())
        .expect("Failed to find breakpoint");

    let fresh_id_ranges: Vec<(u64, u64)> = lines[..split_index]
        .iter()
        .map(|s| {
            let mut parts = s.split('-');

            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();

            return (start, end);
        })
        .collect();

    let available_ids: Vec<u64> = lines[(split_index + 1)..]
        .iter()
        .map(|s| {
            return s
                .parse::<u64>()
                .expect("Failed to parse u64 during available ids collection");
        })
        .collect();

    let mut total_fresh = 0;

    for id in available_ids {
        let mut is_fresh = false;

        for id_range in &fresh_id_ranges {
            let check_fresh = is_in_range(id, id_range);
            if check_fresh {
                is_fresh = check_fresh;
                break; // Early escape
            }
        }

        if is_fresh {
            total_fresh += 1;
        }
    }

    println!("Part 1: {}", total_fresh);
}

pub fn part_two(lines: &Vec<String>) {
    let split_index = lines
        .iter()
        .position(|s| s.is_empty())
        .expect("Failed to find breakpoint");

    let mut fresh_id_ranges: Vec<(u64, u64)> = lines[..split_index]
        .iter()
        .map(|s| {
            let mut parts = s.split('-');

            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();

            return (start, end);
        })
        .collect();

    fresh_id_ranges.sort_unstable_by_key(|&(start, _)| start);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in fresh_id_ranges {
        match merged.last_mut() {
            Some((last_start, last_end)) if start <= *last_end + 1 => {
                *last_end = (*last_end).max(end);
            }
            _ => {
                merged.push((start, end));
            }
        }
    }

    let mut total: u64 = 0;

    for pair in &merged {
        total += (pair.1 + 1) - pair.0;
    }

    println!("Part 2: {}", total);
}
