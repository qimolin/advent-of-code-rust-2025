advent_of_code::solution!(7);

struct Grid {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Grid { data, rows, cols }
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        for (r, row) in self.data.iter().enumerate() {
            if let Some(c) = row.iter().position(|&ch| ch == 'S') {
                return Some((r, c));
            }
        }
        None
    }
}

struct SimulationResult {
    splits: u64,
    timelines: u128,
}

struct Simulator<'a> {
    grid: &'a Grid,
}

impl<'a> Simulator<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self { grid }
    }

    fn run(&self) -> Option<SimulationResult> {
        let (start_row, start_col) = self.grid.find_start()?;

        let mut splits = 0u64;
        let mut total_timelines = 0u128;

        // current_counts[c] stores the number of active timelines in column c
        let mut current_counts = vec![0u128; self.grid.cols];
        current_counts[start_col] = 1;

        for r in start_row..self.grid.rows {
            let mut next_counts = vec![0u128; self.grid.cols];

            for c in 0..self.grid.cols {
                let n = current_counts[c];
                if n == 0 { continue; }

                match self.grid.data[r][c] {
                    '.' | 'S' => {
                        self.move_beam(r, c, n, &mut next_counts, &mut total_timelines);
                    }
                    '^' => {
                        splits += 1;
                        // Split into left and right
                        self.move_beam(r, c.wrapping_sub(1), n, &mut next_counts, &mut total_timelines);
                        self.move_beam(r, c + 1, n, &mut next_counts, &mut total_timelines);
                    }
                    _ => {}
                }
            }
            current_counts = next_counts;
        }

        Some(SimulationResult { splits, timelines: total_timelines })
    }

    /// Helper to handle bounds checking and exit counting in one place
    fn move_beam(&self, r: usize, c: usize, n: u128, next_counts: &mut [u128], total_timelines: &mut u128) {
        if c < self.grid.cols {
            if r + 1 < self.grid.rows {
                next_counts[c] += n;
            } else {
                *total_timelines += n;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let sim = Simulator::new(&grid);
    sim.run().map(|res| res.splits)
}

pub fn part_two(input: &str) -> Option<u128> {
    let grid = Grid::new(input);
    let sim = Simulator::new(&grid);
    sim.run().map(|res| res.timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
