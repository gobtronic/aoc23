pub fn part1(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|l| {
            let mut matches = l.matches(char::is_numeric).collect::<Vec<&str>>();
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
        })
        .map(|numbers| numbers.join("").parse::<i64>().unwrap())
        .sum()
}

pub fn part2(input: Vec<String>) -> i64 {
    0
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
