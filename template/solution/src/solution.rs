pub fn solve(input: impl Iterator<Item = String>) -> String {
    todo!()
}

fn solve_part_1(input: impl Iterator<Item = String>) -> String {
    todo!()
}

fn solve_part_2(input: impl Iterator<Item = String>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; _] = [

    ];
    const PART_1_ANSWER:  = ;
    const PART_2_ANSWER:  = ;

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
