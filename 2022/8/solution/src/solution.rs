use std::collections::HashSet;

type Solution = usize;
pub fn solve(input: impl Iterator<Item = String>) -> Solution {
    solve_part_1(input)
}

fn solve_part_1(input: impl Iterator<Item = String>) -> Solution {
    let grid = build_grid(input);

    let mut visible = HashSet::new();

    for i in 0..grid.len() {
        get_visible_row(&grid, i, &mut visible);
        get_visible_col(&grid, i, &mut visible);
    }

    visible.len()
}

fn get_visible_col(grid: &Vec<Vec<u32>>, col: usize, visible: &mut HashSet<(usize, usize)>) {
    let mut max = None;
    for row in 0..grid.len() {
        let current = grid[row][col];
        match max {
            None => {
                visible.insert((row, col));
                max = Some(current);
            }
            Some(max_height) if max_height < current => {
                visible.insert((row, col));
                max = Some(current);
            }
            _ => {}
        }
    }

    let mut max = None;
    for row in (0..grid.len()).rev() {
        let current = grid[row][col];
        match max {
            None => {
                visible.insert((row, col));
                max = Some(current);
            }
            Some(max_height) if max_height < current => {
                visible.insert((row, col));
                max = Some(current);
            }
            _ => {}
        }
    }
}

fn get_visible_row(grid: &Vec<Vec<u32>>, row: usize, visible: &mut HashSet<(usize, usize)>) {
    let mut max = None;
    for col in 0..grid.len() {
        let current = grid[row][col];
        match max {
            None => {
                visible.insert((row, col));
                max = Some(current);
            }
            Some(max_height) if max_height < current => {
                visible.insert((row, col));
                max = Some(current);
            }
            _ => {}
        }
    }

    let mut max = None;
    for col in (0..grid.len()).rev() {
        let current = grid[row][col];
        match max {
            None => {
                visible.insert((row, col));
                max = Some(current);
            }
            Some(max_height) if max_height < current => {
                visible.insert((row, col));
                max = Some(current);
            }
            _ => {}
        }
    }
}

fn solve_part_2(input: impl Iterator<Item = String>) -> Solution {
    todo!()
}

fn build_grid(input: impl Iterator<Item = String>) -> Vec<Vec<u32>> {
    input
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).expect("failed to parse digit"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 5] = ["30373", "25512", "65332", "33549", "35390"];
    const PART_1_ANSWER: Solution = 21;
    const PART_2_ANSWER: Solution = 21;

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_part_1() {
        let solution = solve_part_1(iter_input());

        assert_eq!(solution, PART_1_ANSWER);
    }

    #[test]
    fn test_part_2() {
        let solution = solve_part_2(iter_input());

        assert_eq!(solution, PART_2_ANSWER);
    }
}
