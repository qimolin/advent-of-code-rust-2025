advent_of_code::solution!(6);

struct Grid {
    numbers: Vec<Vec<i64>>,
    operators: Vec<char>,
}

struct Problem {
    numbers: Vec<i64>,
    op: char,
}

impl Problem {
    fn evaluate(&self) -> Option<i64> {
        if self.numbers.is_empty() {
            return None;
        }

        Some(match self.op {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => return None,
        })
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let numbers: Vec<Vec<i64>> = lines[..lines.len().saturating_sub(1)]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|token| token.parse().ok())
                    .collect()
            })
            .collect();

        let operators: Vec<char> = lines
            .last()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|token| token.chars().next())
                    .collect()
            })
            .unwrap_or_default();

        Self { numbers, operators }
    }

    fn evaluate_column(&self, col_idx: usize) -> Option<i64> {
        let nums: Vec<i64> = self.numbers
            .iter()
            .filter_map(|row| row.get(col_idx).copied())
            .collect();

        let op = self.operators.get(col_idx)?;

        Problem { numbers: nums, op: *op }.evaluate()
    }

    fn parse_cephalopod(input: &str) -> Vec<Problem> {
        let raw_lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = raw_lines.len();
        let width = raw_lines.iter().map(|l| l.len()).max().unwrap_or(0);

        // Pad to uniform width
        let mut grid = vec![vec![' '; width]; height];
        for (r, line) in raw_lines.iter().enumerate() {
            for (c, &ch) in line.iter().enumerate() {
                grid[r][c] = ch;
            }
        }

        let mut problems = Vec::new();
        let mut cur_numbers = Vec::new();
        let mut cur_op = None;

        // Walk columns right-to-left
        for x in (0..width).rev() {
            let col: Vec<char> = (0..height).map(|y| grid[y][x]).collect();

            if col.iter().all(|&ch| ch == ' ') {
                // Blank column - end current problem
                if let Some(op) = cur_op {
                    if !cur_numbers.is_empty() {
                        problems.push(Problem { numbers: cur_numbers, op });
                        cur_numbers = Vec::new();
                    }
                }
                cur_op = None;
            } else {
                // Bottom cell is operator
                let bottom = col[height - 1];
                if !bottom.is_ascii_digit() && bottom != ' ' {
                    cur_op = Some(bottom);
                }

                // Above bottom are digits forming a number
                let digits: String = col[..height - 1]
                    .iter()
                    .filter(|&&ch| ch.is_ascii_digit())
                    .collect();

                if let Ok(value) = digits.parse::<i64>() {
                    cur_numbers.push(value);
                }
            }
        }

        // Last problem
        if let Some(op) = cur_op {
            if !cur_numbers.is_empty() {
                problems.push(Problem { numbers: cur_numbers, op });
            }
        }

        problems.reverse();
        problems
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);

    let total: i64 = (0..grid.operators.len())
        .filter_map(|col| grid.evaluate_column(col))
        .sum();

    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total: i64 = Grid::parse_cephalopod(input)
        .iter()
        .filter_map(|p| p.evaluate())
        .sum();

    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
