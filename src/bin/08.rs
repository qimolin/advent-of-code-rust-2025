advent_of_code::solution!(8);

use std::{convert::Infallible, str::FromStr, usize};

#[derive(Debug)]
struct Point(usize, usize, usize);

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x = self.0.abs_diff(other.0) as f64;
        let y = self.1.abs_diff(other.1) as f64;
        let z = self.2.abs_diff(other.2) as f64;
        (x * x + y * y + z * z).sqrt()
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        Ok(Point(
            line.next().unwrap().parse().unwrap(),
            line.next().unwrap().parse().unwrap(),
            line.next().unwrap().parse().unwrap(),
        ))
    }
}

struct Node {
    parent: usize,
    size: usize,
    _rank: usize,
}

struct DisjointSet {
    nodes: Vec<Node>,
}

impl DisjointSet {
    fn init(count: usize) -> Self {
        Self {
            nodes: (0..count).map(|id| Node {
                parent: id,
                size: 1,
                _rank: 0,
            }).collect()
        }
    }

    fn find(&mut self, id: usize) -> usize {
        if self.nodes[id].parent != id {
            self.nodes[id].parent = self.find(self.nodes[id].parent);
            return self.nodes[id].parent;
        }
        id
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return false;
        }

        let (x, y) = if self.nodes[x].size < self.nodes[y].size {
            (y, x)
        } else {
            (x, y)
        };

        self.nodes[y].parent = x;
        self.nodes[x].size += self.nodes[y].size;
        true
    }

    fn get_sizes(&self) -> Vec<usize> {
        self.nodes.iter().map(|node| node.size).collect()
    }

    fn num_components(&mut self) -> usize {
        (0..self.nodes.len())
            .filter(|&i| self.find(i) == i)
            .count()
    }
}

fn solve_with_connections(input: &str, connections: usize) -> Option<u64> {
    let points: Vec<Point> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.parse().unwrap())
            .collect();

    let mut distances = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            distances.push((points[i].distance(&points[j]), i, j));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut ds = DisjointSet::init(points.len());
    for &(_dist, i, j) in distances.iter().take(connections) {
        ds.union(i, j);
    }
    let mut sizes = ds.get_sizes();
    sizes.sort_by(|a, b| b.cmp(a));

    Some(sizes.iter().take(3).product::<usize>() as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve_with_connections(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    let n = points.len();
    if n < 2 {
        return None;
    }

    let mut distances = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            distances.push((points[i].distance(&points[j]), i, j));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut ds = DisjointSet::init(n);
    let mut last_connection = None;

    // Keep connecting until we have only 1 component
    for &(_dist, i, j) in distances.iter() {
        if ds.union(i, j) {
            last_connection = Some((i, j));

            // Check if all are in one circuit
            if ds.num_components() == 1 {
                break;
            }
        }
    }

    // Multiply the x coordinates of the last two connected junction boxes
    if let Some((i, j)) = last_connection {
        Some((points[i].0 * points[j].0) as u64)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_with_connections(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
