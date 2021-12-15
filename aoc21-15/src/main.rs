use std::fs;

fn shortest_path_length(field: &Vec<Vec<u32>>, multiplier: usize) -> Option<u32> {
    let rows = field.len();
    let cols = field[0].len();
    let mut visited = vec![vec![false; cols * multiplier]; rows * multiplier];
    visited[0][0] = true;
    let mut todo = Vec::new();
    todo.push((0, (0, 0)));
    while let Some((cost, (x, y))) = todo.pop() {
        if (x, y) == (multiplier * rows - 1, multiplier * cols - 1) {
            return Some(cost);
        }
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x < multiplier * rows - 1 {
            neighbors.push((x + 1, y));
        }
        if y < multiplier * cols - 1 {
            neighbors.push((x, y + 1));
        }
        for (nx, ny) in neighbors {
            if visited[nx][ny] {
                continue;
            }
            visited[nx][ny] = true;
            let mut nc = field[nx % rows][ny % rows];
            nc += (nx / rows + ny / cols) as u32;
            if nc > 9 {
                nc -= 9;
            }
            todo.push((cost + nc, (nx, ny)));
        }
        todo.sort_by(|a, b| b.cmp(a));
    }
    None
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut field = Vec::new();
    for l in contents.lines() {
        let row: Vec<_> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
        field.push(row);
    }
    println!("Multiplier 1: {}", shortest_path_length(&field, 1).unwrap());
    println!("Multiplier 5: {}", shortest_path_length(&field, 5).unwrap());
}
