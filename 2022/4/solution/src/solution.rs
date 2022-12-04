pub fn solve(input: impl Iterator<Item = String>) -> usize {
    solve_part_1(input)
}

fn solve_part_1(input: impl Iterator<Item = String>) -> usize {
    input
        .map(|l| parse_ranges(&l))
        .map(is_subrange)
        .map(|r| r as usize)
        .sum::<usize>()
}

fn solve_part_2(input: impl Iterator<Item = String>) -> usize {
    todo!()
}

type RangeTuple = (usize, usize);

fn parse_ranges(ranges: &str) -> (RangeTuple, RangeTuple) {
    let (first_range, second_range) = ranges.split_once(',').expect("ranges didn't contain ','");

    (parse_range(first_range), parse_range(second_range))
}

fn parse_range(range: &str) -> RangeTuple {
    let (start, end) = range.split_once('-').expect("range didn't contain '-'");

    (start.parse().unwrap(), end.parse().unwrap())
}

fn is_subrange(ranges: (RangeTuple, RangeTuple)) -> bool {
    let (first_range, second_range) = ranges;
    let (f_start, f_end) = first_range;
    let (s_start, s_end) = second_range;

    (f_start <= s_start && s_end <= f_end) || (s_start <= f_start && f_end <= s_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 6] = [
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];
    const PART_1_ANSWER: usize = 2;
    const PART_2_ANSWER: usize = 2;

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_parse_ranges() {
        let expected_ranges = [
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];
        let actual_ranges = EXAMPLE_INPUT.map(parse_ranges);

        assert_eq!(expected_ranges, actual_ranges);
    }

    #[test]
    fn test_is_subrange() {
        let expected_results = [false, false, false, true, true, false];

        let actual_results = EXAMPLE_INPUT.map(parse_ranges).map(is_subrange);

        assert_eq!(expected_results, actual_results);
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
