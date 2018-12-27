use nom::*;
use std::collections::HashSet;

use pathfinding::directed::strongly_connected_components::strongly_connected_components_from;

#[derive(Default, Debug)]
struct Doors {
    to_east: HashSet<(isize, isize)>,
    to_south: HashSet<(isize, isize)>,
}

impl Doors {
    fn neighbours(&self, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
        let mut n = vec![];
        if self.to_east.contains(&(x, y)) {
            n.push((x + 1, y));
        }
        if self.to_south.contains(&(x, y)) {
            n.push((x, y + 1));
        }
        if self.to_east.contains(&(x - 1, y)) {
            n.push((x - 1, y));
        }
        if self.to_south.contains(&(x, y - 1)) {
            n.push((x, y - 1));
        }
        n
    }

    fn all_rooms(&self) -> Vec<Vec<(isize, isize)>> {
        strongly_connected_components_from(&(0, 0), |&pos| self.neighbours(pos))
    }

    fn build(re: &str) -> Doors {
        let re = full_regex(re).unwrap().1;
        let (doors, _) = explore(&*re);
        println!("done finding doors...");
        doors
    }

    fn farthest_room_dist(&self) -> usize {
        let rooms = pathfinding::directed::dijkstra::dijkstra_all(&(0, 0), |&pos| {
            self.neighbours(pos).into_iter().map(|(x, y)| ((x, y), 1))
        });
        rooms.values().map(|(_, cost)| *cost).max().unwrap()
    }

    fn room_further_than(&self, limit: usize) -> usize {
        let rooms = pathfinding::directed::dijkstra::dijkstra_all(&(0, 0), |&pos| {
            self.neighbours(pos).into_iter().map(|(x, y)| ((x, y), 1))
        });
        rooms.values().filter(|(_, cost)| *cost >= limit).count()
    }
}

#[derive(Debug, Hash)]
enum RegItem {
    N,
    S,
    E,
    W,
    Alt(Vec<Vec<RegItem>>),
}

fn explore(items: &[RegItem]) -> (Doors, HashSet<(isize, isize)>) {
    let mut doors = Doors::default();
    let mut positions = HashSet::new();
    positions.insert((0,0));
    for item in items {
        let mut next_positions = HashSet::new();
        match item {
            RegItem::Alt(alt) => {
                for opt in alt {
                    let (new_doors, new_pos) = explore(opt);
                    for (x,y) in &positions {
                        for (dx, dy) in &new_pos {
                            next_positions.insert((x+dx,y+dy));
                        }
                        for (dx, dy) in new_doors.to_south.iter() {
                            doors.to_south.insert((x+dx, y+dy));
                        }
                        for (dx, dy) in new_doors.to_east.iter() {
                            doors.to_east.insert((x+dx, y+dy));
                        }
                    }
                }
            }
            RegItem::N => {
                for (x,y) in positions {
                    next_positions.insert((x,y-1));
                    doors.to_south.insert((x, y - 1));
                }
            }
            RegItem::S => {
                for (x,y) in positions {
                    next_positions.insert((x,y+1));
                    doors.to_south.insert((x, y));
                }
            }
            RegItem::W => {
                for (x,y) in positions {
                    next_positions.insert((x-1,y));
                    doors.to_east.insert((x-1, y));
                }
            }
            RegItem::E => {
                for (x,y) in positions {
                    next_positions.insert((x+1,y));
                    doors.to_east.insert((x, y));
                }
            }
        }
        positions = next_positions;
    }
    (doors, positions)
}

named!(full_regex <&str, Vec<RegItem>>, delimited!(
    tag!("^"),
    regex,
    tag!("$")));

named!(regex <&str, Vec<RegItem>>, many0!(regitem));

named!(regitem <&str, RegItem>, alt!(
        map!(tag!("N"), |_| RegItem::N) |
        map!(tag!("S"), |_| RegItem::S) |
        map!(tag!("E"), |_| RegItem::E) |
        map!(tag!("W"), |_| RegItem::W) |
        map!(delimited!(tag!("("), regalts, tag!(")")), |x| RegItem::Alt(x)) |
        map!(delimited!(tag!("("), regalts, tag!("|)")), |mut x| { x.push(vec!()); RegItem::Alt(x)} )
));

named!(regalts <&str, Vec<Vec<RegItem>>>,
        separated_list!(tag!("|"), map!(opt!(regex), |re| re.unwrap_or(vec!()))));

fn main() {
    let doors = Doors::build(std::fs::read_to_string("input").unwrap().trim());
    println!("{}", doors.farthest_room_dist());
    println!("{}", doors.room_further_than(1000));
}

#[test]
fn t_1() {
    let doors = Doors::build("^WNE$");
    assert_eq!(doors.farthest_room_dist(), 3);
}

#[test]
fn t_2() {
    let doors = Doors::build("^ENWWW(NEEE|SSE(EE|N))$");
    assert_eq!(doors.farthest_room_dist(), 10);
}

#[test]
fn p_1() {
    full_regex("^$").unwrap();
    full_regex("^(N|S)$").unwrap();
    full_regex("^(N|)$").unwrap();
}
