struct Node {
    children: Vec<Node>,
    meta: Vec<usize>,
}

impl Node {
    fn parse(it: &mut impl Iterator<Item = usize>) -> Node {
        let children_count = it.next().unwrap();
        let meta_count = it.next().unwrap();
        let children = (0..children_count).map(|_| Node::parse(it)).collect();
        let meta = it.take(meta_count).collect();
        Node { children, meta }
    }

    fn q1(&self) -> usize {
        self.children.iter().map(|c| c.q1()).sum::<usize>()
            + self.meta.iter().cloned().sum::<usize>()
    }

    fn q2(&self) -> usize {
        if self.children.len() == 0 {
            self.meta.iter().cloned().sum::<usize>()
        } else {
            self.meta.iter().map(|&n|
                if n>0 && n<=self.children.len() {
                    self.children[n-1].q2()
                } else {
                    0
                }).sum::<usize>()
        }
    }
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(s.trim()));
}

fn run(s: &str) -> (usize, usize) {
    let mut it = s.split(" ").map(|s| s.parse::<usize>().unwrap());
    let root = Node::parse(&mut it);
    (root.q1(), root.q2())
}

#[test]
fn test() {
    assert_eq!(run("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), (138, 66));
}
