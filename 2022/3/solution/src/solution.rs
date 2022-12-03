pub fn solve(input: impl Iterator<Item = String>) -> usize {
    solve_part_2(input)
}

fn solve_part_1(input: impl Iterator<Item = String>) -> usize {
    input.map(find_double).map(prioritize_char).sum()
}

fn solve_part_2(input: impl Iterator<Item = String>) -> usize {
    let mut window = vec![];

    let mut total_priority = 0;

    for line in input {
        window.push(line);

        if window.len() == 3 {
            let triple = find_triple(&window[0], &window[1], &window[2]);
            total_priority += prioritize_char(triple);
            window.clear();
        }
    }

    total_priority
}

fn find_double(input: String) -> char {
    let (first, second) = input.split_at(input.len() / 2);

    for char in first.chars() {
        if second.contains(char) {
            return char;
        }
    }

    unreachable!("input did not contain a char twice")
}

fn find_triple(first: &str, second: &str, third: &str) -> char {
    for char in first.chars() {
        if second.contains(char) && third.contains(char) {
            return char;
        }
    }

    unreachable!("input did not contain a char thrice")
}

fn prioritize_char(c: char) -> usize {
    let c: u8 = c.try_into().expect("char is not an ascii char");

    let priority = match c as u8 {
        97..=122 => c - 96,
        65..=90 => c - 38,
        _ => panic!("got a non-letter ascii char"),
    };

    priority as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 6] = [
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_find_double() {
        let expected_doubles = vec!['p', 'L', 'P', 'v', 't', 's'];
        let actual_doubles = iter_input().map(find_double).collect::<Vec<_>>();

        assert_eq!(expected_doubles, actual_doubles);
    }

    #[test]
    fn test_prioritize_char() {
        let chars = ['p', 'L', 'P', 'v', 't', 's'];
        let expected_priorities = [16, 38, 42, 22, 20, 19];
        let actual_priorities = chars.map(prioritize_char);

        assert_eq!(expected_priorities, actual_priorities);
    }

    #[test]
    fn test_find_triple() {
        let expected_triples = ['r', 'Z'];
        let actual_triples = [
            find_triple(EXAMPLE_INPUT[0], EXAMPLE_INPUT[1], EXAMPLE_INPUT[1]),
            find_triple(EXAMPLE_INPUT[3], EXAMPLE_INPUT[4], EXAMPLE_INPUT[5]),
        ];

        assert_eq!(expected_triples, actual_triples);
    }

    #[test]
    fn test_part_1() {
        let solution = solve_part_1(iter_input());

        assert_eq!(solution, 157);
    }

    #[test]
    fn test_part_2() {
        let solution = solve_part_2(iter_input());

        assert_eq!(solution, 70);
    }
}
