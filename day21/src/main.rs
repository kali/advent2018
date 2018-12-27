// use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Op {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl Op {
    fn apply(&self, arg: &[isize], regs: &mut [isize; 6]) {
        match self {
            Op::AddR => regs[arg[2] as usize] = regs[arg[0] as usize] + regs[arg[1] as usize],
            Op::AddI => regs[arg[2] as usize] = regs[arg[0] as usize] + arg[1],
            Op::MulR => regs[arg[2] as usize] = regs[arg[0] as usize] * regs[arg[1] as usize],
            Op::MulI => regs[arg[2] as usize] = regs[arg[0] as usize] * arg[1],
            Op::BanR => regs[arg[2] as usize] = regs[arg[0] as usize] & regs[arg[1] as usize],
            Op::BanI => regs[arg[2] as usize] = regs[arg[0] as usize] & arg[1],
            Op::BorR => regs[arg[2] as usize] = regs[arg[0] as usize] | regs[arg[1] as usize],
            Op::BorI => regs[arg[2] as usize] = regs[arg[0] as usize] | arg[1],
            Op::SetR => regs[arg[2] as usize] = regs[arg[0] as usize],
            Op::SetI => regs[arg[2] as usize] = arg[0],
            Op::GtRR => {
                regs[arg[2] as usize] = (regs[arg[0] as usize] > regs[arg[1] as usize]) as isize
            }
            Op::GtRI => regs[arg[2] as usize] = (regs[arg[0] as usize] > arg[1]) as isize,
            Op::GtIR => regs[arg[2] as usize] = (arg[0] > regs[arg[1] as usize]) as isize,
            Op::EqRR => {
                regs[arg[2] as usize] = (regs[arg[0] as usize] == regs[arg[1] as usize]) as isize
            }
            Op::EqRI => regs[arg[2] as usize] = (regs[arg[0] as usize] == arg[1]) as isize,
            Op::EqIR => regs[arg[2] as usize] = (arg[0] == regs[arg[1] as usize]) as isize,
        }
    }
    fn parse(l: &str) -> (Op, [isize;3]) {
        let mut tokens = l.split(" ");
        let op = match tokens.next().unwrap() {
            "addr" => Op::AddR,
            "addi" => Op::AddI,
            "mulr" => Op::MulR,
            "muli" => Op::MulI,
            "banr" => Op::BanR,
            "bani" => Op::BanI,
            "borr" => Op::BorR,
            "bori" => Op::BorI,
            "setr" => Op::SetR,
            "seti" => Op::SetI,
            "gtir" => Op::GtIR,
            "gtri" => Op::GtRI,
            "gtrr" => Op::GtRR,
            "eqir" => Op::EqIR,
            "eqri" => Op::EqRI,
            "eqrr" => Op::EqRR,
            _ => panic!()
        };
        let mut numbers = tokens.map(|s| s.parse::<isize>().unwrap());
        (op, [numbers.next().unwrap(),numbers.next().unwrap(),numbers.next().unwrap()])
    }
}

#[derive(Debug)]
struct Program {
    ip: usize,
    ops: Vec<(Op, [isize; 3])>,
}

impl Program {
    fn parse(s:&str) -> Program {
        let mut ip = 0;
        let mut ops = vec!();
        for line in s.split("\n").filter(|l| l.len() > 0) {
            if line.starts_with("#ip ") {
                ip = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
            } else {
                ops.push(Op::parse(line));
            }
        }
        Program { ip, ops }
    }
    fn run(&self, regs:&mut [isize; 6], breakpoints: &[isize]) {
        loop {
            let pc = regs[self.ip];
            if [ 29].contains(&pc) {
                println!("{} {}", regs[1], regs[4]);
            }
            if pc < 0 || pc as usize >= self.ops.len() {
                break;
            }
            let op = self.ops[pc as usize];
            op.0.apply(&op.1, regs);
            regs[self.ip] += 1;
            if breakpoints.contains(&regs[self.ip]) {
                break;
            }
        }
    }
}

fn rev_ing() -> isize {
    use std::collections::{HashSet, HashMap};
    let mut states = HashSet::new();
    let mut first_seen_r4:HashMap<isize, isize> = HashMap::new();
    let mut r1 = 0isize;
    let mut r4 = 0isize;
    let mut i = 0isize;
    loop {
        r1 = r4 | 0x10000;
        r4 = 2024736;
        loop {
            r4 += r1 & 255;
            r4 &= 16777215;
            r4 *= 65899;
            r4 &= 16777215;
            if r1 < 256 {
                break;
            }
            r1 = (0..).find(|n| (n+1) * 256 > r1).unwrap();
        }
        println!("{} {}", r1, r4);
        if !first_seen_r4.contains_key(&r4) {
            first_seen_r4.insert(r4, i);
        }
        if states.contains(&(r1, r4)) {
            break;
        }
        states.insert((r1, r4));
        i += 1;
    }
    *first_seen_r4.iter().max_by_key(|&(&a,b)| (b, -a)).unwrap().0
}

fn run(s:&str) -> (isize, isize) {
/*
    let prog = Program::parse(s);
    let mut regs = [0; 6];
    loop {
        prog.run(&mut regs, &[29]);
    }
    (regs[4], 0)
    */
    (0, rev_ing())
}

fn main() {
    let p = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&p));
}
