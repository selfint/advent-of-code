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
fn solve_part_2(input: impl Iterator<Item = String>) -> Solution {
    todo!()
}

fn solve_part_1(input: impl Iterator<Item = String>) -> Solution {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Monkey {
    pub op: Op,
    pub test_divisor: u32,
    pub true_target: usize,
    pub false_target: usize,
}

fn parse_monkey(string: &str) -> (Monkey, Vec<u32>) {
    let binding = string.replace(", ", "#");
    let parts = binding.split_ascii_whitespace().collect::<Vec<_>>();

    match parts.as_slice() {
        ["Monkey", _monkey_id, "Starting", "items:", items, "Operation:", "new", "=", lhs, op, rhs, "Test:", "divisible", "by", test_divisor, "If", "true:", "throw", "to", "monkey", true_target, "If", "false:", "throw", "to", "monkey", false_target] => {
            (
                Monkey {
                    op: (*lhs, *op, *rhs).into(),
                    test_divisor: test_divisor.parse().unwrap(),
                    true_target: true_target.parse().unwrap(),
                    false_target: false_target.parse().unwrap(),
                },
                items.split('#').map(|i| i.parse().unwrap()).collect(),
            )
        }
        _ => panic!("failed to parse string"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add(i32),
    Mul(i32),
    Square,
}

impl From<(&str, &str, &str)> for Op {
    fn from((lhs, op, rhs): (&str, &str, &str)) -> Self {
        match (lhs, op, rhs) {
            ("old", "+", "old") => Op::Mul(2),
            ("old", "*", "old") => Op::Square,
            ("old", "+", rhs) => Op::Add(rhs.parse().unwrap()),
            ("old", "*", rhs) => Op::Mul(rhs.parse().unwrap()),
            (lhs, "+", "old") => Op::Add(lhs.parse().unwrap()),
            (lhs, "*", "old") => Op::Mul(lhs.parse().unwrap()),
            _ => panic!("failed to parse op"),
        }
    }
}

type State = Vec<Vec<u32>>;
fn parse_input(input: impl Iterator<Item = String>) -> (Vec<Monkey>, State) {
    let input = input.collect::<Vec<String>>().join("\n");

    let mut monkeys = vec![];
    let mut state = vec![];

    for (monkey, items) in input.split("\n\n").map(parse_monkey) {
        monkeys.push(monkey);
        state.push(items);
    }

    (monkeys, state)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 28] = [
        "Monkey 0:",
        "  Starting items: 79, 98",
        "  Operation: new = old * 19",
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
        "",
        "Monkey 1:",
        "  Starting items: 54, 65, 75, 74",
        "  Operation: new = old + 6",
        "  Test: divisible by 19",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 0",
        "",
        "Monkey 2:",
        "  Starting items: 79, 60, 97",
        "  Operation: new = old * old",
        "  Test: divisible by 13",
        "    If true: throw to monkey 1",
        "    If false: throw to monkey 3",
        "",
        "Monkey 3:",
        "  Starting items: 74",
        "  Operation: new = old + 3",
        "  Test: divisible by 17",
        "    If true: throw to monkey 0",
        "    If false: throw to monkey 1",
        "",
    ];

    const PART_1_ANSWER: Solution = 10605;
    const PART_2_ANSWER: Solution = 0;

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_parse_input() {
        let expected_monkeys = vec![
            Monkey {
                op: Op::Mul(19),
                test_divisor: 23,
                true_target: 2,
                false_target: 3,
            },
            Monkey {
                op: Op::Add(6),
                test_divisor: 19,
                true_target: 2,
                false_target: 0,
            },
            Monkey {
                op: Op::Square,
                test_divisor: 13,
                true_target: 1,
                false_target: 3,
            },
            Monkey {
                op: Op::Add(3),
                test_divisor: 17,
                true_target: 0,
                false_target: 1,
            },
        ];
        let expected_state = vec![
            vec![79, 98],
            vec![54, 65, 75, 74],
            vec![79, 60, 97],
            vec![74],
        ];

        let (actual_monkeys, actual_state) = parse_input(iter_input());

        assert_eq!(expected_monkeys, actual_monkeys);
        assert_eq!(expected_state, actual_state);
    }

    #[test]
    fn test_run_round() {}

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
