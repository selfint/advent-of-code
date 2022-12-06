use std::collections::{HashMap, VecDeque};

pub fn solve(mut input: impl Iterator<Item = String>) -> usize {
    solve_part_1(&input.next().unwrap())
}

fn solve_part_1(input: &str) -> usize {
    let mut buffer = VecDeque::new();
    let mut seen_chars: HashMap<char, usize> = HashMap::new();

    for (i, char) in input.char_indices() {
        buffer.push_front(char);
        *seen_chars.entry(char).or_default() += 1;

        if buffer.len() == 4 {
            if seen_chars.keys().count() == 4 {
                return i + 1;
            } else {
                let last_char = buffer.pop_back().unwrap();
                let last_char_count = seen_chars.get_mut(&last_char).unwrap();

                *last_char_count -= 1;

                if *last_char_count == 0 {
                    seen_chars.remove(&last_char);
                }
            }
        }
    }

    unreachable!("input didn't contain packet header");
}

fn solve_part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    const PART_1_ANSWER: [usize; 5] = [7, 5, 6, 10, 11];
    const PART_2_ANSWER: [usize; 5] = [7, 5, 6, 10, 11];

    #[test]
    fn test_part_1() {
        let solutions = EXAMPLE_INPUT.map(solve_part_1);

        assert_eq!(solutions, PART_1_ANSWER);
    }

    #[test]
    fn test_part_2() {
        let solutions = EXAMPLE_INPUT.map(solve_part_2);

        assert_eq!(solutions, PART_2_ANSWER);
    }
}
