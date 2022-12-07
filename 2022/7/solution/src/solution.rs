use std::collections::HashMap;

type Solution = usize;

pub fn solve(input: impl Iterator<Item = String>) -> Solution {
    solve_part_2(input)
}

fn solve_part_1(input: impl Iterator<Item = String>) -> Solution {
    let dir_sizes = get_dir_sizes(input);

    let mut total_sizes = 0;
    for &size in dir_sizes.values() {
        if size <= 100000 {
            total_sizes += size;
        }
    }

    total_sizes
}

fn get_dir_sizes(input: impl Iterator<Item = String>) -> HashMap<String, usize> {
    let mut dir_sizes = HashMap::new();
    let mut path_from_root = vec![];

    for line in input {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        match parts.as_slice() {
            ["$", "cd", "/"] => {
                path_from_root = vec!["/".to_string()];
            }
            ["$", "cd", ".."] => {
                path_from_root
                    .pop()
                    .expect("tried to move to non-existent parent dir");
            }
            ["$", "cd", target] => {
                path_from_root.push(target.to_string());
            }
            ["$", "ls"] => {}
            ["dir", _dir_name] => {}
            [file_size, _file_name] => {
                for full_path in path_from_root.iter().scan(vec![], |p, d| {
                    p.push(d.to_string());
                    Some(p.join("/"))
                }) {
                    let file_size: usize = file_size.parse().expect("failed to parse file size");

                    *dir_sizes.entry(full_path).or_default() += file_size;
                }
            }
            _ => panic!("got unexpected command: {}", line),
        };
    }

    dir_sizes
}

fn solve_part_2(input: impl Iterator<Item = String>) -> Solution {
    let total_disk_space = 70000000;
    let desired_disk_space = 30000000;

    let dir_sizes = get_dir_sizes(input);

    let free_disk_space = total_disk_space - dir_sizes.get("/").expect("missing '/' dir size");
    let missing_disk_space = desired_disk_space - free_disk_space;

    let mut min_dir_size = None;

    for &dir_size in dir_sizes.values() {
        if dir_size >= missing_disk_space {
            min_dir_size = match min_dir_size {
                None => Some(dir_size),
                Some(min_dir_size) if dir_size < min_dir_size => Some(dir_size),
                Some(min_dir_size) => Some(min_dir_size),
            };
        }
    }

    min_dir_size.expect("no dir satisfies space requirements")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&str; 23] = [
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];
    const PART_1_ANSWER: usize = 95437;
    const PART_2_ANSWER: usize = 24933642;

    fn iter_input() -> impl Iterator<Item = String> {
        EXAMPLE_INPUT.into_iter().map(|s| s.into())
    }

    #[test]
    fn test_get_dir_sizes() {
        let expected_sizes = HashMap::from([
            ("//a/e".to_string(), 584),
            ("//a".to_string(), 94853),
            ("//d".to_string(), 24933642),
            ("/".to_string(), 48381165),
        ]);
        let actual_sizes = get_dir_sizes(iter_input());

        assert_eq!(expected_sizes, actual_sizes);
    }

    #[test]
    fn test_part_1() {
        let solution = solve_part_1(iter_input());

        assert_eq!(solution, PART_1_ANSWER);
    }

    #[test]
    fn test_part_2() {
        let solution = solve_part_2(iter_input());

        assert_eq!(solution, PART_2_ANSWER);
    }
}
