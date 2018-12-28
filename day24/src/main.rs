#[derive(Copy, Clone, Debug, PartialEq)]
enum Element {
    Rad,
    Blud,
    Fire,
    Slash,
    Cold,
}

impl Element {
    fn parse(s: &str) -> Element {
        match s {
            "radiation" => Element::Rad,
            "bludgeoning" => Element::Blud,
            "fire" => Element::Fire,
            "cold" => Element::Cold,
            "slashing" => Element::Slash,
            s => panic!("{:?} is not an element", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Group {
    units: usize,
    hp: usize,
    attack: usize,
    attack_kind: Element,
    initiative: usize,
    weak: Vec<Element>,
    immune: Vec<Element>,
}

impl Group {
    fn parse(s: &str) -> Group {
        let tokens: Vec<&str> = s.split(" ").collect();
        let units = tokens[0].parse::<usize>().unwrap();
        let hp = tokens[4].parse::<usize>().unwrap();
        let initiative = tokens[tokens.len() - 1].parse::<usize>().unwrap();
        let attack = tokens[tokens.len() - 6].parse::<usize>().unwrap();
        let attack_kind = Element::parse(tokens[tokens.len() - 5]);
        let mut weak = vec![];
        let mut immune = vec![];
        if s.contains('(') {
            let spec: String = s
                .chars()
                .skip_while(|&c| c != '(')
                .take_while(|&c| c != ')')
                .collect();
            for sub in spec.split("; ") {
                let elements: Vec<_> = sub
                    .split(" ")
                    .skip(2)
                    .map(|e| Element::parse(e.trim_end_matches(|x| ";),".contains(x))))
                    .collect();
                if sub.trim_start_matches("(").starts_with("weak") {
                    weak = elements;
                } else {
                    immune = elements;
                }
            }
        }
        Group {
            units,
            hp,
            attack,
            initiative,
            attack_kind,
            weak,
            immune,
        }
    }

    pub fn damages_to(&self, other: &Group) -> usize {
        if other.immune.contains(&self.attack_kind) {
            return 0;
        }
        let raw = self.attack * self.units;
        if other.weak.contains(&self.attack_kind) {
            2 * raw
        } else {
            raw
        }
    }
}

fn parse_team(s: &str) -> Vec<Group> {
    s.split("\n")
        .filter(|l| l.len() > 0)
        .skip(1)
        .map(Group::parse)
        .collect()
}

fn parse(s: &str) -> (Vec<Group>, Vec<Group>) {
    let mut teams = s.split("\n\n");
    (
        parse_team(teams.next().unwrap()),
        parse_team(teams.next().unwrap()),
    )
}

fn target_selection_half(from: &[Group], to: &[Group]) -> Vec<Option<usize>> {
    let mut order: Vec<usize> = (0..(from.len())).collect();
    order.sort_by_key(|&o| {
        let a = &from[o];
        (-((a.units * a.attack) as isize), -(a.initiative as isize))
    });
    let mut attackees: Vec<Option<usize>> = vec![None; from.len()];
    for i in order {
        if from[i].units == 0 {
            continue;
        }
        attackees[i] = to
            .iter()
            .enumerate()
            .filter(|(ix, target)| !attackees.contains(&Some(*ix)) && target.units > 0 && from[i].damages_to(target) > 0)
            .max_by_key(|(_ix, t)| {
                let option = (from[i].damages_to(t), t.units * t.attack, t.initiative as isize);
                option
            })
            .map(|(ix, _t)| ix);
    }
    attackees
}

fn target_selection(a: &[Group], b: &[Group]) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    (target_selection_half(a, b), target_selection_half(b, a))
}

fn fight(team_a: &mut [Group], team_b: &mut [Group]) {
    let (targets_a, targets_b) = target_selection(team_a, team_b);
    let mut attacks: Vec<(bool, usize, usize, usize)> = targets_a
        .iter()
        .enumerate()
        .filter(|(_a, t)| t.is_some())
        .map(|(a, t)| (false, a, t.unwrap(), team_a[a].initiative))
        .chain(
            targets_b
                .iter()
                .enumerate()
                .filter(|(_a, t)| t.is_some())
                .map(|(a, t)| (true, a, t.unwrap(), team_b[a].initiative)),
        )
        .collect();
    attacks.sort_by_key(|&(_is_b, _at, _de, pow)| -(pow as isize));
    for attack in attacks {
        // println!("{:?}", attack);
        let (at, def) = if attack.0 {
            (&team_b[attack.1], &mut team_a[attack.2])
        } else {
            (&team_a[attack.1], &mut team_b[attack.2])
        };
        if at.units == 0 {
            continue;
        }
        let dam = at.damages_to(def);
        def.units = def.units.saturating_sub(dam / def.hp);
    }
}

fn to_death(team_a: &mut [Group], team_b: &mut [Group]) {
    let mut previous = None;
    loop {
        fight(team_a, team_b);
        let current = (team_a.iter().map(|g| g.units).sum::<usize>(),
                       team_b.iter().map(|g| g.units).sum::<usize>());
        if Some(current) == previous || current.0 == 0 || current.1 == 0 {
            return;
        }
        previous = Some(current)
    }
}

fn p1(s: &str) -> usize {
    let (mut sys, mut inf) = parse(&s);
    to_death(&mut sys, &mut inf);
    let s = sys.iter().map(|g| g.units).sum::<usize>();
    let i = inf.iter().map(|g| g.units).sum::<usize>();
    s.max(i)
}

fn p2(s: &str) -> usize {
    let (s1, i1) = parse(&s);
    for boost in 0.. {
        println!("trying boost {}", boost);
        let mut sys = s1.clone();
        let mut inf = i1.clone();
        for mut g in &mut sys {
            g.attack += boost;
        }
        to_death(&mut sys, &mut inf);
        let i = inf.iter().map(|g| g.units).sum::<usize>();
        if i == 0 {
            return sys.iter().map(|g| g.units).sum::<usize>();
        }
    }
    unreachable!()
}


fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("P1: {}", p1(&s));
    println!("P2: {}", p2(&s));
}

#[test]
fn t1() {
    let input = r#"
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
"#;
    let (mut sys, mut inf) = parse(input);
    assert_eq!(
        sys,
        vec!(
            Group {
                units: 17,
                hp: 5390,
                attack: 4507,
                attack_kind: Element::Fire,
                initiative: 2,
                immune: vec!(),
                weak: vec!(Element::Rad, Element::Blud),
            },
            Group {
                units: 989,
                hp: 1274,
                attack: 25,
                attack_kind: Element::Slash,
                initiative: 3,
                immune: vec!(Element::Fire),
                weak: vec!(Element::Blud, Element::Slash),
            }
        )
    );
    assert_eq!(inf[0].damages_to(&sys[0]), 185832);
    assert_eq!(inf[0].damages_to(&sys[1]), 185832);
    assert_eq!(inf[1].damages_to(&sys[1]), 107640);
    fight(&mut sys, &mut inf);
    assert_eq!(sys[0].units, 0);
    assert_eq!(sys[1].units, 905);
    assert_eq!(inf[0].units, 797);
    assert_eq!(inf[1].units, 4434);
    fight(&mut sys, &mut inf);
    assert_eq!(sys[0].units, 0);
    assert_eq!(sys[1].units, 761);
    assert_eq!(inf[0].units, 793);
    assert_eq!(inf[1].units, 4434);
    fight(&mut sys, &mut inf);
    assert_eq!(sys[0].units, 0);
    assert_eq!(sys[1].units, 618);
    assert_eq!(inf[0].units, 789);
    assert_eq!(inf[1].units, 4434);
    to_death(&mut sys, &mut inf);
    assert_eq!(sys[0].units, 0);
    assert_eq!(sys[1].units, 0);
    assert_eq!(inf[0].units, 782);
    assert_eq!(inf[1].units, 4434);
    assert_eq!(p1(input), 5216);
}
