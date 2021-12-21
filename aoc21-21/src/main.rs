use std::collections::HashMap;
use std::fs;

type ComboMap = HashMap<([u32; 2], [u64; 2], usize), [u64; 2]>;

fn compute_combos(pos: &[u32; 2], score: &[u64; 2], p: usize, combos: &mut ComboMap) -> [u64; 2] {
    if let Some(res) = combos.get(&(*pos, *score, p)) {
        return *res;
    }
    let mut res = [0, 0];
    for (d, l) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let posp = (pos[p] + d - 1) % 10 + 1;
        let s = score[p] + posp as u64;
        if s >= 21 {
            res[p] += l;
        } else {
            let mut new_pos = *pos;
            new_pos[p] = posp;
            let mut new_score = *score;
            new_score[p] = s;
            let subr = compute_combos(&new_pos, &new_score, 1 - p, combos);
            res[0] += subr[0] * l;
            res[1] += subr[1] * l;
        }
    }
    combos.insert((*pos, *score, p), res);
    res
}

fn main() {
    // read input
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let p1 = contents
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();
    let p2 = contents
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();
    let mut pos = [p1, p2];
    let mut score = [0, 0];
    let mut d = 1;
    let mut done = false;
    while !done {
        for p in [0, 1] {
            let dd = 3 * d + 3;
            d += 3;
            pos[p] = (pos[p] + dd - 1) % 10 + 1;
            score[p] += pos[p];
            if score[p] >= 1000 {
                println!(
                    "d: {}, other: {} -> {}",
                    d - 1,
                    score[1 - p],
                    (d - 1) * score[1 - p]
                );
                done = true;
                break;
            }
        }
    }

    println!(
        "Nb of won universes by winner: {}",
        compute_combos(&[p1, p2], &[0, 0], 0, &mut ComboMap::new()).iter().max().unwrap()
    );
}
