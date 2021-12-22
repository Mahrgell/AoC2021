use regex::Regex;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
struct Quader {
    xyz: [[i64; 2]; 3],
    subq: Vec<Quader>,
}

impl Quader {
    fn overlap(&self, other: &Self) -> Option<Self> {
        let mut xyz = [[0, 0]; 3];
        for i in 0..3 {
            let min = i64::max(self.xyz[i][0], other.xyz[i][0]);
            let max = i64::min(self.xyz[i][1], other.xyz[i][1]);
            if max < min {
                return None;
            }
            xyz[i] = [min, max];
        }
        Some(Quader {
            xyz,
            subq: Vec::new(),
        })
    }

    fn size(&self) -> i64 {
        let sz = (self.xyz[0][1] + 1 - self.xyz[0][0])
            * (self.xyz[1][1] + 1 - self.xyz[1][0])
            * (self.xyz[2][1] + 1 - self.xyz[2][0]);
        sz - self.subq.iter().fold(0, |a, q| a + q.size())
    }

    fn subtract(&mut self, other: &Self) {
        if let Some(overlap) = self.overlap(&other) {
            if overlap == *self {
                self.xyz[0] = [0, -1];
                return;
            }
            for sq in &mut self.subq {
                sq.subtract(&overlap);
            }
            self.subq.retain(|q| q.xyz[0] != [0, -1]);
            self.subq.push(overlap);
        }
    }
}

fn count_on_quads(quads: &Vec<(bool, Quader)>) -> i64 {
    let mut on_quads: Vec<Quader> = Vec::new();
    for (is_on, q) in quads {
        for on_q in &mut on_quads {
            on_q.subtract(q);
        }
        on_quads.retain(|q| q.xyz[0] != [0, -1]);
        if *is_on {
            on_quads.push(q.clone());
        }
    }
    on_quads.iter().fold(0, |a, q| a + q.size())
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    let mut quads = Vec::new();
    let mut init_quads = Vec::new();
    let minibounds = Quader {
        xyz: [[-50, 50], [-50, 50], [-50, 50]],
        subq: Vec::new(),
    };
    for l in contents.lines() {
        let cap = re.captures(&l).unwrap();
        let is_on = &cap[1] == "on";
        let minx = cap[2].parse::<i64>().unwrap();
        let maxx = cap[3].parse::<i64>().unwrap();
        let miny = cap[4].parse::<i64>().unwrap();
        let maxy = cap[5].parse::<i64>().unwrap();
        let minz = cap[6].parse::<i64>().unwrap();
        let maxz = cap[7].parse::<i64>().unwrap();
        let bq = (
            is_on,
            Quader {
                xyz: [[minx, maxx], [miny, maxy], [minz, maxz]],
                subq: Vec::new(),
            },
        );
        if minibounds.overlap(&bq.1).is_some() {
            init_quads.push(bq.clone());
        }
        quads.push(bq);
    }
    println!("Initialization result: {}", count_on_quads(&init_quads));
    println!("Full result: {}", count_on_quads(&quads));
}
