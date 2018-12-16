extern crate pathfinding;

use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Unit {
    x: usize,
    y: usize,
    elf: bool,
    hp: usize,
    ap: usize,
}

#[derive(Clone, PartialEq)]
struct State {
    walls: Vec<Vec<bool>>,
    units: Vec<Unit>,
}

impl State {
    fn parse(s: &str, ap_elf: usize, ap_gob: usize) -> State {
        let mut walls = vec![];
        let mut units = vec![];
        let lines: Vec<&str> = s.split("\n").filter(|l| l.len() > 0).collect();
        for line in lines {
            walls.push(vec![]);
            let map = line.split(" ").next().unwrap();
            for c in map.bytes() {
                walls.last_mut().unwrap().push(c == b'#');
                if c == b'E' || c == b'G' {
                    units.push(Unit {
                        x: walls.last().unwrap().len() - 1,
                        y: walls.len() - 1,
                        elf: c == b'E',
                        hp: 200,
                        ap: if c == b'E' { ap_elf } else { ap_gob },
                    });
                }
            }
            let hps: Vec<_> = line
                .split(" ")
                .skip(1)
                .filter(|t| t.len() > 0)
                .map(|s| s[2..].trim_end_matches(",").trim_end_matches(")"))
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            hps.into_iter()
                .rev()
                .zip(units.iter_mut().rev())
                .for_each(|(hp, u)| u.hp = hp);
        }
        units.sort_by_key(|u| (u.y, u.x));
        State { walls, units }
    }

    fn unit_at(&self, x: usize, y: usize) -> Option<&Unit> {
        self.units.iter().find(|u| u.x == x && u.y == y)
    }

    fn unit_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Unit> {
        self.units.iter_mut().find(|u| u.x == x && u.y == y)
    }

    fn is_free(&self, x: usize, y: usize) -> bool {
        !self.walls[y][x] && self.unit_at(x, y).is_none()
    }

    fn in_range(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let is_elf = self.unit_at(x, y).unwrap().elf;
        let mut v = HashSet::new();
        for g in self.units.iter() {
            if g.elf == is_elf {
                continue;
            }
            v.extend(Self::around(g.x, g.y).filter(|&(x, y)| self.is_free(x, y)));
        }
        let mut v: Vec<_> = v.into_iter().collect();
        v.sort_by_key(|p| (p.1, p.0));
        v
    }

    fn path_cost_to(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Option<usize> {
        pathfinding::directed::dijkstra::dijkstra(
            &(x1, y1),
            |&(x1, y1)| {
                Self::around(x1, y1)
                    .filter(|&(x, y)| self.is_free(x, y))
                    .map(|p| (p, 1))
            },
            |&(x, y)| x == x2 && y == y2,
        )
        .map(|(_, c)| c)
    }

    fn chosen(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let all_reachable = pathfinding::directed::dijkstra::dijkstra_all(&(x, y), |&(x1, y1)| {
            Self::around(x1, y1)
                .filter(|&(x, y)| self.is_free(x, y))
                .map(|p| (p, 1))
        });
        let reachable = self
            .in_range(x, y)
            .into_iter()
            .filter_map(|p| all_reachable.get(&p).map(|(_prec, c)| (p, c)))
            .collect::<Vec<_>>();
        if reachable.len() == 0 {
            return None;
        }
        let min_dist: usize = *reachable.iter().map(|(_, c)| *c).min().unwrap();
        reachable
            .into_iter()
            .filter(|(_, c)| **c == min_dist)
            .map(|(p, _)| p)
            .min_by_key(|p| (p.1, p.0))
    }

    fn around(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(move |(dx, dy)| ((x as isize + *dx) as usize, (y as isize + *dy) as usize))
    }

    fn find_move_towards(&self, x: usize, y: usize, to_x: usize, to_y: usize) -> (usize, usize) {
        let moves_and_costs: Vec<_> = Self::around(x, y)
            .filter(|(x, y)| self.is_free(*x, *y))
            .filter_map(|p| self.path_cost_to(p.0, p.1, to_x, to_y).map(|c| (p, c)))
            .collect();
        moves_and_costs
            .iter()
            .min_by_key(|(p, c)| (c, p.1, p.0))
            .unwrap()
            .0
    }

    fn step_unit(&mut self, mut x: usize, mut y: usize) {
        let me = *self.unit_at(x, y).unwrap();
        if !Self::around(x, y)
            .any(|(x, y)| self.unit_at(x, y).map(|u| u.elf != me.elf).unwrap_or(false))
        {
            if let Some(chosen) = self.chosen(x, y) {
                let mv = self.find_move_towards(x, y, chosen.0, chosen.1);
                let mut u = self.unit_at_mut(x, y).unwrap();
                u.x = mv.0;
                u.y = mv.1;
                x = mv.0;
                y = mv.1;
            }
        }
        if let Some(t) = Self::around(x, y)
            .filter_map(|(x, y)| self.unit_at(x, y).filter(|u| u.elf != me.elf))
            .min_by_key(|p| (p.hp, p.y, p.x))
        {
            let target = self.unit_at_mut(t.x, t.y).unwrap();
            target.hp = target.hp.saturating_sub(me.ap);
            if target.hp == 0 {
                self.units.retain(|u| u.hp > 0);
            }
        }
    }

    fn step(&mut self) {
        let units_at_start = self.units.clone();
        for unit in units_at_start {
            if self.unit_at(unit.x, unit.y).is_none() {
                continue;
            }
            self.step_unit(unit.x, unit.y);
        }
        self.units.sort_by_key(|u| (u.y, u.x));
    }

    fn to_death(&mut self) -> usize {
        let mut i = 0;
        loop {
            self.step();
            if self.units.iter().all(|u| u.elf) || self.units.iter().all(|u| !u.elf) {
                return i;
            }
            i += 1;
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.walls.len() {
            let mut hps = vec![];
            for x in 0..self.walls[0].len() {
                let c = if self.walls[y][x] {
                    '#'
                } else if let Some(u) = self.unit_at(x, y) {
                    hps.push(u.hp);
                    if u.elf {
                        'E'
                    } else {
                        'G'
                    }
                } else {
                    '.'
                };
                write!(fmt, "{}", c)?;
            }
            writeln!(fmt, " {:?}", hps)?;
        }
        Ok(())
    }
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", p1(&s));
    println!("{:?}", p2(&s));
}

fn p1(s: &str) -> usize {
    let mut s = State::parse(s, 3, 3);
    let i = s.to_death();
    let remain = s.units.iter().map(|u| u.hp).sum::<usize>();
    remain * i
}

fn p2(s: &str) -> usize {
    'outer:
    for e in 3.. {
        let mut s = State::parse(s, e, 3);
        let elves = s.units.iter().filter(|u| u.elf).count();
        let mut i = 0;
        loop {
            s.step();
            if s.units.iter().filter(|u| u.elf).count() < elves {
                continue 'outer;
            }
            if s.units.len() == elves {
                let remain = s.units.iter().map(|u| u.hp).sum::<usize>();
                return i * remain
            }
            i+=1;
        }
    }
    panic!()
}

#[test]
fn test_simple() {
    let state = State::parse(
        r#"
#######
#E..G.#
#...#.#
#.G.#G#
#######"#,
        0,
        0,
    );
    assert_eq!(
        state.in_range(1, 1),
        vec!((3, 1), (5, 1), (2, 2), (5, 2), (1, 3), (3, 3))
    );
    assert_eq!(state.chosen(1, 1), Some((3, 1)));
    assert_eq!(state.find_move_towards(1, 1, 3, 1), (2, 1));
}

#[test]
fn test_move() {
    let mut state = State::parse(
        r#"
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"#,
        0,
        0,
    );
    state.step();
    let s1 = State::parse(
        r#"
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########"#,
        0,
        0,
    );
    assert_eq!(state, s1);
    state.step();
    let s2 = State::parse(
        r#"
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########"#,
        0,
        0,
    );
    assert_eq!(state, s2);
    state.step();
    let s3 = State::parse(
        r#"
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########"#,
        0,
        0,
    );
    assert_eq!(state, s3);
}

#[test]
fn test_move_attack() {
    let mut state = State::parse(
        r#"
#######
#.G...#   G(200)
#...EG#   E(200), G(200)
#.#.#G#   G(200)
#..G#E#   G(200), E(200)
#.....#
#######"#,
        3,
        3,
    );
    state.step();
    let s1 = State::parse(
        r#"
#######   
#..G..#   G(200)
#...EG#   E(197), G(197)
#.#G#G#   G(200), G(197)
#...#E#   E(197)
#.....#   
#######"#,
        3,
        3,
    );
    state.step();
    let s2 = State::parse(
        r#"
#######   
#...G.#   G(200)
#..GEG#   G(200), E(188), G(194)
#.#.#G#   G(194)
#...#E#   E(194)
#.....#   
#######"#,
        3,
        3,
    );
    assert_eq!(state, s2);
    (0..=20).for_each(|_| state.step());
    let s23 = State::parse(
        r#"
#######   
#...G.#   G(200)
#..G.G#   G(200), G(131)
#.#.#G#   G(131)
#...#E#   E(131)
#.....#   
#######"#,
        3,
        3,
    );
    assert_eq!(state, s23);
    (0..24).for_each(|_| state.step());
    let s47 = State::parse(
        r#"
#######   
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(59)
#...#.#   
#....G#   G(200)
#######"#,
        3,
        3,
    );
    assert_eq!(state, s47);
}

#[test]
fn test_p1() {
    assert_eq!(
        p1(r#"
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"#),
        36334
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2(r#"
#######
#.G...#   
#...EG#   
#.#.#G#   
#..G#E#   
#.....#
#######"#),
        4988
    );
}
