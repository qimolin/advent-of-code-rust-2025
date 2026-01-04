advent_of_code::solution!(12);

use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct BitBoard {
    data: Vec<u64>,
}

impl BitBoard {
    fn new(num_cells: usize) -> Self {
        let num_words = (num_cells + 63) / 64;
        BitBoard {
            data: vec![0; num_words],
        }
    }

    fn from_mask_vec(masks: &[u64]) -> Self {
        BitBoard {
            data: masks.to_vec(),
        }
    }

    fn or_inplace(&mut self, other: &BitBoard) {
        for (a, b) in self.data.iter_mut().zip(&other.data) {
            *a |= *b;
        }
    }

    fn and_is_zero(&self, other: &BitBoard) -> bool {
        self.data
            .iter()
            .zip(&other.data)
            .all(|(a, b)| (*a & *b) == 0)
    }

    fn count_ones(&self) -> u32 {
        self.data.iter().map(|w| w.count_ones()).sum()
    }
}

#[derive(Clone, Debug)]
struct Orientation {
    cells: Vec<(i32, i32)>, // normalized, sorted
}

#[derive(Clone, Debug)]
struct Problem {
    shapes: Vec<Vec<Orientation>>,
    shape_sizes: Vec<u8>,
    regions: Vec<(usize, usize, Vec<u8>)>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let problem = parse_problem(input);
    let mut ok_regions = 0u64;

    for (w, h, counts) in &problem.regions {
        if can_fill_region(*w, *h, &problem.shapes, &problem.shape_sizes, counts) {
            ok_regions += 1;
        }
    }

    Some(ok_regions)
}

pub fn part_two(_input: &str) -> Option<u64> {
    // No part two for Day 12
    None
}

fn parse_problem(input: &str) -> Problem {
    let lines: Vec<&str> = input.lines().collect();

    let mut region_start = None;
    for (i, &line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if is_region_line(trimmed) {
            region_start = Some(i);
            break;
        }
    }
    let region_start = region_start.expect("No region lines found in input");

    let (shape_lines, region_lines) = lines.split_at(region_start);

    let (shapes, shape_sizes) = parse_shapes(shape_lines);
    let regions = parse_regions(region_lines, shapes.len());

    Problem {
        shapes,
        shape_sizes,
        regions,
    }
}

fn is_region_line(line: &str) -> bool {
    if let Some(colon) = line.find(':') {
        let (left, _) = line.split_at(colon);
        if let Some((w_str, h_str)) = left.split_once('x') {
            if !w_str.is_empty()
                && !h_str.is_empty()
                && w_str.chars().all(|c| c.is_ascii_digit())
                && h_str.chars().all(|c| c.is_ascii_digit())
            {
                return true;
            }
        }
    }
    false
}

fn parse_shapes(lines: &[&str]) -> (Vec<Vec<Orientation>>, Vec<u8>) {
    let mut shape_blocks: Vec<Vec<String>> = Vec::new();
    let mut current_block: Vec<String> = Vec::new();

    for &line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !current_block.is_empty() {
                shape_blocks.push(current_block);
                current_block = Vec::new();
            }
            continue;
        }

        if trimmed.ends_with(':') {
            if !current_block.is_empty() {
                shape_blocks.push(current_block);
                current_block = Vec::new();
            }
            continue; // skip header line
        }

        // Row of '.' and '#'
        current_block.push(trimmed.to_string());
    }
    if !current_block.is_empty() {
        shape_blocks.push(current_block);
    }

    let mut all_shapes_orientations: Vec<Vec<Orientation>> = Vec::new();
    let mut shape_sizes: Vec<u8> = Vec::new();

    for block in shape_blocks {
        let (base_cells, size) = parse_single_shape(&block);
        let orientations = generate_orientations(&base_cells);
        all_shapes_orientations.push(orientations);
        shape_sizes.push(size as u8);
    }

    (all_shapes_orientations, shape_sizes)
}

fn parse_single_shape(rows: &[String]) -> (Vec<(i32, i32)>, usize) {
    let height = rows.len();
    assert!(height > 0);
    let width = rows[0].len();

    let mut cells: Vec<(i32, i32)> = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        assert_eq!(row.len(), width);
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                cells.push((x as i32, y as i32));
            }
        }
    }

    assert!(!cells.is_empty(), "Shape with no '#' cells");

    normalize_cells(&mut cells);
    let size = cells.len();
    (cells, size)
}

fn normalize_cells(cells: &mut Vec<(i32, i32)>) {
    let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();
    for (x, y) in cells.iter_mut() {
        *x -= min_x;
        *y -= min_y;
    }
}

fn generate_orientations(base: &[(i32, i32)]) -> Vec<Orientation> {
    let mut seen: HashSet<Vec<(i32, i32)>> = HashSet::new();
    let mut result: Vec<Orientation> = Vec::new();

    for &flip in &[false, true] {
        for rot in 0..4 {
            let mut transformed: Vec<(i32, i32)> = base
                .iter()
                .map(|&(x, y)| {
                    let (x, y) = if flip { (-x, y) } else { (x, y) };
                    // rotate around origin
                    let (x2, y2) = match rot {
                        0 => (x, y),
                        1 => (-y, x),
                        2 => (-x, -y),
                        3 => (y, -x),
                        _ => unreachable!(),
                    };
                    (x2, y2)
                })
                .collect();

            normalize_cells(&mut transformed);
            transformed.sort();
            if seen.insert(transformed.clone()) {
                result.push(Orientation { cells: transformed });
            }
        }
    }

    result
}

fn parse_regions(lines: &[&str], num_shapes: usize) -> Vec<(usize, usize, Vec<u8>)> {
    let mut regions = Vec::new();

    for &line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (dim_part, rest) = trimmed
            .split_once(':')
            .expect("Region line must contain ':'");

        let (w_str, h_str) = dim_part
            .split_once('x')
            .expect("Region dimensions must contain 'x'");
        let w: usize = w_str.parse().expect("Invalid width");
        let h: usize = h_str.parse().expect("Invalid height");

        let mut counts: Vec<u8> = rest
            .split_whitespace()
            .map(|s| s.parse::<u8>().expect("Invalid count"))
            .collect();

        // Ensure counts length matches the number of shapes (pad with zeros if needed)
        if counts.len() < num_shapes {
            counts.resize(num_shapes, 0);
        } else if counts.len() > num_shapes {
            counts.truncate(num_shapes);
        }

        regions.push((w, h, counts));
    }

    regions
}

fn can_fill_region(
    width: usize,
    height: usize,
    shapes: &[Vec<Orientation>],
    shape_sizes: &[u8],
    counts: &[u8],
) -> bool {
    let area = width * height;

    // Quick prune: if total required cells > board area, impossible.
    let total_cells_needed: u32 = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| c as u32 * shape_sizes[i] as u32)
        .sum();

    if total_cells_needed as usize > area {
        return false;
    }

    // Precompute all placements (bitmasks) of each shape on this board.
    let placements = compute_placements(width, height, shapes);

    let mut remaining = counts.to_vec();
    let board = BitBoard::new(area);
    let total_cells = area as u32;

    backtrack(
        width,
        total_cells,
        shape_sizes,
        &placements,
        &mut remaining,
        &board,
    )
}

fn compute_placements(
    width: usize,
    height: usize,
    shapes: &[Vec<Orientation>],
) -> Vec<Vec<BitBoard>> {
    let area = width * height;
    let num_words = (area + 63) / 64;

    let mut placements: Vec<Vec<BitBoard>> = Vec::new();

    for shape_orients in shapes {
        let mut shape_placements: Vec<BitBoard> = Vec::new();

        for orient in shape_orients {
            // Find bounding box extents of this orientation
            let mut max_x = 0i32;
            let mut max_y = 0i32;
            for &(x, y) in &orient.cells {
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
            }

            let max_x_u = max_x as usize;
            let max_y_u = max_y as usize;

            if max_x_u >= width || max_y_u >= height {
                // This orientation cannot fit anywhere on this board
                continue;
            }

            let ox_max = width - 1 - max_x_u;
            let oy_max = height - 1 - max_y_u;

            for oy in 0..=oy_max {
                for ox in 0..=ox_max {
                    let mut words = vec![0u64; num_words];
                    for &(cx, cy) in &orient.cells {
                        let x = ox + cx as usize;
                        let y = oy + cy as usize;
                        let idx = y * width + x;
                        let word = idx / 64;
                        let bit = idx % 64;
                        words[word] |= 1u64 << bit;
                    }
                    shape_placements.push(BitBoard::from_mask_vec(&words));
                }
            }
        }

        placements.push(shape_placements);
    }

    placements
}

fn backtrack(
    width: usize,
    total_cells: u32,
    shape_sizes: &[u8],
    placements: &[Vec<BitBoard>],
    remaining: &mut [u8],
    board: &BitBoard,
) -> bool {
    // If all counts are zero, we successfully placed everything.
    if remaining.iter().all(|&c| c == 0) {
        return true;
    }

    // Area-based pruning
    let occupied = board.count_ones();
    let cells_needed: u32 = remaining
        .iter()
        .enumerate()
        .map(|(i, &c)| c as u32 * shape_sizes[i] as u32)
        .sum();
    if occupied + cells_needed > total_cells {
        return false;
    }

    // Choose the next shape to place: heuristic = shape with remaining > 0
    // and the fewest available placements (static approx).
    let mut best_shape: Option<usize> = None;
    let mut best_options = usize::MAX;

    for (i, &cnt) in remaining.iter().enumerate() {
        if cnt == 0 {
            continue;
        }
        let opts = placements[i].len();
        if opts == 0 {
            // Need at least one copy of a shape that has no possible placement -> impossible.
            return false;
        }
        if opts < best_options {
            best_options = opts;
            best_shape = Some(i);
        }
    }

    let shape_idx = match best_shape {
        Some(i) => i,
        None => return true, // all 0, should have returned above
    };

    // Try placing this shape at all non-overlapping placements.
    for pmask in &placements[shape_idx] {
        if !board.and_is_zero(pmask) {
            continue; // overlaps with already placed shapes
        }

        remaining[shape_idx] -= 1;
        let mut new_board = board.clone();
        new_board.or_inplace(pmask);

        if backtrack(
            width,
            total_cells,
            shape_sizes,
            placements,
            remaining,
            &new_board,
        ) {
            return true;
        }

        remaining[shape_idx] += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
