mod solution;

use std::io;

use solution::solve;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.expect("failed to get line from stdin"))
        .take_while(|l| !l.is_empty());

    let solution = solve(input);

    println!("solution = {}", solution);
}
