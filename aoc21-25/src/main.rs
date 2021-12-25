use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut field = Vec::new();
    for l in contents.lines() {
        let row: Vec<_> = l
            .chars()
            .map(|c| match c {
                '.' => 0u8,
                '>' => 1,
                'v' => 2,
                _ => panic!(),
            })
            .collect();
        field.push(row);
    }
    let nb_rows = field.len();
    let nb_cols = field[0].len();
    let mut count = 0;
    loop {
        count += 1;
        let mut something_moved = false;
        for t in [1, 2] {
            let mut new_field = field.clone();
            for r in 0..nb_rows {
                for c in 0..nb_cols {
                    let nr = (r + t - 1) % nb_rows;
                    let nc = (c + 2 - t) % nb_cols;
                    if field[r][c] == t as u8 && field[nr][nc] == 0 {
                        something_moved = true;
                        new_field[r][c] = 0;
                        new_field[nr][nc] = t as u8;
                    }
                }
            }
            field = new_field;
        }
        if !something_moved {
            break;
        }
    }
    println!("Sea cucumbers stop moving at turn {}", count);
}
