mod part_1;
mod part_2;

use std::io;

use part_2::solve;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.expect("failed to get line from stdin"))
        .map(|l| if l.is_empty() { None } else { Some(l) });

    let solution = solve(input);

    println!("solution = {}", solution);
}
