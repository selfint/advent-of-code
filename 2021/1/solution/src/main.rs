mod part_1;
mod part_2;

use std::io;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.expect("failed to read line from stdin").parse().ok());

    let solution = part_2::solve(input);

    println!("solution = {}", solution);
}
