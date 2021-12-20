use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Vec3d {
    xyz: [i32; 3],
}

#[derive(Clone, Debug)]
struct Scanner {
    beacons: Vec<Vec3d>,
    center: Vec3d,
}

impl std::ops::Add for Vec3d {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3d {
            xyz: [
                self.xyz[0] + other.xyz[0],
                self.xyz[1] + other.xyz[1],
                self.xyz[2] + other.xyz[2],
            ],
        }
    }
}

impl std::ops::Sub for Vec3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3d {
            xyz: [
                self.xyz[0] - other.xyz[0],
                self.xyz[1] - other.xyz[1],
                self.xyz[2] - other.xyz[2],
            ],
        }
    }
}

impl Vec3d {
    fn max_axis_dist(&self, other: &Self) -> i32 {
        let mut d = 0;
        for i in 0..3 {
            d = i32::max(d, i32::abs(self.xyz[i] - other.xyz[i]));
        }
        d
    }

    fn manhatten_dist(&self, other: &Self) -> i32 {
        let mut d = 0;
        for i in 0..3 {
            d += i32::abs(self.xyz[i] - other.xyz[i]);
        }
        d
    }

    fn transform(&self, trafo: u32) -> Self {
        assert!(trafo < 24);
        let mut xyz = match trafo / 4 {
            0 => self.xyz.clone(),
            1 => [-self.xyz[0], self.xyz[2], self.xyz[1]],
            2 => [self.xyz[1], self.xyz[2], self.xyz[0]],
            3 => [-self.xyz[1], self.xyz[0], self.xyz[2]],
            4 => [self.xyz[2], self.xyz[0], self.xyz[1]],
            5 => [-self.xyz[2], self.xyz[1], self.xyz[0]],
            _ => panic!(),
        };
        match trafo % 4 {
            0 => {}
            1 => {
                xyz[1] = -xyz[1];
                xyz[2] = -xyz[2]
            }
            2 => {
                xyz[0] = -xyz[0];
                xyz[2] = -xyz[2]
            }
            3 => {
                xyz[0] = -xyz[0];
                xyz[1] = -xyz[1]
            }
            _ => panic!(),
        }
        Vec3d { xyz }
    }

    // finds the first possible trafo to transform self into target
    fn get_trafo(&self, target: &Self) -> Option<u32> {
        for trafo in 0..24 {
            if self.transform(trafo) == *target {
                return Some(trafo);
            }
        }
        None
    }

    // returns the the transformed self which is minimal with regards to PartialOrd
    fn get_min_rotated(&self) -> Self {
        let mut min_rot = *self;
        for t in 1..24 {
            let rot = self.transform(t);
            if rot < min_rot {
                min_rot = rot;
            }
        }
        min_rot
    }
}

impl Scanner {
    fn transform(&self, trafo: u32, offset: &Vec3d) -> Self {
        let mut scanner = self.clone();
        for b in &mut scanner.beacons {
            *b = b.transform(trafo) + *offset;
        }
        scanner.center = scanner.center.transform(trafo) + *offset;
        scanner
    }

    fn is_in_range(&self, b: &Vec3d) -> bool {
        self.center.max_axis_dist(b) < 1000
    }

    // counts the overlapping beacons between scanners
    // returns None if there are conflicts!
    fn overlaps(&self, other: &Self) -> Option<u32> {
        let mut overlaps = 0;
        for b in &self.beacons {
            if !other.is_in_range(b) {
                continue;
            }
            if other.beacons.contains(b) {
                overlaps += 1;
            } else {
                return None;
            }
        }
        for b in &other.beacons {
            if self.is_in_range(b) && !self.beacons.contains(b) {
                return None;
            }
        }
        Some(overlaps)
    }
}

fn main() {
    // read input
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut scanners = Vec::new();
    for l in contents.lines() {
        if l == "" {
            continue;
        } else if l.contains("scanner") {
            scanners.push(Scanner {
                beacons: Vec::new(),
                center: Vec3d { xyz: [0, 0, 0] },
            });
        } else {
            let xyz: Vec<_> = l.split(',').map(|w| w.parse::<i32>().unwrap()).collect();
            scanners.last_mut().unwrap().beacons.push(Vec3d {
                xyz: [xyz[0], xyz[1], xyz[2]],
            });
        }
    }
    // write down all delta between beacons so we can match them later
    let mut delta_buoy = HashMap::new();
    for (si, s) in scanners.iter().enumerate() {
        for bi1 in 0..s.beacons.len() - 1 {
            for bi2 in (bi1 + 1)..s.beacons.len() {
                let d = s.beacons[bi1] - s.beacons[bi2];
                let d = d.get_min_rotated();
                delta_buoy
                    .entry(d)
                    .or_insert(Vec::new())
                    .push((si, (bi1, bi2)));
            }
        }
    }
    // go through those delta beacons to check if we can deduce an overlap from those
    let mut completed_scanners = vec![0];
    loop {
        delta_buoy.retain(|_, v| v.len() > 1);
        let mut scanner_added = false;
        for (_, v) in &mut delta_buoy {
            let aligned: Vec<_> = v
                .iter()
                .filter(|(s, (_, _))| completed_scanners.contains(s))
                .collect();
            let unaligned: Vec<_> = v
                .iter()
                .filter(|(s, (_, _))| !completed_scanners.contains(s))
                .collect();
            for (a_si, (a_bi1, a_bi2)) in &aligned {
                let a_b1 = scanners[*a_si].beacons[*a_bi1];
                let a_b2 = scanners[*a_si].beacons[*a_bi2];
                for (u_si, (u_bi1, u_bi2)) in &unaligned {
                    if completed_scanners.contains(u_si) {
                        continue;
                    }
                    let d_beacons1 = a_b2 - a_b1;
                    for (bi1, bi2) in [(*u_bi1, *u_bi2), (*u_bi2, *u_bi1)] {
                        let u_b1 = scanners[*u_si].beacons[bi1];
                        let u_b2 = scanners[*u_si].beacons[bi2];
                        let d_beacons2 = u_b2 - u_b1;
                        if let Some(trafo) = d_beacons2.get_trafo(&d_beacons1) {
                            let offset = a_b1 - u_b1.transform(trafo);
                            let new_scanner = scanners[*u_si].transform(trafo, &offset);
                            let mut is_matching = true;
                            let mut found_12_match = false;
                            for compl_si in &completed_scanners {
                                match scanners[*compl_si].overlaps(&new_scanner) {
                                    Some(m) if m >= 12 => found_12_match = true,
                                    Some(_) => {}
                                    None => {
                                        is_matching = false;
                                        break;
                                    }
                                }
                            }
                            if is_matching && found_12_match {
                                scanners[*u_si] = new_scanner;
                                completed_scanners.push(*u_si);
                                println!("Adding scanner {}", *u_si);
                                scanner_added = true;
                                break;
                            }
                        }
                    }
                }
            }
            v.retain(|(s, (_, _))| !completed_scanners.contains(s));
        }
        if !scanner_added || scanners.len() == completed_scanners.len() {
            break;
        }
    }
    println!("-------------------------------------------");
    println!("{} scanners aligned", completed_scanners.len());
    let mut beacons = Vec::new();
    for s in &scanners {
        for b in &s.beacons {
            if !beacons.contains(b) {
                beacons.push(*b);
            }
        }
    }
    println!("{} beacons", beacons.len());
    let mut max_dist = 0;
    for si1 in 0..scanners.len() - 1 {
        for si2 in (si1 + 1)..scanners.len() {
            max_dist = i32::max(
                max_dist,
                scanners[si1].center.manhatten_dist(&scanners[si2].center),
            );
        }
    }
    println!("Max manhatten distance: {}", max_dist);
}
