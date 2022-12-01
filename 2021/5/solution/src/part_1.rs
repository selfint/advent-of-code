use std::collections::HashMap;

pub fn solve<T, I>(input: I) -> usize
where
    T: Into<String>,
    I: Iterator<Item = T>,
{
    let sums: HashMap<[usize; 2], usize> = HashMap::new();

    for line in input {
        let ((src_x, src_y), (dst_x, dst_y)) = parse(line.into());
    }

    0
}

fn parse(line: String) -> ((usize, usize), (usize, usize)) {
    let line = line.split(" -> ").collect::<Vec<&str>>();

    let (src, dst) = match line[..] {
        [src, dst] => (src, dst),
        _ => panic!("failed to parse line"),
    };

    let [src_x, src_y] = src
        .split(',')
        .map(|s| s.parse::<usize>().expect("failed to parse"))
        .collect::<Vec<usize>>()[..] else {
            panic!("failed to parse src");
        };

    let [dst_x, dst_y] = dst
        .split(',')
        .map(|s| s.parse::<usize>().expect("failed to parse"))
        .collect::<Vec<usize>>()[..] else {
            panic!("failed to parse dst");
        };

    ((src_x, src_y), (dst_x, dst_y))
}

#[cfg(test)]
mod tests {
    use crate::part_1::parse;

    use super::solve;

    const EXAMPLE_INPUT: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn test_parse_line() {
        assert_eq!(parse("0,9 -> 5,9".into()), ((0, 9), (5, 9)));
    }

    #[test]
    fn test_example() {
        let lines = EXAMPLE_INPUT.into_iter();

        let solution = solve(lines);

        assert_eq!(solution, 5);
    }
}
