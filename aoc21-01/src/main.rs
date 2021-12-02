use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let numbers: Vec<_> = contents.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let mut last_last_num = 999999;
    let mut last_num = 999999;
    let mut last_sliding_sum = 999999999;
    let mut incs = 0;
    let mut sliding_incs = 0;
    for n in numbers {
        if n > last_num {
            incs += 1;
        }
        let sum = last_last_num+last_num+n;
        if sum > last_sliding_sum {
            sliding_incs += 1;
        }
        last_sliding_sum = sum;
        last_last_num = last_num;
        last_num = n;
    }
    println!("{} increments", incs);
    println!("{} sliding increments", sliding_incs);
}