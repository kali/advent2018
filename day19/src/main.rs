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
    fn run(&self, regs:&mut [isize; 6], breakpoints: &[usize]) {
        loop {
            let pc = regs[self.ip];
            if breakpoints.contains(&(pc as usize)) {
                return
            }
            if pc < 0 || pc as usize >= self.ops.len() {
                break;
            }
            let op = self.ops[pc as usize];
            op.0.apply(&op.1, regs);
            regs[self.ip] += 1;
        }
    }
}

fn run(s:&str) -> (isize, isize) {
    let prog = Program::parse(s);
    let mut regs = [0; 6];
    prog.run(&mut regs, &[]);
    let a = regs[0];

    let mut regs = [1, 0, 0, 0, 0, 0];
    prog.run(&mut regs, &[1]);
    println!("r5 (1) = {}", regs[5]);
    let b = rev_ing_loops(regs[5] as usize) as isize;

    (a,b)
}

fn rev_ing_loops(r5: usize) -> usize {
    let mut r0 = 0;
    for i in 1..=r5 {
        if r5 % i == 0 {
            r0 += i;
        }
    }
    r0
}

fn main() {
    let p = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&p));
}

#[test]
fn t_1() {
    let p = "
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";
    assert_eq!(run(p).0, 7);
}
