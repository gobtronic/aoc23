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
    let (instructions, locations) = extract(&input);

    let mut cur_locations: Vec<&Location> =
        locations.iter().filter(|l| l.id.ends_with('A')).collect();
    let mut cur_distance = 0_i64;
    let mut distances: Vec<Option<i64>> = Vec::with_capacity(cur_locations.len());
    for _ in 0..cur_locations.len() {
        distances.push(None);
    }

    while lcm_distance(&distances).is_none() {
        for inst in &instructions {
            let next_locations = match inst {
                'L' => cur_locations
                    .iter()
                    .map(|cur_l| locations.iter().find(|l| l.id == cur_l.left).unwrap())
                    .collect(),
                'R' => cur_locations
                    .iter()
                    .map(|cur_l| locations.iter().find(|l| l.id == cur_l.right).unwrap())
                    .collect(),
                _ => panic!("Instruction should be 'L' or 'R'"),
            };
            cur_locations = next_locations;
            for (i, loc) in cur_locations.iter().enumerate() {
                if loc.id.ends_with('Z') {
                    let _ = std::mem::replace(&mut distances[i], Some(cur_distance));
                }
            }
            cur_distance += 1;
        }
    }

    lcm_distance(&distances).unwrap()
}

fn lcm_distance(distances: &[Option<i64>]) -> Option<i64> {
    if distances.into_iter().any(|d| d.is_none()) {
        return None;
    }

    let distances: Vec<i64> = distances.into_iter().map(|d| d.unwrap()).collect();
    let result = distances.windows(2).fold(0, |acc, pair| {
        if acc == 0 {
            lcm(pair[0], pair[1])
        } else {
            lcm(acc, pair[1])
        }
    });

    Some(result)
}

fn lcm(first: i64, second: i64) -> i64 {
    let first = first.try_into().unwrap();
    let second = second.try_into().unwrap();
    (first * second / gcd(first, second)).try_into().unwrap()
}

fn gcd(first: u32, second: u32) -> u32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
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
        r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#,
    ));
    assert_eq!(res, 6)
}
