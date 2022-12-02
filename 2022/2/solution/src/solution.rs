pub fn solve<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    input.map(|r| solve_round(&r)).map(|r| score(&r)).sum()
}

fn score(round: &str) -> usize {
    match round {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => panic!("got unexpected round"),
    }
}

fn solve_round(round: &str) -> String {
    (match round {
        "A X" => "A Z",
        "A Y" => "A X",
        "A Z" => "A Y",
        "B X" => "B X",
        "B Y" => "B Y",
        "B Z" => "B Z",
        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
        _ => panic!("got unexpected round"),
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 3] = ["A Y", "B X", "C Z"];

    #[test]
    fn test_score_round() {
        let expected_scores = [8, 1, 6];

        let actual_scores = EXAMPLE_INPUT.map(score);

        assert_eq!(expected_scores, actual_scores);
    }

    #[test]
    fn test_solve_round() {
        let expected_solved_rounds = ["A X", "B X", "C X"];

        let actual_solved_rounds = EXAMPLE_INPUT.map(solve_round);

        assert_eq!(expected_solved_rounds, actual_solved_rounds);
    }

    #[test]
    fn test_example() {
        let lines = EXAMPLE_INPUT.into_iter().map(|s| s.into());

        let solution = solve(lines);

        assert_eq!(solution, 12);
    }
}
