use std::fs;

fn add_nb_and_check(board: &mut Vec<Option<i32>>, nb: i32) -> Option<i32> {
    let idx = board.iter().position(|&x| x == Some(nb));
    if let Some(i) = idx {
        board[i] = None;
        let row = i / 5;
        let col = i % 5;
        let mut row_complete = true;
        let mut col_complete = true;
        for ii in 0..5 {
            if let Some(_) = board[row * 5 + ii] {
                row_complete = false;
            }
            if let Some(_) = board[ii * 5 + col] {
                col_complete = false;
            }
        }
        if row_complete || col_complete {
            return Some(
                nb * board
                    .iter()
                    .fold(0, |acc, x| if let Some(val) = x { acc + val } else { acc }),
            );
        }
    }
    None
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut numbers_drawn: Vec<_> = vec![];
    let mut bingo_boards: Vec<Vec<Option<i32>>> = Vec::with_capacity(100);
    for l in contents.lines() {
        let words: Vec<_> = l.split(' ').collect();
        if words.len() == 1 {
            if words[0] == "" {
                bingo_boards.push(Vec::with_capacity(25));
            } else {
                numbers_drawn = words[0]
                    .split(',')
                    .map(|l| l.parse::<i32>().unwrap())
                    .collect();
            }
        } else {
            let mut row: Vec<_> = words
                .iter()
                .filter(|&l| l != &"")
                .map(|l| Some(l.parse::<i32>().unwrap()))
                .collect();
            bingo_boards.last_mut().unwrap().append(&mut row);
        }
    }

    for nb in numbers_drawn {
        for mut b in &mut bingo_boards {
            if let Some(score) = add_nb_and_check(&mut b, nb) {
                println!("Final score: {}", score);
                b.clear();
            }
        }
        bingo_boards.retain(|b| !b.is_empty());
        if bingo_boards.is_empty() {
            break;
        }
    }
}
