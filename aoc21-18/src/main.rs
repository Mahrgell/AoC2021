use std::fs;

#[derive(Clone)]
enum SN {
    Pair([Box<SN>; 2]),
    Single(u32),
}

impl SN {
    fn read(ch: &Vec<char>, pos: &mut usize) -> SN {
        if ch[*pos] == '[' {
            *pos += 1;
            //initialize with dummies...
            let mut pair: [Box<SN>; 2] = [Box::new(SN::Single(0)), Box::new(SN::Single(0))];
            pair[0] = Box::new(SN::read(ch, pos));
            assert!(ch[*pos] == ',');
            *pos += 1;
            pair[1] = Box::new(SN::read(ch, pos));
            assert!(ch[*pos] == ']');
            *pos += 1;
            SN::Pair(pair)
        } else {
            let val = ch[*pos].to_digit(10).unwrap();
            *pos += 1;
            SN::Single(val)
        }
    }

    fn extract_single(&self) -> u32 {
        if let SN::Single(v) = self {
            *v
        } else {
            panic!()
        }
    }

    fn add(sn1: &SN, sn2: &SN) -> SN {
        let mut sum = SN::Pair([Box::new(sn1.clone()), Box::new(sn2.clone())]);
        sum.reduce();
        sum
    }

    fn reduce(&mut self) {
        loop {
            if let (true, _) = self.explode(3) {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self, depth: u32) -> (bool, [u32; 2]) {
        if let SN::Pair(vv) = self {
            if depth > 0 {
                for side in [0usize, 1] {
                    let (b, mut rem) = vv[side].explode(depth - 1);
                    if b {
                        vv[1 - side].promote(rem[1 - side], side);
                        rem[1 - side] = 0;
                        return (true, rem);
                    }
                }
            } else {
                for side in [0usize, 1] {
                    if let SN::Pair(vvv) = &*vv[side] {
                        let mut rem = [vvv[0].extract_single(), vvv[1].extract_single()];
                        vv[1 - side].promote(rem[1 - side], side);
                        vv[side] = Box::new(SN::Single(0));
                        rem[1 - side] = 0;
                        return (true, rem);
                    }
                }
            }
        }
        (false, [0, 0])
    }

    fn promote(&mut self, val: u32, side: usize) {
        if val != 0 {
            match self {
                SN::Single(v) => *v += val,
                SN::Pair(vv) => vv[side].promote(val, side),
            };
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SN::Single(v) if *v < 10 => false,
            SN::Single(v) => {
                let vhalf = *v / 2;
                *self = SN::Pair([
                    Box::new(SN::Single(vhalf)),
                    Box::new(SN::Single(*v - vhalf)),
                ]);
                true
            }
            SN::Pair(vv) => vv[0].split() || vv[1].split(),
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            SN::Single(v) => *v as u64,
            SN::Pair(vv) => 3 * vv[0].magnitude() + 2 * vv[1].magnitude(),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut snail_nbs = Vec::new();
    for l in contents.lines() {
        let ch: Vec<_> = l.chars().collect();
        snail_nbs.push(SN::read(&ch, &mut 0));
    }
    let mut sum = snail_nbs[0].clone();
    for i in 1..snail_nbs.len() {
        sum = SN::add(&sum, &snail_nbs[i]);
    }
    println!("Full list sum magnitude: {}", sum.magnitude());

    let mut max = 0;
    // ostrich algorithm, this could find a maximum of nb^2
    for s1 in &snail_nbs {
        for s2 in &snail_nbs {
            let sum = SN::add(s1, s2).magnitude();
            if sum > max {
                max = sum;
            }
        }
    }
    println!("max sum magnitude: {}", max);
}
