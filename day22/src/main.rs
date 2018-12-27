use pathfinding::directed::dijkstra::dijkstra;

fn main() {
    let depth = 4080;
    let target = (14, 785);
    println!("{:?}", run(depth, target.0, target.1));
}

#[derive(Default)]
struct Map {
    depth: usize,
    x_target: usize,
    y_target: usize,
    levels: Vec<Vec<usize>>,
}

impl Map {
    fn new(depth: usize, x_target:usize, y_target: usize) -> Map {
        Map {
            depth,
            x_target,
            y_target,
            levels: vec![],
        }
    }
    fn build(&mut self, up_to_x:usize, up_to_y:usize) {
        let mut levels = vec![vec![0; up_to_x + 1]; up_to_y + 1];
        for y in 0..=up_to_y {
            for x in 0..=up_to_x {
                let index = if (x == 0 && y == 0) || (x == self.x_target && y == self.y_target) {
                    0
                } else if y == 0 {
                    16807 * x
                } else if x == 0 {
                    48271 * y
                } else {
                    levels[y][x - 1] * levels[y - 1][x]
                };
                levels[y][x] = (index + self.depth) % 20183;
            }
        }
        self.levels = levels;
    }
    fn level_at(&mut self, x: usize, y: usize) -> usize {
        if y >= self.levels.len() || x >= self.levels[0].len() {
            self.build(x.max(self.levels[0].len()), y.max(self.levels.len()));
        }
        self.levels[y][x]
    }
    fn type_at(&mut self, x: usize, y: usize) -> u8 {
        [b'.', b'=', b'|'][self.level_at(x, y) % 3]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Gear {
    None,
    Climb,
    Torch,
}

fn run(depth: usize, x: usize, y: usize) -> (usize, usize) {
    let mut map = Map::new(depth, x, y);
    map.build(x, y);
    let part1 = map
        .levels
        .iter()
        .take(11)
        .map(|row| row.iter().take(11).map(|c| c % 3).sum::<usize>())
        .sum::<usize>();
    let path = dijkstra(
        &(0, 0, Gear::Torch),
        |&(x, y, gear)| {
            let here = map.type_at(x, y);
//            println!("From: {:?}", (x, y, gear));
            fn compatible(typ: u8, gear: Gear) -> bool {
                match typ {
                    b'.' => gear != Gear::None,
                    b'|' => gear != Gear::Climb,
                    b'=' => gear != Gear::Torch,
                    _ => unreachable!(),
                }
            }
            let mut moves: Vec<((usize, usize, Gear), usize)> =
                [Gear::None, Gear::Climb, Gear::Torch]
                    .iter()
                    .filter(|&g| compatible(here, *g))
                    .map(|g| ((x, y, *g), 7))
                    .collect();
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter(|&(dx, dy)| x as isize + *dx >= 0 && y as isize + *dy as isize >= 0)
                .map(|(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
                .filter(|&(x, y)| {
//                    println!("   xy: {},{} -> {}", x, y, map.type_at(x, y) as char);
                    compatible(map.type_at(x, y), gear)
                })
                .for_each(|(x, y)| moves.push(((x, y, gear), 1)));
//            println!("   -> {:?}", moves);
            moves.into_iter()
        },
        |&(x1, y1, gear)| gear == Gear::Torch && x == x1 && y == y1,
    );
//    println!("{:#?}", path);
    (part1, path.unwrap().1)
}

#[test]
fn t1() {
    assert_eq!(run(510, 10, 10), (114, 45));
}
