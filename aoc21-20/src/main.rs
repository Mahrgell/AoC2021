use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut key = Vec::new();
    let mut field = HashSet::new();
    let mut key_done = false;
    let mut row_i = 0i32;
    let mut max_col = 0i32;
    for l in contents.lines() {
        if l == "" {
            key_done = true;
        }
        else if !key_done {
            key = l.chars().map(|c| c== '#').collect();

        } else {
            let row : Vec<_> = l.chars().map(|c| c== '#').collect();
            max_col = i32::max(max_col, row.len() as i32);
            for (i, &b) in row.iter().enumerate() {
                if b {
                    field.insert((row_i, i as i32));
                }
            }
            row_i += 1;
        }
    }
    let mut min_row = 0;
    let mut max_row = row_i-1;
    let mut min_col = 0;
    for i in 1..=50 {
        min_row -= 1;
        max_row += 1;
        min_col -= 1;
        max_col += 1;
        let mut new_field = HashSet::new();
        for r in (min_row)..=(max_row) {
            for c in (min_col)..=(max_col) {
                let mut val = 0;
                for nr in (r-1)..=(r+1) {
                    for nc in (c-1)..=(c+1) {
                        val <<= 1;
                        let light = if nr > min_row && nr < max_row && nc > min_col && nc < max_col {
                            field.contains(&(nr,nc))
                        }
                        else {
                            !key[0] || ((i%2) == 0)
                        };
                        val += light as usize;
                    }
                }
                if key[val] {
                    new_field.insert((r, c));
                }
            }
        }
        field = new_field;
        if [2, 50].contains(&i) {
            println!("{}", field.len());
        }
    }
}
