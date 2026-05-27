#[derive(Debug)]
enum Direction {
    Left,
    Right
}

pub fn part_one(lines: &Vec<String>) {
    let mut base = 50;
    let mut times_resting_at_zero = 0;

    for line in lines {
        let (raw_direction, number) = line.split_at(1);
        let raw_direction = raw_direction.chars().next().unwrap();

        let direction: Direction = if raw_direction == 'R' {Direction::Right} else {Direction::Left};
        let number: i32 = number.parse().unwrap();

        match direction {
            Direction::Left => {
                base = (base - number) % 100;
                if base < 0 {
                    base += 100;
                }
            },
            Direction::Right => {
                base = (base + number) % 100;
            },
        }

        if base == 0 {
            times_resting_at_zero += 1;
        }
    }

    println!("Part 1: {}", times_resting_at_zero);
}