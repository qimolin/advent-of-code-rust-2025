advent_of_code::solution!(1);

enum Rotation {
    L(i32),
    R(i32),
}

struct Safe {
    dial: u32, // 0..=99
    password: u64,
}

impl Safe {
    pub fn new() -> Self {
        Self {
            dial: 50,
            password: 0,
        }
    }

    pub fn turn_dial_part_one(&mut self, rotation: Rotation) {
        let delta: i32 = match rotation {
            Rotation::L(n) => -n,
            Rotation::R(n) => n,
        };

        let dial_size = 100_i32;
        let current = self.dial as i32;

        let new = (current + delta).rem_euclid(dial_size);
        self.dial = new as u32;

        if self.dial == 0 {
            self.password += 1;
        }
    }

    pub fn turn_dial_part_two(&mut self, rotation: Rotation) {
        let (step, steps) = match rotation {
            Rotation::L(n) => (-1, n),
            Rotation::R(n) => (1, n),
        };

        let dial_size = 100_i32;

        for _ in 0..steps {
            let current = self.dial as i32;
            let new = (current + step).rem_euclid(dial_size);
            self.dial = new as u32;

            if self.dial == 0 {
                self.password += 1;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut safe = Safe::new();

    for instruction in input.split_whitespace() {
        if instruction.is_empty() {
            continue;
        }

        let direction = instruction.chars().next()?;
        let number_str = &instruction[1..];
        let steps: i32 = number_str.parse().ok()?;

        let rotation = match direction {
            'L' => Rotation::L(steps),
            'R' => Rotation::R(steps),
            _ => continue,
        };

        safe.turn_dial_part_one(rotation);
    }

    Some(safe.password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut safe = Safe::new();

    for instruction in input.split_whitespace() {
        if instruction.is_empty() {
            continue;
        }

        let direction = instruction.chars().next()?;
        let number_str = &instruction[1..];
        let steps: i32 = number_str.parse().ok()?;

        let rotation = match direction {
            'L' => Rotation::L(steps),
            'R' => Rotation::R(steps),
            _ => continue,
        };

        safe.turn_dial_part_two(rotation);
    }

    Some(safe.password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
