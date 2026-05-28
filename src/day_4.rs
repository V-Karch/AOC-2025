fn count_removable_rolls(lines: &Vec<String>) -> i32 {
    let mut total = 0;

    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, char) in line.char_indices() {
            if char == '@' {
                let spots_to_check = [
                    ((line_index as isize) - 1, (char_index as isize) - 1), // Upper Left
                    ((line_index as isize), (char_index as isize) - 1),     // Directly Left
                    ((line_index as isize) + 1, (char_index as isize) - 1), // Bottom Left
                    ((line_index as isize) - 1, (char_index as isize)),     // Above
                    ((line_index as isize) + 1, (char_index as isize)),     // Below
                    ((line_index as isize) - 1, (char_index as isize) + 1), // Upper Right
                    ((line_index as isize), (char_index as isize) + 1),     // Directly Right
                    ((line_index as isize) + 1, (char_index as isize) + 1), // Bottom Right
                ];

                let mut values_at_spots: Vec<char> = Vec::new();

                for spot in spots_to_check {
                    if spot.0 < 0
                        || spot.1 < 0
                        || spot.0 >= lines.len() as isize
                        || spot.1 >= lines[spot.0 as usize].len() as isize
                    {
                        continue;
                    }

                    let value_to_append = &lines[spot.0 as usize].chars().nth(spot.1 as usize);
                    match value_to_append {
                        Some(value) => values_at_spots.push(value.clone()),
                        None => {} // Do nothing on failure
                    }
                }

                if values_at_spots.iter().filter(|&&c| c == '@').count() < 4 {
                    total += 1;
                }
            }
        }
    }

    return total;
}

fn remove_removable_rolls(lines: &Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, char) in line.char_indices() {
            if char == '@' {
                let spots_to_check = [
                    ((line_index as isize) - 1, (char_index as isize) - 1),
                    ((line_index as isize), (char_index as isize) - 1),
                    ((line_index as isize) + 1, (char_index as isize) - 1),
                    ((line_index as isize) - 1, (char_index as isize)),
                    ((line_index as isize) + 1, (char_index as isize)),
                    ((line_index as isize) - 1, (char_index as isize) + 1),
                    ((line_index as isize), (char_index as isize) + 1),
                    ((line_index as isize) + 1, (char_index as isize) + 1),
                ];

                let mut adjacent_count = 0;

                for (y, x) in spots_to_check {
                    if y < 0
                        || x < 0
                        || y >= lines.len() as isize
                        || x >= lines[y as usize].len() as isize
                    {
                        continue;
                    }

                    if lines[y as usize].chars().nth(x as usize) == Some('@') {
                        adjacent_count += 1;
                    }
                }

                if adjacent_count < 4 {
                    new_lines[line_index][char_index] = '.';
                }
            }
        }
    }

    return new_lines
        .into_iter()
        .map(|line| line.into_iter().collect())
        .collect();
}

pub fn part_one(lines: &Vec<String>) {
    let total = count_removable_rolls(lines);
    println!("Part 1: {}", total);
}

pub fn part_two(lines: &Vec<String>) {
    let mut total = 0;
    let mut current_removable = count_removable_rolls(lines);
    let mut new_lines: Vec<String> = remove_removable_rolls(lines);
    total += current_removable;

    while current_removable > 0 {
        current_removable = count_removable_rolls(&new_lines);
        new_lines = remove_removable_rolls(&new_lines);
        total += current_removable;
    }

    println!("Part 2: {}", total);
}
