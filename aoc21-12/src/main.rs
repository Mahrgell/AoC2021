use std::collections::HashMap;
use std::fs;

fn number_of_pathes(connections: &HashMap<String, Vec<String>>, allow_twice: bool) -> u32 {
    let mut todo = vec![(String::new(), "start", allow_twice)];
    let mut solution_counter = 0;
    while let Some((path, last, twice_available)) = todo.pop() {
        for next in &connections[last] {
            if next == "start" {
                continue;
            }
            if next == "end" {
                solution_counter += 1;
                continue;
            }
            let mut new_ta = twice_available;
            if next.chars().nth(0).unwrap().is_lowercase() && path.contains(next) {
                if new_ta {
                    new_ta = false;
                } else {
                    continue;
                }
            }
            let new_path = path.clone() + "-" + next;
            todo.push((new_path, &next, new_ta));
        }
    }
    solution_counter
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for l in contents.lines() {
        let words: Vec<_> = l.split('-').collect();
        for (from, to) in [(words[0], words[1]), (words[1], words[0])] {
            connections
                .entry(String::from(from))
                .or_insert(Vec::new())
                .push(String::from(to));
        }
    }
    println!(
        "Number of pathes: {}",
        number_of_pathes(&connections, false)
    );
    println!(
        "Number of pathes with one double visit: {}",
        number_of_pathes(&connections, true)
    );
}
