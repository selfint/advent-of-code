pub fn solve<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    input.map(find_double).map(prioritize_char).sum()
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
    fn test_example() {
        let solution = solve(iter_input());

        assert_eq!(solution, 157);
    }
}
