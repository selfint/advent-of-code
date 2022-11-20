pub fn solve<I>(input: I) -> usize
where
    I: Iterator<Item = Option<usize>>,
{
    let mut increased_amount = 0;
    let mut prev_depth = None;

    for line in input {
        let Some(depth) = line else { break; };

        if let Some(prev_depth) = prev_depth {
            if depth > prev_depth {
                increased_amount += 1;
            }
        }

        prev_depth = Some(depth);
    }

    increased_amount
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_simple() {
        let input = [0, 1, 2].into_iter().map(Some);

        let solution = solve(input);

        assert_eq!(solution, 2);
    }
}
