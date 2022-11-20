use std::collections::VecDeque;

pub fn solve<I>(input: I) -> usize
where
    I: Iterator<Item = Option<usize>>,
{
    let mut increased_amount = 0;
    let mut window_sum = 0;
    let mut window: VecDeque<usize> = VecDeque::with_capacity(3);
    let mut prev_window_sum = None;

    for depth in input {
        let Some(depth) = depth else {break;};

        window.push_back(depth);
        window_sum += depth;

        if window.len() < 3 {
            continue;
        }

        if window.len() > 3 {
            window_sum -= window.pop_front().unwrap();

            if let Some(prev_window_sum) = prev_window_sum {
                if window_sum > prev_window_sum {
                    increased_amount += 1;
                }
            }
        }

        prev_window_sum = Some(window_sum);
    }

    increased_amount
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_example() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
            .into_iter()
            .map(Some);

        let solution = solve(input);

        assert_eq!(solution, 5);
    }

    #[test]
    fn test_window_size() {
        let input = [0, 0, 1, 0, 1].into_iter().map(Some);

        let solution = solve(input);

        assert_eq!(solution, 2);
    }
}
