advent_of_code::solution!(2);

fn num_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut d = 0;
    while n > 0 {
        d += 1;
        n /= 10;
    }
    d
}

fn is_invalid_id_part_one(id: u64) -> bool {
    if id == 0 {
        return false;
    }

    let num_digits = num_digits(id);

    if num_digits < 2 || num_digits % 2 != 0 {
        return false;
    }

    let half = num_digits / 2;
    let divisor = 10_u64.pow(half);

    let first_half = id / divisor;
    let second_half = id % divisor;

    first_half == second_half
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum_invalid: u64 = 0;

    for token in input.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        let (start_str, end_str) = token.split_once('-')?;
        let start: u64 = start_str.parse().ok()?;
        let end: u64 = end_str.parse().ok()?;

        for id in start..=end {
            if is_invalid_id_part_one(id) {
                sum_invalid += id;
            }
        }
    }

    Some(sum_invalid)
}

fn is_invalid_id_part_two(id: u64) -> bool {
    if id < 11 {
        // must be at least 2 digits and repeat at least twice
        return false;
    }

    let num_digits = num_digits(id);
    if num_digits < 2 {
        return false;
    }

    // Try all possible block digit lengths
    for block_digits in 1..=num_digits / 2 {
        if num_digits % block_digits != 0 {
            continue;
        }

        let pow = 10_u64.pow(block_digits);
        let repeats = num_digits / block_digits;

        let pattern = id % pow;
        let mut tmp = id / pow;

        let mut ok = true;
        for _ in 1..repeats {
            if tmp % pow != pattern {
                ok = false;
                break;
            }
            tmp /= pow;
        }

        if ok {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum_invalid: u64 = 0;

    for token in input.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        let (start_str, end_str) = token.split_once('-')?;
        let start: u64 = start_str.parse().ok()?;
        let end: u64 = end_str.parse().ok()?;

        for id in start..=end {
            if is_invalid_id_part_two(id) {
                sum_invalid += id;
            }
        }
    }

    Some(sum_invalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
