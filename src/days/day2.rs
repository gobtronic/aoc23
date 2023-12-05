pub fn part1(input: Vec<String>) -> i64 {
    games(input)
        .iter()
        .filter(|g| g.is_valid())
        .map(|g| g.id)
        .sum()
}

pub fn part2(input: Vec<String>) -> i64 {
    0
}

fn games(input: Vec<String>) -> Vec<Game> {
    input.into_iter().map(|line| Game::from(line)).collect()
}

#[derive(Debug)]
struct Game {
    pub id: i64,
    pub rounds: Vec<(u8, u8, u8)>,
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        let game_parts: Vec<&str> = value.split(": ").collect();
        let id = game_parts
            .first()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<Vec<&str>>()
            .join("")
            .parse::<i64>()
            .unwrap();
        let rounds: Vec<(u8, u8, u8)> = game_parts
            .last()
            .unwrap()
            .split("; ")
            .map(|p| Game::round(p))
            .collect();

        Game { id, rounds }
    }
}

impl Game {
    fn round(value: &str) -> (u8, u8, u8) {
        let mut colors = (0, 0, 0);
        value
            .split(", ")
            .map(|val| val.split(" ").collect::<Vec<&str>>())
            .for_each(|c_parts| {
                let nb = c_parts.first().unwrap().parse::<u8>().unwrap();
                match c_parts.last().unwrap() {
                    &"red" => colors.0 = nb,
                    &"green" => colors.1 = nb,
                    &"blue" => colors.2 = nb,
                    _ => {}
                }
            });
        colors
    }

    fn is_valid(&self) -> bool {
        !self.rounds.iter().any(|r| r.0 > 12 || r.1 > 13 || r.2 > 14)
    }
}

#[test]
fn part1_example() {
    let res = part1(
        [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .map(|s| s.to_string())
        .into_iter()
        .collect(),
    );
    assert_eq!(res, 8);
}
