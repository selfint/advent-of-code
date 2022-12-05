pub fn solve(input: impl Iterator<Item = String>) -> String {
    solve_part_1(input)
}

fn solve_part_1(mut input: impl Iterator<Item = String>) -> String {
    let stack_input = input.by_ref().take_while(|l| !l.is_empty()).collect();

    let mut stacks = parse_stacks(stack_input);
    let moves = parse_moves(input);

    for (from, to) in moves {
        let element = stacks[from].pop().expect("tried to pop from empty stack");
        stacks[to].push(element);
    }

    stacks
        .iter_mut()
        .map(|s| s.pop().expect("empty stack"))
        .collect()
}

fn solve_part_2(input: impl Iterator<Item = String>) -> String {
    todo!()
}

fn parse_stacks(stack_input: Vec<String>) -> Vec<Vec<char>> {
    let mut stacks = vec![];

    let mut stack_input = stack_input.iter().rev();

    let total_stacks = stack_input
        .next()
        .expect("failed to get stack indices line")
        .split_ascii_whitespace()
        .count();

    for _stack in 0..total_stacks {
        stacks.push(vec![]);
    }

    for line in stack_input {
        let chars = line.chars().collect::<Vec<_>>();
        for stack in 0..total_stacks {
            let char_index = 1 + 4 * stack;
            let stack_char = chars[char_index];

            if stack_char != ' ' {
                stacks[stack].push(stack_char);
            }
        }
    }

    stacks
}

fn parse_moves(moves_input: impl Iterator<Item = String>) -> Vec<(usize, usize)> {
    let mut moves = vec![];

    for line in moves_input {
        let line = line
            .replace("move", "")
            .replace("from", "")
            .replace("to", "");

        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        let [amount, from, to] = parts.as_slice() else {
            panic!("failed to parse line")
        };

        let amount = amount.parse().unwrap();
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();

        for _ in 0..amount {
            moves.push((from - 1, to - 1));
        }
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 9] = [
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];
    const PART_1_ANSWER: &str = "CMZ";
    const PART_2_ANSWER: &str = "MCD";

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_parse_stacks() {
        let expected_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        let stack_input = iter_input()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>();

        let actual_stacks = parse_stacks(stack_input);

        assert_eq!(expected_stacks, actual_stacks);
    }

    #[test]
    fn test_parse_moves() {
        let expected_moves = vec![(1, 0), (0, 2), (0, 2), (0, 2), (1, 0), (1, 0), (0, 1)];
        let moves_input = iter_input().skip_while(|l| !l.is_empty()).skip(1);

        let actual_moves = parse_moves(moves_input);

        assert_eq!(expected_moves, actual_moves);
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
