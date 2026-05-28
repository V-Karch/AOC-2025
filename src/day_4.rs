pub fn part_one(lines: &Vec<String>) {
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

    println!("Part 1: {}", total);
}
