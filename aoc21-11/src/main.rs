use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut num = Vec::new();
    for l in contents.lines() {
        let row: Vec<_> = l.chars().map(|c| Some(c.to_digit(10).unwrap())).collect();
        num.push(row);
    }
    let rows = num.len();
    let cols = num[0].len();
    let mut flash_count = 0;
    for turn in 1.. {
        for row in &mut num {
            for d in row {
                *d = Some(d.unwrap_or(0) + 1);
            }
        }
        let mut done = false;
        while !done {
            done = true;
            for r in 0..rows {
                for c in 0..cols {
                    if let Some(x) = num[r][c] {
                        if x > 9 {
                            num[r][c] = None;
                            done = false;
                            flash_count += 1;
                            let min_r = if r > 0 { r - 1 } else { 0 };
                            let min_c = if c > 0 { c - 1 } else { 0 };
                            let max_r = if r < rows - 1 { r + 1 } else { rows - 1 };
                            let max_c = if c < cols - 1 { c + 1 } else { cols - 1 };
                            for nr in min_r..=max_r {
                                for nc in min_c..=max_c {
                                    if let Some(n) = num[nr][nc] {
                                        num[nr][nc] = Some(n + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if turn == 100 {
            println!("Flash counter at turn 100: {}", flash_count);
        }
        if num.iter().all(|row| row.iter().all(|&x| x == None)) {
            println!("All octopi flashed at turn {}", turn);
            break;
        }
    }
}
