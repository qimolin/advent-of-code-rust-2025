advent_of_code::solution!(3);

use std::cmp::max;
use std::collections::HashMap;

fn find_battery_joltage_part_one(battery: &str) -> u64 {
    let mut best_right = 0;
    let mut best_pair = 0;
    for ch in battery.chars().rev() {
        let digit = ch.to_digit(10).unwrap();

        if best_right != 0 {
            let candidate = 10 * digit + best_right;
            if candidate > best_pair {
                best_pair = candidate;
            }
        }

        if digit > best_right {
            best_right = digit;
        }
    }

    best_pair as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltage_sum = 0;
    for battery in input.split_whitespace() {
        if battery.is_empty() {
            continue;
        }

        joltage_sum += find_battery_joltage_part_one(battery);
    }

    Some(joltage_sum)
}

fn find_battery_joltage_part_two(battery: &str, digits: usize) -> u64 {
    let bytes = battery.as_bytes(); // assume ASCII digits
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();

    fn helper(
        bytes: &[u8],
        i: usize, // current position in the string
        digits: usize, // digits left to pick
        memo: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        // Base cases
        if digits == 0 {
            return 0;
        }

        if bytes.len() - i == digits {
            // Must take all remaining digits; parse them as a number
            let mut acc: u64 = 0;
            for &b in &bytes[i..] {
                acc = acc * 10 + (b - b'0') as u64;
            }
            return acc;
        }

        // Check memo
        if let Some(&res) = memo.get(&(i, digits)) {
            return res;
        }

        // Take current digit
        let first_digit = (bytes[i] - b'0') as u64;
        let power = 10_u64.pow((digits - 1) as u32);

        let a = first_digit * power
            + helper(bytes, i + 1, digits - 1, memo);

        // Skip current digit
        let b = helper(bytes, i + 1, digits, memo);

        let res = max(a, b);
        memo.insert((i, digits), res);
        res
    }

    helper(bytes, 0, digits, &mut memo)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltage_sum = 0;
    for battery in input.split_whitespace() {
        if battery.is_empty() {
            continue;
        }

        joltage_sum += find_battery_joltage_part_two(battery, 12);
    }

    Some(joltage_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
