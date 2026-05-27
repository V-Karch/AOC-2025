pub fn part_one(lines: &Vec<String>) {
    let mut base = 50;
    let mut times_resting_at_zero = 0;

    for line in lines {
        let (raw_direction, number) = line.split_at(1);
        let direction = raw_direction.chars().next().unwrap();

        let number: i32 = number.parse().unwrap();

        match direction {
            'L' => {
                base = (base - number) % 100;
                if base < 0 {
                    base += 100;
                }
            }
            'R' => {
                base = (base + number) % 100;
            }
            _ => {}
        }

        if base == 0 {
            times_resting_at_zero += 1;
        }
    }

    println!("Part 1: {}", times_resting_at_zero);
}

pub fn part_two(lines: &Vec<String>) {
    let mut base: i32 = 50;
    let mut boundary_crossings: i32 = 0;

    for line in lines {
        let (direction_char, distance_str) = line.split_at(1);
        let direction_char = direction_char.chars().next().unwrap();
        let distance: i32 = distance_str.parse().unwrap();

        match direction_char {
            'R' => {
                let distance_next = (100 - base).rem_euclid(100);

                if distance_next == 0 {
                    boundary_crossings += distance / 100;
                } else if distance_next <= distance {
                    boundary_crossings += 1 + (distance - distance_next) / 100;
                }

                base = (base + distance).rem_euclid(100);
            }

            'L' => {
                let mut distance_back = base.rem_euclid(100);

                if distance_back == 0 {
                    distance_back = 100;
                }

                if distance_back <= distance {
                    boundary_crossings += 1 + (distance - distance_back) / 100;
                }

                base = (base - distance).rem_euclid(100);
            }

            _ => {}
        }
    }

    println!("Part 2: {}", boundary_crossings);
}
