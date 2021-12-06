use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let ages: Vec<_> = contents
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    let mut fish = [0u64; 9];
    for n in 0..9 {
        fish[n] = ages.iter().filter(|&&v| v == n).count() as u64;
    }
    for day in 1..=256 {
        let fish0 = fish[0];
        for age in 0..8 {
            fish[age] = fish[age + 1];
        }
        fish[8] = fish0;
        fish[6] += fish0;
        if day == 80 || day == 256 {
            let nb_fish = fish.iter().fold(0, |acc, v| acc + v);
            println!("{} fish after {} days.", nb_fish, day);
        }
    }
}
