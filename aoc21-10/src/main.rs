use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut corruption_score = 0;
    let mut completetion_scores = Vec::new();
    for l in contents.lines() {
        let mut stack = Vec::new();
        let mut corrupted = false;
        for c in l.chars() {
            if ['<', '(', '[', '{'].contains(&c) {
                stack.push(c);
            } else {
                let (opener, pen) = match c {
                    ')' => ('(', 3),
                    ']' => ('[', 57),
                    '}' => ('{', 1197),
                    '>' => ('<', 25137),
                    _ => panic!(),
                };
                if stack.pop() != Some(opener) {
                    corruption_score += pen;
                    corrupted = true;
                    break;
                }
            }
        }
        if corrupted {
            continue;
        }
        let mut score = 0u64;
        while let Some(c) = stack.pop() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            };
        }
        completetion_scores.push(score);
    }
    println!("corruption score: {}", corruption_score);
    completetion_scores.sort_unstable();
    let mid_compl_score = completetion_scores[completetion_scores.len() / 2];
    println!("middling completion score: {}", mid_compl_score);
}
