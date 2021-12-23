use std::collections::HashSet;
use std::fs;

type Space = Option<u8>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Burrow {
    hallway: [Space; 11],
    sides: [Vec<Space>; 4],
}

fn move_cost(n: usize, a: u8) -> u32 {
    n as u32 * u32::pow(10, a as u32)
}

fn add_to_hash(hash: &mut u64, space: Space) {
    *hash *= 5;
    if let Some(a) = space {
        *hash += 1 + a as u64;
    }
}

fn get_from_hash(hash: &mut u64) -> Space {
    let space = match *hash % 5 {
        0 => None,
        a => Some(a as u8 - 1),
    };
    *hash /= 5;
    space
}

fn step_to(from: usize, to: usize) -> usize {
    if from > to {
        from - 1
    } else {
        from + 1
    }
}

fn diff(from: usize, to: usize) -> usize {
    if from > to {
        from - to
    } else {
        to - from
    }
}

const LEGIT_HALLWAYS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

impl Burrow {
    fn get_moves(&self) -> Vec<(u64, u32)> {
        let mut poss = Vec::new();
        let mut hash_dummy = self.clone();
        for (i, s) in self.sides.iter().enumerate() {
            if s.iter().all(|&v| v.is_none() || v == Some(i as u8)) {
                continue;
            }
            for (is, a) in s.iter().enumerate() {
                if let Some(amphri) = a {
                    let front_ih = 2 + 2 * i;

                    hash_dummy.sides[i][is] = None;
                    for to in [0, 10] {
                        let mut ih = front_ih;
                        while ih != to {
                            ih = step_to(ih, to);
                            if self.hallway[ih].is_some() {
                                break;
                            }
                            if LEGIT_HALLWAYS.contains(&ih) {
                                hash_dummy.hallway[ih] = Some(*amphri);
                                poss.push((
                                    hash_dummy.hash(),
                                    move_cost(diff(ih, front_ih) + 1 + is, *amphri),
                                ));
                                hash_dummy.hallway[ih] = None;
                            }
                        }
                    }

                    hash_dummy.sides[i][is] = Some(*amphri);
                    break;
                }
            }
        }

        for (ih, a) in self.hallway.iter().enumerate() {
            if let Some(amphri) = a {
                let side = &self.sides[*amphri as usize];
                if !side.iter().all(|&v| v.is_none() || v == *a) {
                    continue;
                }
                let mut front_ih = 2 + 2 * (*amphri as usize);
                let path_length = diff(ih, front_ih);
                let mut free_path = true;
                while front_ih != ih {
                    if self.hallway[front_ih].is_some() {
                        free_path = false;
                        break;
                    }
                    front_ih = step_to(front_ih, ih);
                }
                if !free_path {
                    continue;
                }
                let mut deepest = 0;
                while deepest < side.len() - 1 && side[deepest + 1].is_none() {
                    deepest += 1;
                }
                hash_dummy.hallway[ih] = None;
                hash_dummy.sides[*amphri as usize][deepest] = Some(*amphri);
                poss.push((
                    hash_dummy.hash(),
                    move_cost(path_length + 1 + deepest, *amphri),
                ));
                hash_dummy.hallway[ih] = Some(*amphri);
                hash_dummy.sides[*amphri as usize][deepest] = None;
            }
        }
        poss
    }

    fn hash(&self) -> u64 {
        let mut h = 0;
        for hi in LEGIT_HALLWAYS {
            add_to_hash(&mut h, self.hallway[hi]);
        }
        for s in &self.sides {
            for ss in s {
                add_to_hash(&mut h, *ss);
            }
        }
        h
    }

    fn from_hash(hash: u64, side_size: usize) -> Self {
        let mut hash = hash;
        let mut sides: [Vec<Option<u8>>; 4] = Default::default();
        for i in (0..4).rev() {
            sides[i] = vec![None; side_size];
            for ii in 0..side_size {
                sides[i][side_size - ii - 1] = get_from_hash(&mut hash);
            }
        }
        let mut hallway = [None; 11];
        for hi in LEGIT_HALLWAYS.iter().rev() {
            hallway[*hi] = get_from_hash(&mut hash);
        }
        Burrow { hallway, sides }
    }
}

fn calc(burrow: &Burrow, side_size: usize) {
    let final_hash = Burrow {
        hallway: [None; 11],
        sides: [
            vec![Some(0); side_size],
            vec![Some(1); side_size],
            vec![Some(2); side_size],
            vec![Some(3); side_size],
        ],
    }
    .hash();

    let mut to_explore = vec![(burrow.hash(), 0)];
    let mut explored = HashSet::new();

    while let Some((hash, cost)) = to_explore.pop() {
        if hash == final_hash {
            println!("Final cost: {}", cost);
            break;
        }
        for (h, c) in Burrow::from_hash(hash, side_size).get_moves() {
            if explored.contains(&h) {
                continue;
            }
            let new_cost = cost + c;
            if let Some((_, entry_cost)) = to_explore.iter_mut().find(|(hh, _)| *hh == h) {
                *entry_cost = u32::min(*entry_cost, new_cost);
            } else {
                to_explore.push((h, new_cost));
            }
        }
        to_explore.sort_by(|(_, c1), (_, c2)| c2.cmp(c1));
        explored.insert(hash);
    }
    println!("{} positions explored", explored.len());
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut sides: [Vec<Option<u8>>; 4] = Default::default();
    for l in contents.lines() {
        let mut s = 0;
        for ch in l.chars() {
            match ch {
                c if ['A', 'B', 'C', 'D'].contains(&c) => {
                    sides[s].push(Some(c as u8 - 'A' as u8));
                    s += 1;
                }
                _ => {}
            }
        }
    }
    let side_size = sides[0].len();
    let hallway = [None; 11];
    let burrow1 = Burrow {
        hallway: hallway.clone(),
        sides: sides.clone(),
    };
    calc(&burrow1, side_size);

    sides[0].insert(1, Some(3));
    sides[0].insert(2, Some(3));
    sides[1].insert(1, Some(2));
    sides[1].insert(2, Some(1));
    sides[2].insert(1, Some(1));
    sides[2].insert(2, Some(0));
    sides[3].insert(1, Some(0));
    sides[3].insert(2, Some(2));
    let burrow2 = Burrow { hallway, sides };
    calc(&burrow2, side_size + 2);
}
