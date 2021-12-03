use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut x = 0u64;
    let mut depth = 0u64;
    let mut aim = 0u64;
    let mut depth2 = 0u64;
    for l in contents.lines() {
        let words: Vec<_> = l.split(' ').collect();
        let val = words[1].parse::<u64>().unwrap();
        match words[0] {
            "forward" => {
                x += val;
                depth2 += aim * val;
            }
            "up" => {
                depth -= val;
                aim -= val;
            }
            "down" => {
                depth += val;
                aim += val;
            }
            _ => panic!(),
        }
    }

    println!("pos x = {}, d= {}, prod = {}", x, depth, x * depth);
    println!("pos x = {}, d2= {}, prod2 = {}", x, depth2, x * depth2);
}
