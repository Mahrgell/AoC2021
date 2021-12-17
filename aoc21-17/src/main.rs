use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    // assumption: x always positive, y always negative for all inputs
    let re = Regex::new(r"^target area: x=(\d+)..(\d+), y=-(\d+)..-(\d+)").unwrap();
    let cap = re.captures(&contents).unwrap();
    let target_minx = cap[1].parse::<i32>().unwrap();
    let target_maxx = cap[2].parse::<i32>().unwrap();
    let target_miny = -cap[3].parse::<i32>().unwrap();
    let target_maxy = -cap[4].parse::<i32>().unwrap();
    // max height is reached for a shot with dy = -target_miny at t = - target_miny
    // this shot will be at target_miny at t = 2 * -target_miny turns + 1
    println!("Max height: {}", -target_miny * (-target_miny - 1) / 2);
    // compute dx for which all shots long enough travelling will end in target x zone
    // simple assumption (verified for my input only..): there is only one such speed!
    let cutoff_dx_t = (((target_maxx * 2) as f64).sqrt() + 0.5) as i32;
    let mut solutions = Vec::new();
    // go by all possible number of turns to end up in the zone
    for t in 1..=(-2 * target_miny) {
        let (min_dx, max_dx) = if t >= cutoff_dx_t {
            (cutoff_dx_t, cutoff_dx_t)
        } else {
            let min = target_minx as f64 / t as f64 + (t + 1) as f64 / 2.;
            let max = target_maxx as f64 / t as f64 + (t + 1) as f64 / 2.;
            (f64::ceil(min) as i32, f64::floor(max) as i32)
        };
        let min_dy = f64::ceil(target_miny as f64 / t as f64 + (t + 1) as f64 / 2.) as i32;
        let max_dy = f64::floor(target_maxy as f64 / t as f64 + (t + 1) as f64 / 2.) as i32;
        for x in min_dx..=max_dx {
            for y in min_dy..=max_dy {
                solutions.push((x, y));
            }
        }
    }
    solutions.sort();
    solutions.dedup();
    println!("{}", solutions.len());
}
