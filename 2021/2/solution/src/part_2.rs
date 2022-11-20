pub fn solve<T, I>(input: I) -> i32
where
    T: Into<String>,
    I: Iterator<Item = Option<T>>,
{
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input {
        let Some(line) = line else {break;};

        // TODO: does this do nothing if we received &str items?
        let line: &str = &line.into();

        let mut parts = line.split_ascii_whitespace();

        let command = parts.next().expect("line didn't contain command");
        let amount: i32 = parts
            .next()
            .expect("line didn't contain amount")
            .parse()
            .expect("failed to parse amount");

        match command {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("got unexpected command"),
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_example() {
        let lines = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .map(Some)
        .into_iter();

        let solution = solve(lines);

        assert_eq!(solution, 900);
    }
}
