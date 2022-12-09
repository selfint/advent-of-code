use std::collections::HashSet;

type Solution = usize;
pub fn solve(input: impl Iterator<Item = String>) -> Solution {
    solve_part_2(input)
}

type Knot = ((i32, i32), (i32, i32));
type State = Vec<(i32, i32)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

fn solve_part_1(input: impl Iterator<Item = String>) -> Solution {
    let mut state = ((0, 0), (0, 0));
    let mut tail_locations = HashSet::new();

    for line in input {
        for step in parse_step(&line) {
            state = apply_step(state, step);
            tail_locations.insert(state.1);
        }
    }

    tail_locations.len()
}

fn solve_part_2(input: impl Iterator<Item = String>) -> Solution {
    let total_knots = 10;
    let mut state = [(0, 0)].repeat(total_knots);
    let mut tail_locations = HashSet::new();

    for line in input {
        for step in parse_step(&line) {
            state[total_knots - 1] = update_head(step, state[total_knots - 1]);

            for i in (1..total_knots).rev() {
                let head = state[i];
                let tail = state[i - 1];
                let new_tail = update_tail((head, tail));
                state[i - 1] = new_tail;
            }

            tail_locations.insert(state[0]);
        }
    }

    tail_locations.len()
}

fn parse_step(step: &str) -> Vec<Step> {
    let parts: Vec<_> = step.split_ascii_whitespace().collect();

    let (step, amount) = match parts.as_slice() {
        ["U", amount] => (Step::Up, amount.parse().unwrap()),
        ["D", amount] => (Step::Down, amount.parse().unwrap()),
        ["L", amount] => (Step::Left, amount.parse().unwrap()),
        ["R", amount] => (Step::Right, amount.parse().unwrap()),
        _ => panic!("failed to parse step"),
    };

    [step].repeat(amount)
}

fn apply_step(state: Knot, step: Step) -> Knot {
    let (head, tail) = state;
    let new_head = update_head(step, head);
    let new_tail = update_tail((new_head, tail));

    (new_head, new_tail)
}

fn update_head(step: Step, (h_y, h_x): (i32, i32)) -> (i32, i32) {
    match step {
        Step::Up => (h_y - 1, h_x),
        Step::Down => (h_y + 1, h_x),
        Step::Left => (h_y, h_x - 1),
        Step::Right => (h_y, h_x + 1),
    }
}

fn update_tail(((h_y, h_x), (t_y, t_x)): Knot) -> (i32, i32) {
    match (h_y - t_y, h_x - t_x) {
        // direct
        (0, 2) => (t_y, t_x + 1),
        (0, -2) => (t_y, t_x - 1),
        (2, 0) => (t_y + 1, t_x),
        (-2, 0) => (t_y - 1, t_x),

        // diagonal
        (2, 1) => (t_y + 1, t_x + 1),
        (-2, 1) => (t_y - 1, t_x + 1),
        (2, -1) => (t_y + 1, t_x - 1),
        (-2, -1) => (t_y - 1, t_x - 1),
        (1, 2) => (t_y + 1, t_x + 1),
        (-1, 2) => (t_y - 1, t_x + 1),
        (1, -2) => (t_y + 1, t_x - 1),
        (-1, -2) => (t_y - 1, t_x - 1),
        (2, 2) => (t_y + 1, t_x + 1),
        (-2, 2) => (t_y - 1, t_x + 1),
        (2, -2) => (t_y + 1, t_x - 1),
        (-2, -2) => (t_y - 1, t_x - 1),

        // distance is ok
        (0, 0) | (1, 0) | (0, 1) | (1, 1) | (-1, 0) | (0, -1) | (-1, -1) | (1, -1) | (-1, 1) => {
            (t_y, t_x)
        }

        // illegal
        _ => panic!("illegal state: ({h_y}, {h_x}) ({t_y}, {t_x})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 8] = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
    const EXAMPLE_INPUT_2: [&str; 8] = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];
    const PART_1_ANSWER: Solution = 13;
    const PART_2_ANSWER: Solution = 36;

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    fn iter_input_2() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT_2.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_parse_step() {
        let expected = vec![
            Step::Right,
            Step::Right,
            Step::Right,
            Step::Right,
            Step::Up,
            Step::Up,
            Step::Up,
            Step::Up,
            Step::Left,
            Step::Left,
            Step::Left,
            Step::Down,
        ];

        let actual: Vec<_> = ["R 4", "U 4", "L 3", "D 1"]
            .iter()
            .flat_map(|&s| parse_step(s))
            .collect();

        assert!(expected.into_iter().zip(actual).all(|(e, a)| e == a));
    }

    #[test]
    fn test_apply_step_direct() {
        let expected = ((1, 3), (1, 2));
        let actual = apply_step(((1, 2), (1, 1)), Step::Right);

        assert_eq!(actual, expected, "1 failed");

        let expected = ((3, 1), (2, 1));
        let actual = apply_step(((2, 1), (1, 1)), Step::Down);

        assert_eq!(actual, expected, "2 failed");
    }

    #[test]
    fn test_apply_step_diagonal() {
        let expected = ((1, 2), (2, 2));
        let actual = apply_step(((2, 2), (3, 1)), Step::Up);

        assert_eq!(actual, expected, "1 failed");

        let expected = ((2, 3), (2, 2));
        let actual = apply_step(((2, 2), (3, 1)), Step::Right);

        assert_eq!(actual, expected, "2 failed");
    }

    #[test]
    fn test_part_1() {
        let solution = solve_part_1(iter_input());

        assert_eq!(solution, PART_1_ANSWER);
    }

    #[test]
    fn test_part_2() {
        let solution = solve_part_2(iter_input_2());

        assert_eq!(solution, PART_2_ANSWER);
    }
}
