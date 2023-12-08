pub fn part1(input: Vec<String>) -> i64 {
    let (instructions, locations) = extract(&input);

    let mut distance = 0;
    let mut cur_location = locations.iter().find(|l| l.id == "AAA").unwrap();
    while cur_location.id != "ZZZ" {
        for inst in &instructions {
            let next_location = match inst {
                'L' => locations
                    .iter()
                    .find(|l| l.id == cur_location.left)
                    .unwrap(),
                'R' => locations
                    .iter()
                    .find(|l| l.id == cur_location.right)
                    .unwrap(),
                _ => panic!("Instruction should be 'L' or 'R'"),
            };
            cur_location = next_location;
            distance += 1;
        }
    }
    distance
}

pub fn part2(input: Vec<String>) -> i64 {
    0
}

fn extract(input: &[String]) -> (Vec<char>, Vec<Location>) {
    let instructions = input.first().unwrap().chars().collect();
    let locations: Vec<Location> = input
        .iter()
        .skip(2)
        .map(|loc| {
            let parts: Vec<&str> = loc.split(" = ").collect();
            let location = parts.first().unwrap();
            let branches = parts.last().unwrap();
            let branches: Vec<&str> = branches.split(", ").collect();
            Location {
                id: location.to_string(),
                left: branches.first().unwrap().replace('(', ""),
                right: branches.last().unwrap().replace(')', ""),
            }
        })
        .collect();

    (instructions, locations)
}

struct Location {
    id: String,
    left: String,
    right: String,
}

#[test]
fn part1_example() {
    let res = part1(aoc23::parse_lines(
        r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#,
    ));
    assert_eq!(res, 6)
}

#[test]
fn part2_example() {
    let res = part2(aoc23::parse_lines(
        r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#,
    ));
    assert_eq!(res, 0)
}
