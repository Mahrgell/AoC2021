use std::collections::HashMap;
use std::fs;

fn to_coord(s: &str) -> (i32, i32) {
    let n: Vec<_> = s.split(',').map(|l| l.parse::<i32>().unwrap()).collect();
    (n[0], n[1])
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut field1: HashMap<(i32, i32), i32> = HashMap::new();
    let mut field2: HashMap<(i32, i32), i32> = HashMap::new();
    for l in contents.lines() {
        let words: Vec<_> = l.split(" -> ").collect();
        let c1 = to_coord(words[0]);
        let c2 = to_coord(words[1]);
        let mut dx = c2.0 - c1.0;
        let mut dy = c2.1 - c1.1;
        let is_diag = dx != 0 && dy != 0;
        let steps = std::cmp::max(i32::abs(dx), i32::abs(dy));
        if steps > 0 {
            dx /= steps;
            dy /= steps;
        }
        for i in 0..=steps {
            let key = (c1.0 + i * dx, c1.1 + i * dy);
            if !is_diag {
                *field1.entry(key).or_insert(0) += 1;
            }
            *field2.entry(key).or_insert(0) += 1;
        }
    }

    let cnt1 = field1.values().filter(|&&v| v >= 2).count();
    let cnt2 = field2.values().filter(|&&v| v >= 2).count();
    println!("{} fields are intersecting. (no diag)", cnt1);
    println!("{} fields are intersecting. (incl diag)", cnt2);
}
