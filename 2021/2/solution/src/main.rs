use std::io;

use part_1::solve;

mod part_1;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.expect("failed to get line from stdin"))
        .map(|l| if l.is_empty() { None } else { Some(l) });

    let solution = solve(input);

    println!("solution = {}", solution);
}
