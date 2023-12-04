pub fn part1(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|line| extract_numbers(line))
        .map(|numbers| numbers.join("").parse::<i64>().unwrap())
        .sum()
}

pub fn part2(input: Vec<String>) -> i64 {
    let literals = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let nums: [String; 9] = core::array::from_fn(|i| format!("{}", i + 1));

    input
        .iter()
        .inspect(|line| println!("Mutating {}", line))
        .map(|line| {
            let mut transformed = line.clone();
            let mut matches: Vec<_> = vec![];
            literals
                .iter()
                .for_each(|lit| matches.append(&mut line.match_indices(lit).collect()));
            matches.sort();
            matches.iter().for_each(|m| {
                transformed = transformed.replace(
                    m.1,
                    nums.get(literals.iter().enumerate().find(|n| *n.1 == m.1).unwrap().0)
                        .unwrap(),
                );
            });

            let numbers = extract_numbers(&transformed);
            println!("Mutated {:?}\n", numbers);
            numbers.join("").parse::<i64>().unwrap()
        })
        .sum()
}

fn extract_numbers(line: &str) -> Vec<&str> {
    let mut matches = line.matches(char::is_numeric).collect::<Vec<&str>>();
    match matches.len() {
        3.. => {
            matches.reverse();
            matches.rotate_right(1);
            matches.truncate(2);
        }
        1 => {
            matches.push(matches.first().unwrap());
        }
        0 => matches.push("0"),
        _ => {}
    }
    matches
}

#[test]
fn part1_example() {
    let res = part1(
        ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .map(|s| s.to_string())
            .into_iter()
            .collect(),
    );
    assert_eq!(res, 142);
}

#[test]
fn part2_example() {
    let res = part2(
        [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .map(|s| s.to_string())
        .into_iter()
        .collect(),
    );
    assert_eq!(res, 281);
}
