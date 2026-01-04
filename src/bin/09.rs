advent_of_code::solution!(9);

struct Grid {
    red_tiles: Vec<RedTile>,
}

#[derive(Clone, Copy)]
struct RedTile {
    coordinate: (usize, usize),
}

#[derive(Clone, Copy)]
struct Rect {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

impl Rect {
    fn from_points(a: &RedTile, b: &RedTile) -> Self {
        let (ax, ay) = (a.coordinate.0 as i64, a.coordinate.1 as i64);
        let (bx, by) = (b.coordinate.0 as i64, b.coordinate.1 as i64);
        Rect {
            x1: ax.min(bx),
            x2: ax.max(bx),
            y1: ay.min(by),
            y2: ay.max(by),
        }
    }

    fn area(&self) -> u64 {
        ((self.x2 - self.x1 + 1) as u64) * ((self.y2 - self.y1 + 1) as u64)
    }

    fn overlaps(&self, other: &Rect) -> bool {
        self.x1 <= other.x2
            && other.x1 <= self.x2
            && self.y1 <= other.y2
            && other.y1 <= self.y2
    }

    fn inner(&self) -> Option<Rect> {
        let ix1 = self.x1 + 1;
        let ix2 = self.x2 - 1;
        let iy1 = self.y1 + 1;
        let iy2 = self.y2 - 1;

        if ix1 <= ix2 && iy1 <= iy2 {
            Some(Rect {
                x1: ix1,
                x2: ix2,
                y1: iy1,
                y2: iy2,
            })
        } else {
            None
        }
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let red_tiles = input
            .lines()
            .filter_map(|line| {
                let mut parts = line.split(',');
                let x = parts.next()?.trim().parse().ok()?;
                let y = parts.next()?.trim().parse().ok()?;
                Some(RedTile { coordinate: (x, y) })
            })
            .collect();

        Self { red_tiles }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let n = grid.red_tiles.len();
    if n < 2 {
        return None;
    }

    let mut max_area: u64 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let rect = Rect::from_points(&grid.red_tiles[i], &grid.red_tiles[j]);
            max_area = max_area.max(rect.area());
        }
    }

    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let n = grid.red_tiles.len();
    if n < 2 {
        return None;
    }

    let mut candidates: Vec<Rect> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            candidates.push(Rect::from_points(&grid.red_tiles[i], &grid.red_tiles[j]));
        }
    }
    candidates.sort_by(|a, b| b.area().cmp(&a.area()));

    let mut lines: Vec<Rect> = Vec::new();
    for i in 0..n {
        let j = (i + 1) % n;
        lines.push(Rect::from_points(&grid.red_tiles[i], &grid.red_tiles[j]));
    }

    for rect in candidates {
        match rect.inner() {
            None => return Some(rect.area()),
            Some(inner) => {
                if lines.iter().all(|seg| !seg.overlaps(&inner)) {
                    return Some(rect.area());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
