use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let numbers: Vec<_> = contents.lines().nth(0).unwrap().split(',').collect();
    let mut numbers: Vec<_> = numbers.iter().map(|l| l.parse::<i32>().unwrap()).collect();
    numbers.sort();
    let median = numbers[numbers.len() / 2];
    println!(
        "There are {} numbers, the median is {}.",
        numbers.len(),
        median
    );
    let cost = numbers.iter().fold(0, |acc, v| acc + i32::abs(v - median));
    println!("TIt costs {} to move them all to that position.", cost);
    let mut best_cost = 999999999;
    let mut best_x = median;
    loop {
        let cost = numbers.iter().fold(0, |acc, v| {
            let d = i32::abs(v - best_x);
            acc + d * (d + 1) / 2
        });
        if cost < best_cost {
            best_cost = cost;
            best_x += 1; // educated guess on direction
        } else {
            break;
        }
    }
    println!("Best advanced cost: {} @ x = {}", best_cost, best_x - 1);
}
