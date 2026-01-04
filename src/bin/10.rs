advent_of_code::solution!(10);

use good_lp::{variable, ProblemVariables, SolverModel, Solution, constraint, microlp};

struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        // Parse target pattern inside [ ... ]
        let start = line.find('[').expect("no '['");
        let end = line.find(']').expect("no ']'");
        let pattern = &line[start + 1..end];
        let target: Vec<bool> = pattern
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("invalid char in pattern"),
            })
            .collect();

        // Parse buttons: everything between ] and { as (...) groups
        let before_brace = line.split('{').next().unwrap();
        let mut buttons = Vec::new();
        let mut i = 0;
        while let Some(open_rel) = before_brace[i..].find('(') {
            let open = i + open_rel;
            let close = before_brace[open..]
                .find(')')
                .map(|c| open + c)
                .expect("unclosed '('");
            let inside = &before_brace[open + 1..close];
            let indices = inside
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<usize>().expect("bad index"))
                .collect();
            buttons.push(indices);
            i = close + 1;
        }

        // Parse joltages inside { ... }
        let brace_start = line.find('{').expect("no '{'");
        let brace_end = line.rfind('}').expect("no '}'");
        let joltage_str = &line[brace_start + 1..brace_end];
        let joltages: Vec<u32> = joltage_str
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<u32>().expect("bad joltage"))
            .collect();

        Self {
            target,
            buttons,
            joltages,
        }
    }
}

fn min_presses_for_machine_lights(machine: &Machine) -> Option<usize> {
    let n = machine.target.len();
    let m = machine.buttons.len();

    // Build augmented matrix [A | b] over GF(2)
    let mut mat = vec![vec![0u8; m + 1]; n];

    // Fill A
    for (j, btn) in machine.buttons.iter().enumerate() {
        for &i in btn {
            assert!(i < n);
            mat[i][j] ^= 1;
        }
    }
    // Fill b
    for i in 0..n {
        mat[i][m] = if machine.target[i] { 1 } else { 0 };
    }

    // Gaussian elimination to row echelon form
    let mut row = 0;
    for col in 0..m {
        let mut pivot = None;
        for r in row..n {
            if mat[r][col] == 1 {
                pivot = Some(r);
                break;
            }
        }
        let Some(pivot_row) = pivot else {
            continue;
        };

        if pivot_row != row {
            mat.swap(pivot_row, row);
        }

        for r in (row + 1)..n {
            if mat[r][col] == 1 {
                for c in col..=m {
                    mat[r][c] ^= mat[row][c];
                }
            }
        }

        row += 1;
        if row == n {
            break;
        }
    }

    // Check for inconsistency (0 ... 0 | 1)
    for r in 0..n {
        let all_zero_a = (0..m).all(|c| mat[r][c] == 0);
        if all_zero_a && mat[r][m] == 1 {
            return None;
        }
    }

    // Identify pivot columns and free columns
    let mut pivot_col_for_row = vec![None; n];
    let mut is_pivot_col = vec![false; m];

    for r in 0..n {
        if let Some(pc) = (0..m).find(|&c| mat[r][c] == 1) {
            pivot_col_for_row[r] = Some(pc);
            is_pivot_col[pc] = true;
        }
    }

    let free_cols: Vec<usize> = (0..m).filter(|&c| !is_pivot_col[c]).collect();
    let f = free_cols.len();

    // Back-substitution helper given choices for free variables
    fn back_sub_with_free(
        mat: &[Vec<u8>],
        m: usize,
        pivot_col_for_row: &[Option<usize>],
        free_cols: &[usize],
        free_values: &[u8],
    ) -> Vec<u8> {
        let n = mat.len();
        let mut x = vec![0u8; m];

        // set free variables
        for (fv_idx, &col) in free_cols.iter().enumerate() {
            x[col] = free_values[fv_idx];
        }

        // back-substitute for pivot variables
        for r in (0..n).rev() {
            let Some(pc) = pivot_col_for_row[r] else {
                continue;
            };

            let mut rhs = mat[r][m];
            for c in (pc + 1)..m {
                if mat[r][c] == 1 {
                    rhs ^= x[c];
                }
            }
            x[pc] = rhs;
        }

        x
    }

    // Particular solution x0 (all free vars = 0)
    let zero_free = vec![0u8; f];
    let x0 = back_sub_with_free(&mat, m, &pivot_col_for_row, &free_cols, &zero_free);

    // Nullspace basis vectors
    let mut basis: Vec<Vec<u8>> = Vec::with_capacity(f);
    for k in 0..f {
        let mut fv = vec![0u8; f];
        fv[k] = 1;
        let xk = back_sub_with_free(&mat, m, &pivot_col_for_row, &free_cols, &fv);
        let vk = xk
            .iter()
            .zip(x0.iter())
            .map(|(&a, &b)| a ^ b)
            .collect::<Vec<u8>>();
        basis.push(vk);
    }

    // Enumerate all combinations in the nullspace to find minimal Hamming weight
    if f == 0 {
        let presses = x0.iter().map(|&v| v as usize).sum();
        return Some(presses);
    }

    let mut best: Option<usize> = None;
    let total = 1usize << f;
    for mask in 0..total {
        let mut x = x0.clone();
        let mut tmp = mask;
        let mut k = 0;
        while tmp != 0 {
            if (tmp & 1) == 1 {
                for j in 0..m {
                    x[j] ^= basis[k][j];
                }
            }
            tmp >>= 1;
            k += 1;
        }

        let presses: usize = x.iter().map(|&v| v as usize).sum();
        if best.map_or(true, |b| presses < b) {
            best = Some(presses);
        }
    }

    best
}

fn min_presses_for_machine_jolts(machine: &Machine) -> Option<u32> {
    let k = machine.joltages.len();   // counters
    let m = machine.buttons.len();    // buttons

    if k == 0 {
        return Some(0);
    }

    // Build MILP:
    //   variables x_j >= 0 integer  (how many times to press button j)
    //   for each counter i: sum_{j: i in button_j} x_j == joltages[i]
    //   minimize sum_j x_j
    let mut vars = ProblemVariables::new();
    let x = vars.add_vector(variable().min(0).integer(), m);

    // Objective: minimize total button presses
    let mut objective = 0.0 * x[0]; // zero Expression
    for &v in &x {
        objective = objective + v;
    }

    let mut problem = vars.minimise(objective).using(microlp);

    // Constraints: one equation per counter
    for (i, &target) in machine.joltages.iter().enumerate() {
        let mut expr = 0.0 * x[0]; // 0 * var => zero expression
        for (j, btn) in machine.buttons.iter().enumerate() {
            if btn.contains(&i) {
                expr = expr + x[j];
            }
        }

        // Add constraint: expr == target
        // If no button touches counter i, expr is just 0, and solver will fail if target != 0
        problem = problem.with(constraint!(expr == target as f64));
    }

    let solution = match problem.solve() {
        Ok(sol) => sol,
        Err(_) => return None,
    };

    // Sum up the optimal number of presses
    let total: f64 = x.iter().map(|&v| solution.value(v)).sum();
    Some(total.round() as u32)
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let machine = Machine::from_line(line);
        let presses = min_presses_for_machine_lights(&machine)
            .expect("no solution for machine (lights)");
        total += presses as u64;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let machine = Machine::from_line(line);
        let presses = min_presses_for_machine_jolts(&machine)
            .expect("no solution for machine (jolts)");
        total += presses as u64;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
