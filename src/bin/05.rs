advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ingredients_ranges, available_ingredients) = input.split_once("\n\n")?;

    // Parse and normalize ranges, then sort
    let mut ranges: Vec<(u64, u64)> = ingredients_ranges
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (a, b) = line.split_once('-')?;
            let start = a.parse::<u64>().ok()?;
            let end = b.parse::<u64>().ok()?;
            Some((start.min(end), start.max(end)))
        })
        .collect();

    // Sort by start so we can early-break on lookup
    ranges.sort_unstable_by_key(|&(start, _)| start);

    let mut fresh_ingredients_count = 0;

    'ingredients: for line in available_ingredients.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let ingredient_number: u64 = match line.parse() {
            Ok(v) => v,
            Err(_) => continue, // or return None if invalid should be fatal
        };

        // Scan sorted ranges; break early when current range starts past ingredient_number
        for &(min, max) in &ranges {
            if ingredient_number < min {
                continue 'ingredients;
            }
            if ingredient_number <= max {
                fresh_ingredients_count += 1;
                continue 'ingredients;
            }
        }
    }

    Some(fresh_ingredients_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Parse all non-empty lines into (start, end)
    let mut ranges: Vec<(u64, u64)> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (a, b) = line.split_once('-')?;
            let start = a.parse::<u64>().ok()?;
            let end = b.parse::<u64>().ok()?;
            Some((start.min(end), start.max(end)))
        })
        .collect();

    if ranges.is_empty() {
        return Some(0);
    }

    // Sort by start
    ranges.sort_unstable_by_key(|&(start, _)| start);

    // Merge overlapping / adjacent ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some((_last_start, last_end)) = merged.last_mut() {
            // Overlaps or touches? Merge.
            if start <= *last_end + 1 {
                if end > *last_end {
                    *last_end = end;
                }
            } else {
                // Disjoint, start new interval
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    // Sum lengths of merged ranges
    let total: u64 = merged
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
