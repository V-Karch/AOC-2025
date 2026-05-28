fn get_max_joltage_part_one(line: &String) -> u32 {
    let mut digit_1: u32 = 0;
    let mut digit_2: u32 = 0;

    for (index, number) in line.char_indices() {
        let number_int: u32 = number.to_digit(10).expect("Couldn't convert char to int");
        if number_int > digit_1 && index + 1 < line.len() {
            digit_1 = number_int;
            digit_2 = 0;
        } else if number_int > digit_2 {
            digit_2 = number_int;
        }
    }

    return digit_1 * 10 + digit_2;
}

pub fn part_one(lines: &Vec<String>) {
    let mut sum = 0;

    for line in lines {
        sum += get_max_joltage_part_one(line);
    }

    println!("Part 1: {}", sum);
}

fn get_max_joltage_part_two(line: &String) -> u64 {
    let digits: Vec<char> = line.chars().collect();
    let k = 12;

    let mut to_remove = digits.len() - k;
    let mut stack: Vec<char> = Vec::with_capacity(k);

    for &digit in &digits {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
            stack.pop();
            to_remove -= 1;
        }

        stack.push(digit);
    }

    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    let result: String = stack.into_iter().take(k).collect();

    return result.parse::<u64>().expect("Failed to parse result");
}

pub fn part_two(lines: &Vec<String>) {
    let mut sum: u64 = 0;

    for line in lines {
        sum += get_max_joltage_part_two(line);
    }

    println!("Part 2: {}", sum);
}
