use std::io;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.expect("failed to get line from stdin"))
        .take_while(|l| !l.is_empty());

    let solution = solve_part_1(input);

    println!("solution = {solution}");
}

type Solution = u32;
fn solve_part_1(input: impl Iterator<Item = String>) -> Solution {
    todo!()
}

fn solve_part_2(input: impl Iterator<Item = String>) -> Solution {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 0] = [];
    const PART_1_ANSWER: Solution = 0;
    const PART_2_ANSWER: Solution = 0;

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
