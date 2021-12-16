use std::fs;

struct Packet {
    version: u64,
    type_id: u64,
    value: Option<u64>,
    subp: Vec<Packet>,
}

fn read_nb(input: &Vec<u8>, pos: &mut usize, length: usize) -> u64 {
    let mut nb = 0;
    for _ in 0..length {
        nb = nb * 2 + input[*pos] as u64;
        *pos += 1;
    }
    nb
}

fn read_literal(input: &Vec<u8>, pos: &mut usize) -> u64 {
    let mut nb = 0;
    let mut nb_end = false;
    while !nb_end {
        nb_end = 0 == read_nb(input, pos, 1);
        nb = nb * 16 + read_nb(input, pos, 4)
    }
    nb
}

impl Packet {
    fn read(input: &Vec<u8>, pos: &mut usize) -> Packet {
        let version = read_nb(input, pos, 3);
        let type_id = read_nb(input, pos, 3);
        let mut value = None;
        let mut subp = Vec::new();
        if type_id == 4 {
            value = Some(read_literal(input, pos));
        } else {
            if read_nb(input, pos, 1) == 0 {
                let bitlen = read_nb(input, pos, 15);
                let target = *pos + bitlen as usize;
                while *pos < target {
                    subp.push(Packet::read(input, pos));
                }
            } else {
                let nb_subp = read_nb(input, pos, 11);
                for _ in 0..nb_subp {
                    subp.push(Packet::read(input, pos));
                }
            }
        }
        Packet {
            version,
            type_id,
            value,
            subp,
        }
    }

    fn version_sum(&self) -> u64 {
        self.subp
            .iter()
            .fold(self.version, |a, p| a + p.version_sum())
    }

    fn value(&self) -> u64 {
        match self.type_id {
            0 => self.subp.iter().fold(0, |a, p| a + p.value()),
            1 => self.subp.iter().fold(1, |a, p| a * p.value()),
            2 => self
                .subp
                .iter()
                .fold(u64::MAX, |a, p| u64::min(a, p.value())),
            3 => self.subp.iter().fold(0, |a, p| u64::max(a, p.value())),
            4 => self.value.unwrap(),
            5 => (self.subp[0].value() > self.subp[1].value()) as u64,
            6 => (self.subp[0].value() < self.subp[1].value()) as u64,
            7 => (self.subp[0].value() == self.subp[1].value()) as u64,
            _ => panic!(),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let input: Vec<_> = contents
        .chars()
        .filter(|c| ('0'..='9').contains(c) || ('A'..='F').contains(c))
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let p = Packet::read(&input, &mut 0);
    println!("Version sum: {}", p.version_sum());
    println!("Value: {}", p.value());
}
