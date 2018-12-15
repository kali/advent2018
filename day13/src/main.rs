use std::fmt;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", q1(&s));
    println!("{:?}", q2(&s));
}

struct World {
    lines: Vec<Vec<u8>>,
    carts: Vec<Cart>,
}

impl World {
    fn parse(s: &str) -> World {
        let mut lines: Vec<Vec<u8>> = s
            .split("\n")
            .filter(|l| l.len() > 0)
            .map(|s| format!("{}", s).as_bytes().to_vec())
            .collect();
        let mut carts = vec![];
        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                let dir = match c {
                    b'<' => Some(Dir::Left),
                    b'>' => Some(Dir::Right),
                    b'^' => Some(Dir::Up),
                    b'v' => Some(Dir::Down),
                    _ => None,
                };
                if let Some(dir) = dir {
                    carts.push(Cart { x, y, dir, rot: 0, crashed: false });
                }
            }
        }
        for &Cart { x, y, .. } in carts.iter() {
            lines[y][x] = if lines[y][x] == b'<' || lines[y][x] == b'>' {
                b'-'
            } else {
                b'|'
            };
        }
        World { lines, carts }
    }

    fn step(&mut self) -> Vec<Cart> {
        self.carts.sort_by_key(|c| (c.y, c.x));
        for i in 0..self.carts.len() {
            match self.carts[i].dir {
                Dir::Up => self.carts[i].y -= 1,
                Dir::Down => self.carts[i].y += 1,
                Dir::Left => self.carts[i].x -= 1,
                Dir::Right => self.carts[i].x += 1,
            }
            if let Some(c) = self
                .carts
                .iter()
                .enumerate()
                .find(|&(ix, cart)| ix != i && cart.x == self.carts[i].x && cart.y == self.carts[i].y)
                .map(|(ix,_)| ix)
            {
                self.carts[i].crashed = true;
                self.carts[c].crashed = true;
            }
            let dir = match (
                self.carts[i].dir,
                self.lines[self.carts[i].y][self.carts[i].x],
            ) {
                (Dir::Up, b'/') => Dir::Right,
                (Dir::Left, b'/') => Dir::Down,
                (Dir::Down, b'/') => Dir::Left,
                (Dir::Right, b'/') => Dir::Up,
                (Dir::Up, b'\\') => Dir::Left,
                (Dir::Left, b'\\') => Dir::Up,
                (Dir::Down, b'\\') => Dir::Right,
                (Dir::Right, b'\\') => Dir::Down,
                (_, b'+') => {
                    let dir = (5 - self.carts[i].rot + self.carts[i].dir as u8) % 4;
                    self.carts[i].rot = (self.carts[i].rot + 1) % 3;
                    unsafe { std::mem::transmute(dir) }
                }
                (d, _) => d,
            };
            self.carts[i].dir = dir;
        }
        let mut crashed = vec!();
        for i in (0..self.carts.len()).rev() {
            if self.carts[i].crashed {
                crashed.push(self.carts.remove(i));
            }
        }
        crashed.reverse();
        crashed
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, l) in self.lines.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                if let Some(cart) = self.carts.iter().find(|c| c.x == x && c.y == y) {
                    let c = match cart.dir {
                        Dir::Up => '^',
                        Dir::Down => 'v',
                        Dir::Left => '<',
                        Dir::Right => '>',
                    };
                    write!(f, "{}", c)?;
                } else {
                    write!(f, "{}", *c as char)?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Cart {
    x: usize,
    y: usize,
    dir: Dir,
    rot: u8,
    crashed: bool,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Dir {
    Right = 0,
    Up,
    Left,
    Down,
}

fn q1(s: &str) -> (usize, usize) {
    let mut w = World::parse(s);
    loop {
        let crashes = w.step();
        if crashes.len() > 0 {
            return (crashes[0].x, crashes[0].y)
        }
    }
}

fn q2(s: &str) -> (usize, usize) {
    let mut w = World::parse(s);
    loop {
        let _crashes = w.step();
        if w.carts.len() == 1 {
            return (w.carts[0].x, w.carts[0].y);
        }
    }
}

#[test]
fn test_plus() {
    let mut w = World::parse(r#"
  ++
  +
->+
"#);
    (0..4).for_each(|_| { w.step(); });
    assert_eq!(w.carts, &[Cart{x: 3, y:0, dir:Dir::Up, rot: 1, crashed:false}]);
}

#[test]
fn test_q1() {
    let map = r#"
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
"#;
    assert_eq!(q1(map), (7, 3));
}

#[test]
fn test_q2() {
    let map = r#"
/---\  
|   |  
| /-+-\
| v | |
\-+-/ |
  ^   ^
  \---/
"#;
    assert_eq!(q2(map), (6, 4));
}
