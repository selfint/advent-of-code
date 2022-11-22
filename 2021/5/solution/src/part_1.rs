pub fn solve<T, I>(mut input: I) -> usize
where
    T: Into<String>,
    I: Iterator<Item = T>,
{
    0
}

#[cfg(test)]
mod tests {
    use super::solve;

    const EXAMPLE_INPUT: [&str; 0] = [];

    #[test]
    fn test_example() {
        let lines = EXAMPLE_INPUT.into_iter();

        let solution = solve(lines);

        assert_eq!(solution, 4512);
    }
}
