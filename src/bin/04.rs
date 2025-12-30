advent_of_code::solution!(4);

struct Grid {
    data: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
}

impl Grid {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = data.len() as i32;
        let cols = if rows > 0 { data[0].len() as i32 } else { 0 };
        Grid { data, rows, cols }
    }

    fn is_roll(&self, r: i32, c: i32) -> bool {
        if r < 0 || r >= self.rows || c < 0 || c >= self.cols {
            return false;
        }
        self.data[r as usize][c as usize] == '@'
    }

    fn remove_roll(&mut self, r: i32, c: i32) {
        self.data[r as usize][c as usize] = '.';
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let mut accessible_count = 0;

    for r in 0..grid.rows {
        for c in 0..grid.cols {
            // Only check if the current cell is a roll
            if !grid.is_roll(r, c) {
                continue;
            }

            let mut neighbor_rolls = 0;
            // Check all 8 neighbors
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    } // Skip the cell itself

                    if grid.is_roll(r + dr, c + dc) {
                        neighbor_rolls += 1;
                    }
                }
            }

            if neighbor_rolls < 4 {
                accessible_count += 1;
            }
        }
    }

    Some(accessible_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::new(input);
    let mut removed_rolls = 0;

    loop {
        let mut to_remove = Vec::new();

        for r in 0..grid.rows {
            for c in 0..grid.cols {
                // Only check if the current cell is a roll
                if !grid.is_roll(r, c) {
                    continue;
                }

                let mut neighbor_rolls = 0;
                // Check all 8 neighbors
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        } // Skip the cell itself

                        if grid.is_roll(r + dr, c + dc) {
                            neighbor_rolls += 1;
                        }
                    }
                }

                if neighbor_rolls < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove {
            grid.remove_roll(r, c);
            removed_rolls += 1;
        }
    }

    Some(removed_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
