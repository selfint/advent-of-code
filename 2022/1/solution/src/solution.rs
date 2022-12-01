pub fn solve<T, I>(input: I) -> usize
where
    T: Into<String>,
    I: Iterator<Item = T>,
{
    let mut max_sums = Vec::with_capacity(3);
    let mut current_sum = 0;

    for line in input {
        let line: String = line.into();
        if line.is_empty() {
            if max_sums.len() < 3 {
                max_sums.push(current_sum);
            } else {
                let mut min_max_sum = None;
                let mut min_max_sum_index = None;
                for (i, &max_sum) in max_sums.iter().enumerate() {
                    if current_sum > max_sum {
                        if let Some(min_sum) = min_max_sum {
                            if max_sum < min_sum {
                                min_max_sum_index = Some(i);
                                min_max_sum = Some(max_sum);
                            }
                        } else {
                            min_max_sum = Some(max_sum);
                            min_max_sum_index = Some(i);
                        }
                    }
                }

                if let Some(min_sum_index) = min_max_sum_index {
                    max_sums[min_sum_index] = current_sum;
                }
            }

            current_sum = 0;

            continue;
        }

        let amount = line.parse::<usize>().expect("failed to parse line");

        current_sum += amount;
    }

    max_sums.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    const EXAMPLE_INPUT: [&str; 15] = [
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000", "",
    ];

    #[test]
    fn test_example() {
        let lines = EXAMPLE_INPUT.into_iter();

        let solution = solve(lines);

        assert_eq!(solution, 45000);
    }
}
