use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut points = Vec::new();
    let mut folds = Vec::new();
    let mut points_completed = false;
    for l in contents.lines() {
        if l == "" {
            points_completed = true;
            continue;
        }
        if !points_completed {
            let coords: Vec<_> = l.split(',').map(|w| w.parse::<u32>().unwrap()).collect();
            points.push((coords[0], coords[1]));
        } else {
            let re = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
            let cap = re.captures(l).unwrap();
            let axis = cap[1].chars().nth(0).unwrap();
            let val = cap[2].parse::<u32>().unwrap();
            folds.push((axis, val));
        }
    }
    for (axis, val) in folds {
        for (x, y) in &mut points {
            let a = if axis == 'x' { x } else { y };
            if *a > val {
                *a = 2 * val - *a
            }
        }
        points.sort();
        points.dedup();
        println!("{} points remaining.", points.len());
    }
    let mut max = (0, 0);
    for (x, y) in &points {
        if *x > max.0 {
            max.0 = *x;
        }
        if *y > max.1 {
            max.1 = *y;
        }
    }
    let mut output = vec![vec!['.'; max.0 as usize + 1]; max.1 as usize + 1];
    for (x, y) in points {
        output[y as usize][x as usize] = '*';
    }
    for l in output {
        let s: String = l.into_iter().collect();
        println!("{}", s);
    }
}
