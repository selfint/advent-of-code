type Solution = i32;

pub fn solve(input: impl Iterator<Item = String>) -> Solution {
    solve_part_1(input)
}

fn solve_part_1(input: impl Iterator<Item = String>) -> Solution {
    let x_values = calc_x_values(input);

    x_values
        .into_iter()
        .enumerate()
        .filter(|(index, _)| (index + 20) % 40 == 0)
        .map(|(index, x)| index as i32 * x)
        .sum()
}

fn solve_part_2(input: impl Iterator<Item = String>) -> Solution {
    todo!()
}

fn calc_x_values(input: impl Iterator<Item = String>) -> Vec<i32> {
    let mut x_values = vec![1];
    let mut x = 1;
    let mut change;

    for (i, line) in input.enumerate() {
        x_values.push(x);

        let parts: Vec<_> = line.split_ascii_whitespace().collect();

        match parts.as_slice() {
            ["noop"] => change = 0,
            ["addx", amount] => {
                x_values.push(x);
                change = amount.parse().unwrap()
            }
            _ => panic!("got unexpected command: '{line}'"),
        }

        x += change;
    }

    x_values.push(x);

    x_values
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 146] = [
        "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
        "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1",
        "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1", "addx 16",
        "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop", "addx -3", "addx 9",
        "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop", "noop", "noop", "noop",
        "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop", "addx 2", "addx 6", "noop",
        "noop", "noop", "noop", "noop", "addx 1", "noop", "noop", "addx 7", "addx 1", "noop",
        "addx -13", "addx 13", "addx 7", "noop", "addx 1", "addx -33", "noop", "noop", "noop",
        "addx 2", "noop", "noop", "noop", "addx 8", "noop", "addx -1", "addx 2", "addx 1", "noop",
        "addx 17", "addx -9", "addx 1", "addx 1", "addx -3", "addx 11", "noop", "noop", "addx 1",
        "noop", "addx 1", "noop", "noop", "addx -13", "addx -19", "addx 1", "addx 3", "addx 26",
        "addx -30", "addx 12", "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9",
        "addx 18", "addx 1", "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1",
        "addx 2", "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22",
        "addx -6", "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop",
        "addx 20", "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
    ];

    const PART_1_ANSWER: Solution = 13140;
    const PART_2_ANSWER: Solution = 13140;

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_calc_x_values() {
        let expected = vec![1, 1, 1, 4, -1];
        let actual = calc_x_values(["noop", "addx 3", "addx -5"].into_iter().map(|s| s.into()));

        assert_eq!(expected, actual, "1 failed");
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
