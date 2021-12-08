use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut cnt1478 = 0;
    let mut total_sum = 0;
    for l in contents.lines() {
        let words: Vec<_> = l.split(" | ").collect();
        let outputs: Vec<_> = words[1].split(' ').collect();
        cnt1478 += outputs
            .iter()
            .filter(|w| [2, 3, 4, 7].contains(&w.len()))
            .count();
        // task 1 done, now the interesting part...
        let outputs: Vec<_> = outputs
            .iter()
            .map(|&w| w.chars().sorted().collect::<String>())
            .collect();
        let inputs: Vec<_> = words[0].split(' ').collect();
        let inputs: Vec<_> = inputs
            .iter()
            .map(|&w| w.chars().sorted().collect::<String>())
            .collect();
        let mut nb: [String; 10] = Default::default();
        // identifiable by segment number
        nb[1] = inputs.iter().find(|w| w.len() == 2).unwrap().clone();
        nb[7] = inputs.iter().find(|w| w.len() == 3).unwrap().clone();
        nb[4] = inputs.iter().find(|w| w.len() == 4).unwrap().clone();
        nb[8] = inputs.iter().find(|w| w.len() == 7).unwrap().clone();
        // 0, 6 and 9 have one segment turned off
        let inputs_len6: Vec<_> = inputs.iter().filter(|w| w.len() == 6).collect();
        // c and f segment are in 1, but 0, 6, 9 all contain f, but the 6 misses c
        for c in nb[1].chars() {
            if let Some(nb6) = inputs_len6.iter().find(|&w| !w.contains(c)) {
                nb[6] = nb6.to_string();
                break;
            }
        }
        // b and d segment are in 4 but not 1, but 0, 6, 9 all contain b, but the 0 misses d
        for c in nb[4].chars() {
            if nb[1].contains(c) {
                continue;
            }
            if let Some(nb0) = inputs_len6.iter().find(|&w| !w.contains(c)) {
                nb[0] = nb0.to_string();
                break;
            }
        }
        // the remaining one of the len6 must be the 9
        nb[9] = inputs_len6
            .iter()
            .find(|&&w| w != &nb[6] && w != &nb[0])
            .unwrap()
            .to_string();

        let inputs_len5: Vec<_> = inputs.iter().filter(|w| w.len() == 5).collect();
        for c in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
            // c segment is the only one missing in 6, only 5 misses it
            if !nb[6].contains(c) {
                nb[5] = inputs_len5
                    .iter()
                    .find(|&w| !w.contains(c))
                    .unwrap()
                    .to_string();
            }
            // if it isn't c but in 1, it must be f -> only 2 misses it
            else if nb[1].contains(c) {
                nb[2] = inputs_len5
                    .iter()
                    .find(|&w| !w.contains(c))
                    .unwrap()
                    .to_string();
            }
        }
        // the remaining one of the len5 must be the 3
        nb[3] = inputs_len5
            .iter()
            .find(|&&w| w != &nb[5] && w != &nb[2])
            .unwrap()
            .to_string();
        let mut output_result = 0;
        for o in outputs {
            let val = nb.iter().position(|w| w == &o).unwrap();
            output_result *= 10;
            output_result += val;
        }
        total_sum += output_result;
    }
    println!("Unique numbers appear {} times.", cnt1478);
    println!("The sum of all outputs is {}.", total_sum);
}
