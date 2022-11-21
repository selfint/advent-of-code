pub fn solve<T, I>(mut input: I) -> u32
where
    T: Into<String>,
    I: Iterator<Item = Option<T>>,
{
    let mut counts: Vec<[usize; 2]> = vec![];

    // init counts with data from first line
    let Some(Some(first_line)) = input.next() else {return 0;};
    let first_line: &str = &first_line.into();

    for (idx, digit) in first_line.char_indices() {
        counts.push([0, 0]);

        match digit {
            '0' => counts[idx][0] += 1,
            '1' => counts[idx][1] += 1,
            _ => panic!("got unexpected digit {}", digit),
        }
    }

    // get counts from rest of lines
    for line in input {
        let Some(line) = line else {break;};

        // TODO: does this do nothing if we received &str items?
        let line: &str = &line.into();

        for (idx, digit) in line.char_indices() {
            match digit {
                '0' => counts[idx][0] += 1,
                '1' => counts[idx][1] += 1,
                _ => panic!("got unexpected digit {}", digit),
            }
        }
    }

    // get binary values for rates
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for [zero_count, one_count] in counts {
        match zero_count.cmp(&one_count) {
            std::cmp::Ordering::Less => {
                gamma_rate.push('1');
                epsilon_rate.push('0');
            }
            std::cmp::Ordering::Greater => {
                gamma_rate.push('0');
                epsilon_rate.push('1');
            }
            std::cmp::Ordering::Equal => {
                panic!("zero count equals one count, what are we supposed to do?")
            }
        }
    }

    // convert rates to decimal
    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).expect("Not a binary number!");
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).expect("Not a binary number!");

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_example() {
        let lines = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .map(Some)
        .into_iter();

        let solution = solve(lines);

        assert_eq!(solution, 198);
    }
}
