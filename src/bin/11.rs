advent_of_code::solution!(11);

use std::collections::{HashMap, HashSet};

struct Graph<'a> {
    adjacency: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn from_input(input: &'a str) -> Self {
        let mut adjacency = HashMap::new();

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let (src, dests) = line.split_once(':').unwrap();
            let src = src.trim();
            let neighbors: Vec<&str> = dests.split_whitespace().collect();

            adjacency.insert(src, neighbors);
        }

        Graph { adjacency }
    }

    fn count_paths(&self, start: &'a str, target: &'a str) -> u64 {
        let mut memo = HashMap::new();
        let mut visiting = HashSet::new();
        self.dfs(start, target, &mut memo, &mut visiting)
    }

    fn dfs(
        &self,
        current: &'a str,
        target: &'a str,
        memo: &mut HashMap<&'a str, u64>,
        visiting: &mut HashSet<&'a str>,
    ) -> u64 {
        // Base case: reached target
        if current == target {
            return 1;
        }

        // Already computed
        if let Some(&count) = memo.get(current) {
            return count;
        }

        // Cycle detection
        if visiting.contains(current) {
            return 0;
        }

        visiting.insert(current);

        let mut total = 0;
        if let Some(neighbors) = self.adjacency.get(current) {
            for &neighbor in neighbors {
                total += self.dfs(neighbor, target, memo, visiting);
            }
        }

        visiting.remove(current);
        memo.insert(current, total);

        total
    }

    fn count_paths_with_mandatory(
        &self,
        start: &'a str,
        target: &'a str,
        must_visit: &[&'a str],
    ) -> u64 {
        #[derive(Hash, Eq, PartialEq, Clone, Copy)]
        struct State<'a> {
            node: &'a str,
            mask: u32, // Each bit represents one node in must_visit
        }

        fn dfs<'a>(
            graph: &Graph<'a>,
            state: State<'a>,
            target: &'a str,
            must_visit: &[&'a str],
            memo: &mut HashMap<State<'a>, u64>,
            visiting: &mut HashSet<State<'a>>,
        ) -> u64 {
            if state.node == target {
                // Check if all bits in the mask are set
                let goal_mask = (1 << must_visit.len()) - 1;
                return if state.mask == goal_mask { 1 } else { 0 };
            }

            if let Some(&cached) = memo.get(&state) {
                return cached;
            }

            if visiting.contains(&state) {
                return 0;
            }

            visiting.insert(state);

            let mut total = 0;
            if let Some(neighbors) = graph.adjacency.get(state.node) {
                for &neighbor in neighbors {
                    // Update mask if the neighbor is one of the mandatory nodes
                    let mut next_mask = state.mask;
                    for (i, &m) in must_visit.iter().enumerate() {
                        if neighbor == m {
                            next_mask |= 1 << i;
                        }
                    }

                    let next_state = State {
                        node: neighbor,
                        mask: next_mask,
                    };
                    total += dfs(graph, next_state, target, must_visit, memo, visiting);
                }
            }

            visiting.remove(&state);
            memo.insert(state, total);
            total
        }

        let mut memo = HashMap::new();
        let mut visiting = HashSet::new();

        // Initial mask check for the start node
        let mut initial_mask = 0;
        for (i, &m) in must_visit.iter().enumerate() {
            if start == m {
                initial_mask |= 1 << i;
            }
        }

        dfs(
            self,
            State {
                node: start,
                mask: initial_mask,
            },
            target,
            must_visit,
            &mut memo,
            &mut visiting,
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = Graph::from_input(input);
    Some(graph.count_paths("you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = Graph::from_input(input);
    Some(graph.count_paths_with_mandatory("svr", "out", &["dac", "fft"]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
