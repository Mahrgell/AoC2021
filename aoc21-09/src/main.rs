use std::fs;

fn get_basin_size(field: &Vec<Vec<u32>>, pos: (i32, i32)) -> i32 {
    let mut unproc = Vec::new();
    let mut proc = Vec::new();
    unproc.push(pos);
    while let Some(curr_pos) = unproc.pop() {
        for offs in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_pos = (curr_pos.0 + offs.0, curr_pos.1 + offs.1);
            if new_pos.0 < 0
                || new_pos.0 >= field.len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= field[0].len() as i32
            {
                continue;
            }
            if field[new_pos.0 as usize][new_pos.1 as usize] == 9 {
                continue;
            }
            if !proc.contains(&new_pos) && !unproc.contains(&new_pos) {
                unproc.push(new_pos);
            }
        }
        proc.push(curr_pos);
    }
    proc.len() as i32
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut field: Vec<Vec<_>> = Vec::new();
    for l in contents.lines() {
        let row: Vec<_> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
        field.push(row);
    }
    let rows = field.len();
    let cols = field[0].len();
    let mut low_points_sum = 0;
    let mut basin_sizes = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let d = field[r][c];
            if r > 0 && field[r - 1][c] <= d {
                continue;
            }
            if r < rows - 1 && field[r + 1][c] <= d {
                continue;
            }
            if c > 0 && field[r][c - 1] <= d {
                continue;
            }
            if c < cols - 1 && field[r][c + 1] <= d {
                continue;
            }
            low_points_sum += 1 + d;
            basin_sizes.push(get_basin_size(&field, (r as i32, c as i32)));
        }
    }

    println!("The sum of risk levels is {}", low_points_sum);
    basin_sizes.sort();
    let mut basin_size_product = basin_sizes.pop().unwrap();
    basin_size_product *= basin_sizes.pop().unwrap();
    basin_size_product *= basin_sizes.pop().unwrap();
    println!(
        "The product of the 3 largest basin sizes is {}",
        basin_size_product
    );
}
