use std::collections::HashSet;

type Solution = u32;
pub fn solve(input: impl Iterator<Item = String>) -> Solution {
    solve_part_2(input)
}

fn solve_part_1(input: impl Iterator<Item = String>) -> Solution {
    let grid = build_grid(input);

    let mut visible = HashSet::new();

    for i in 0..grid.len() {
        get_visible_row(&grid, i, &mut visible);
        get_visible_col(&grid, i, &mut visible);
    }

    visible.len() as u32
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
    let grid = build_grid(input);

    let mut max = None;

    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let (up_total, down_total, left_total, right_total) = get_score(&grid, i, j);
            let score = up_total * down_total * left_total * right_total;

            max = match max {
                None => Some(score),
                Some(max) if max < score => Some(score),
                Some(max) => Some(max),
            };
        }
    }

    max.expect("failed to find max")
}

fn get_score(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> (u32, u32, u32, u32) {
    let current = grid[i][j];

    let mut left_total = 0;
    let mut right_total = 0;
    let mut up_total = 0;
    let mut down_total = 0;

    let mut left_stop = false;
    let mut right_stop = false;
    let mut up_stop = false;
    let mut down_stop = false;

    for o in 1..grid.len() {
        if j >= o && grid[i][j - o] < current && !left_stop {
            left_total += 1;
        } else {
            if j >= o && !left_stop {
                left_total += 1;
            }
            left_stop = true;
        }

        if j + o < grid.len() && grid[i][j + o] < current && !right_stop {
            right_total += 1;
        } else {
            if j + o < grid.len() && !right_stop {
                right_total += 1;
            }
            right_stop = true;
        }

        if i >= o && grid[i - o][j] < current && !up_stop {
            up_total += 1;
        } else {
            if i >= o && !up_stop {
                up_total += 1;
            }
            up_stop = true;
        }
        if i + o < grid.len() && grid[i + o][j] < current && !down_stop {
            down_total += 1;
        } else {
            if i + o < grid.len() && !down_stop {
                down_total += 1;
            }
            down_stop = true;
        }
    }

    (up_total, down_total, left_total, right_total)
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
    const PART_2_ANSWER: Solution = 8;

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_get_score() {
        let grid = build_grid(iter_input());

        let totals = get_score(&grid, 3, 2);
        assert_eq!(totals, (2, 1, 2, 2));
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
