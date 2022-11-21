mod part_1;

use std::io;

use part_1::solve;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.expect("failed to get line from stdin"))
        .take_while(|l| l != "done");

    let solution = solve(input);

    println!("solution = {}", solution);
}
