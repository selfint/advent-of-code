use std::cmp::Ordering;

pub fn solve<T, I>(input: I) -> u32
where
    T: Into<String>,
    I: Iterator<Item = Option<T>>,
{
    let input = input
        .take_while(|l| l.is_some())
        .map(|l| -> String { l.unwrap().into() })
        .collect::<Vec<_>>();

    let total_bits = input[0].chars().count();

    let mut oxy_input = input.clone();
    let mut co2_input = input;

    let mut oxy_rate = String::new();
    let mut co2_rate = String::new();

    for _ in 0..total_bits {
        if oxy_input.len() == 1 {
            oxy_rate.push_str(&oxy_input[0]);
            break;
        }

        let [oxy_zero, oxy_ones] = get_counts(&oxy_input);

        let oxy_keep_digit = match oxy_zero.cmp(&oxy_ones) {
            Ordering::Less => '1',
            Ordering::Equal => '1',
            Ordering::Greater => '0',
        };

        oxy_rate.push(oxy_keep_digit);
        oxy_input = update_input(&oxy_input, oxy_keep_digit);
    }

    for _ in 0..total_bits {
        if co2_input.len() == 1 {
            co2_rate.push_str(&co2_input[0]);
            break;
        }

        let [co2_zero, co2_one] = get_counts(&co2_input);
        let co2_keep_digit = match co2_zero.cmp(&co2_one) {
            Ordering::Less => '0',
            Ordering::Equal => '0',
            Ordering::Greater => '1',
        };

        co2_rate.push(co2_keep_digit);
        co2_input = update_input(&co2_input, co2_keep_digit);
    }

    let oxy_rate = u32::from_str_radix(&oxy_rate, 2).expect("Not a binary number!");
    let co2_rate = u32::from_str_radix(&co2_rate, 2).expect("Not a binary number!");

    oxy_rate * co2_rate
}

fn update_input(input: &[String], keep_digit: char) -> Vec<String> {
    input
        .iter()
        .filter_map(|l| l.strip_prefix(keep_digit).map(|inner| inner.to_owned()))
        .collect()
}

fn get_counts(input: &[String]) -> [usize; 2] {
    input.iter().fold([0, 0], |mut counts, line| {
        let digit = line.chars().next().expect("got empty line");

        match digit {
            '0' => counts[0] += 1,
            '1' => counts[1] += 1,
            _ => panic!("got unexpected digit '{}'", digit),
        };

        counts
    })
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

        assert_eq!(solution, 230);
    }
}
