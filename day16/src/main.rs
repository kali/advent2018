use std::collections::HashMap;

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
    fn all() -> &'static [Op] {
        &[
            Op::AddR,
            Op::AddI,
            Op::MulR,
            Op::MulI,
            Op::BanR,
            Op::BanI,
            Op::BorR,
            Op::BorI,
            Op::SetR,
            Op::SetI,
            Op::GtIR,
            Op::GtRI,
            Op::GtRR,
            Op::EqIR,
            Op::EqRI,
            Op::EqRR,
        ]
    }
    fn apply(&self, arg: &[isize], regs: &mut [isize; 4]) {
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
}

#[derive(Debug)]
struct Sample {
    before: [isize; 4],
    instruction: [isize; 4],
    after: [isize; 4],
}

impl Sample {
    pub fn parse(s: &str) -> Sample {
        let mut lines = s.split("\n");
        let before = lines.next().unwrap();
        let before_vec: Vec<isize> = before[9..]
            .trim_right_matches("]")
            .split(", ")
            .map(|t| t.parse::<isize>().unwrap())
            .collect();
        let mut before = [0, 0, 0, 0];
        before.copy_from_slice(&before_vec[0..4]);
        let instruction_vec: Vec<isize> = lines
            .next()
            .unwrap()
            .split(" ")
            .map(|t| t.parse::<isize>().unwrap())
            .collect();
        let mut instruction = [0, 0, 0, 0];
        instruction.copy_from_slice(&instruction_vec[0..4]);
        let after = lines.next().unwrap();
        let after_vec: Vec<isize> = after[9..]
            .trim_right_matches("]")
            .split(", ")
            .map(|t| t.parse::<isize>().unwrap())
            .collect();
        let mut after = [0, 0, 0, 0];
        after.copy_from_slice(&after_vec[0..4]);
        Sample {
            before,
            instruction,
            after,
        }
    }

    pub fn probe_ops(&self, ops: impl Iterator<Item=Op>) -> Vec<Op> {
        ops.filter(|op| {
            let mut regs = self.before.clone();
            op.apply(&self.instruction[1..4] as _, &mut regs);
            regs == self.after
        })
        .collect()
    }

    pub fn opcode(&self) -> usize {
        self.instruction[0] as usize
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut parts = input.split("\n\n\n");
    let samples:Vec<Sample> = parts.next().unwrap().split("\n\n").map(|s| Sample::parse(s.trim())).collect();
    println!("{}", samples.iter().filter(|sam| sam.probe_ops(Op::all().iter().cloned()).len() >= 3).count());
    let mut to_map = Op::all().to_vec();
    let mut mapped:HashMap<usize, Op> = HashMap::new();
    for sample in samples.iter().cycle() {
        if mapped.contains_key(&sample.opcode()) {
            continue;
        }
        let candidates = sample.probe_ops(to_map.iter().cloned());
        if candidates.len() == 1 {
            let op = candidates[0];
            to_map.retain(|&tm| tm != op);
            mapped.insert(sample.opcode(), op);
        }
        if to_map.len() == 0 {
            break;
        }
    }
    println!("{:?}", mapped);
    let prog = parts.next().unwrap();
    let mut reg = [0; 4];
    for inst in prog.split("\n").filter(|l| l.len() > 0) {
        let mut tokens = inst.split(" ").map(|s| s.parse::<isize>().unwrap());
        let opcode = tokens.next().unwrap() as usize;
        let op = mapped[&opcode];
        let args = [ tokens.next().unwrap(), tokens.next().unwrap(), tokens.next().unwrap() ];
        op.apply(&args, &mut reg);
    }
    println!("{}", reg[0]);
}

#[test]
fn t_1() {
    let s = Sample {
        before: [3, 2, 1, 1],
        instruction: [9, 2, 1, 2],
        after: [3, 2, 2, 1],
    };
    assert_eq!(s.probe_op().len(), 3);
}
