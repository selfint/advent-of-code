mod part_1;

use std::io;

use crate::part_1::solve;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.expect("failed to read line from stdin").parse().ok());

    let solution = solve(input);

    println!("solution = {}", solution);
}
