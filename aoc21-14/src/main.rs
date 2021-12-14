use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let polymer: Vec<_> = contents.lines().nth(0).unwrap().chars().collect();
    let mut sub_rules = HashMap::new();
    for l in contents.lines() {
        let words: Vec<_> = l.split(" -> ").collect();
        if words.len() != 2 {
            continue;
        }
        let from1 = words[0].chars().nth(0).unwrap();
        let from2 = words[0].chars().nth(1).unwrap();
        let to = words[1].chars().nth(0).unwrap();
        sub_rules.insert((from1, from2), to);
    }
    let mut pairs = HashMap::new();
    let final_char = polymer[polymer.len() - 1];
    for i in 1..polymer.len() {
        *pairs.entry((polymer[i - 1], polymer[i])).or_insert(0u64) += 1;
    }
    for i in 1..=40 {
        let mut new_pairs = HashMap::new();
        for (p, val) in &pairs {
            if let Some(c) = sub_rules.get(p) {
                *new_pairs.entry((p.0, *c)).or_insert(0) += val;
                *new_pairs.entry((*c, p.1)).or_insert(0) += val;
            } else {
                *new_pairs.entry(*p).or_insert(0) += val;
            }
        }
        pairs = new_pairs;
        if [10, 40].contains(&i) {
            let mut char_occ = HashMap::new();
            for ((c, _), val) in &pairs {
                *char_occ.entry(*c).or_insert(0) += val;
            }
            *char_occ.entry(final_char).or_insert(0) += 1;
            let solution = char_occ.values().max().unwrap() - char_occ.values().min().unwrap();
            println!("{}: {}", i, solution);
        }
    }
}
