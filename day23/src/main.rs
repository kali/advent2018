#[derive(Debug)]
struct Bot {
    x: isize,
    y: isize,
    z: isize,
    r: usize,
}

impl Bot {
    fn parse(input: &str) -> Bot {
        let mut tokens = input.split(" ");
        let pos = tokens.next().unwrap();
        let r = tokens.next().unwrap()[2..].parse::<usize>().unwrap();
        let pos = &pos[5..];
        let pos = &pos[..pos.len() - 2];
        let mut values = pos.split(",").map(|n| n.parse::<isize>().unwrap());
        let x = values.next().unwrap();
        let y = values.next().unwrap();
        let z = values.next().unwrap();
        Bot { x, y, z, r }
    }

    fn dist(&self, other: &Bot) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

fn run1(bots: &[Bot]) -> usize {
    let strongest = bots.iter().max_by_key(|b| b.r).unwrap();
    bots.iter()
        .filter(|b| b.dist(strongest) <= strongest.r)
        .count()
}

#[derive(Copy, Clone, Debug)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
    side: usize,
    max: usize,
}

impl Cube {
    fn new(x: isize, y: isize, z: isize, side: isize, bots: &[Bot]) -> Cube {
        let max = bots
            .iter()
            .filter(|bot| {
                let min_x_dist = if bot.x < x {
                    x - bot.x
                } else if bot.x > x + side {
                    bot.x - x - side
                } else {
                    0
                };
                let min_y_dist = if bot.y < y {
                    y - bot.y
                } else if bot.y > y + side {
                    bot.y - y - side
                } else {
                    0
                };
                let min_z_dist = if bot.z < z {
                    z - bot.z
                } else if bot.z > z + side {
                    bot.z - z - side
                } else {
                    0
                };
                min_x_dist + min_y_dist + min_z_dist <= bot.r as isize
            })
            .count();
        Cube {
            x,
            y,
            z,
            side: side as usize,
            max,
        }
    }
    fn split(&self, bots: &[Bot]) -> Vec<Cube> {
        let seg = 10isize.min(self.side as isize);
        (0..seg.pow(3))
            .map(|n| (n / seg / seg, n / seg % seg, n % seg))
            .map(|(dx, dy, dz)| {
                let side = self.side as isize / seg;
                let x = self.x + dx * side;
                let y = self.y + dy * side;
                let z = self.z + dz * side;
                Cube::new(x, y, z, side, bots)
            })
            .collect()
    }
}

fn run2(bots: &[Bot]) -> usize {
    let x0 = bots.iter().map(|b| b.x).min().unwrap();
    let y0 = bots.iter().map(|b| b.y).min().unwrap();
    let z0 = bots.iter().map(|b| b.z).min().unwrap();
    let x1 = bots.iter().map(|b| b.x).max().unwrap();
    let y1 = bots.iter().map(|b| b.y).max().unwrap();
    let z1 = bots.iter().map(|b| b.z).max().unwrap();
    let side = (x1 - x0).max(y1 - y0).max(z1 - z0);
    println!("(x,y,z)0: {:?} side: {}", (x0, y0, z0), side);
    // (x0,y0,z0,side,at_most)
    let mut cubes = vec![Cube::new(x0, y0, z0, side, bots)];
    loop {
        let best_corner = cubes
            .iter()
            .map(|c| {
                bots.iter()
                    .filter(|b| {
                        ((b.x - c.x).abs() + (b.y - c.y).abs() + (b.z - c.z).abs()) <= b.r as isize
                    })
                    .count()
            })
            .max()
            .unwrap();
        println!("best {:?}", best_corner);
        cubes.retain(|c| c.max >= best_corner);
        println!("retained {:?}", cubes.len());
        if let Some((best_cube, _)) = cubes
            .iter()
            .enumerate()
            .filter(|c| c.1.side > 1)
            .max_by_key(|(ix, c)| c.max) {
                let best_cube = cubes.swap_remove(best_cube);
                println!("split: {:?}", best_cube);
                cubes.extend(best_cube.split(&bots).into_iter());
                println!("cubes: {:?}", cubes.len());
            } else {
                break;
            }
    }
    let best_cube = cubes
        .iter()
        .map(|c| {
            let score = bots
                .iter()
                .filter(|b| {
                    ((b.x - c.x).abs() + (b.y - c.y).abs() + (b.z - c.z).abs()) <= b.r as isize
                })
                .count();
            let dist = c.x.abs() + c.y.abs() + c.z.abs();
            (c, score, dist)
        })
        .max_by_key(|s| (s.1, -s.2))
        .unwrap();
    best_cube.2 as usize
}

fn run(input: &str) -> (usize, usize) {
    let bots: Vec<_> = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(Bot::parse)
        .collect();
    (run1(&bots), run2(&bots))
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&input));
}

#[test]
fn t1() {
    let input = r#"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"#;
    assert_eq!(run(input).0, 7);
}

#[test]
fn t2() {
    let input = r#"
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
"#;
    assert_eq!(run(input).1, 36);
}
