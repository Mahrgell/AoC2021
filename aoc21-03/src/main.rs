use std::fs;
use std::cmp::Ordering;

fn filter_list(nums: &Vec<u64>, keep_most_frequent: bool, max_check: u64) -> u64 {
    let mut lnums = nums.clone();
    let mut check = max_check;
    while lnums.len() > 1 {
        let occ = lnums.iter().filter(|&n| (n&check) != 0).count();
        let mut cmp_val = match lnums.len().cmp(&(2*occ as usize)) {
            Ordering::Greater => 0,
            _ => check
        };
        if !keep_most_frequent {
            cmp_val = check - cmp_val;
        }
        lnums = lnums.into_iter().filter(|&n| (n&check) == cmp_val).collect();
        check /= 2;
    }
    lnums[0]
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let nums : Vec<_> = contents.lines().map(|l| u64::from_str_radix(l, 2).unwrap()).collect();
    let mut gamma = 0u64;
    let mut epsilon = 0u64;
    let mut check = 1;
    let max_num = nums.iter().max().unwrap();
    while &check < max_num {
        let occ = nums.iter().filter(|&n| (n&check) != 0).count();
        match nums.len().cmp(&(2*occ as usize)) {
            Ordering::Less => gamma += check,
            Ordering::Equal => {gamma += check; epsilon += check;},
            Ordering::Greater => {epsilon += check;},
        }
        check *= 2;
    }
    println!("gamma = {}, eps= {}, prod = {}", gamma, epsilon, gamma*epsilon);

    let oxy = filter_list(&nums, true, check/2);
    let co2 = filter_list(&nums, false, check/2);
    println!("oxy {}, co2 {} , prod {}", oxy, co2, oxy*co2);
}