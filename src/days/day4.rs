pub fn part1(input: Vec<String>) -> i64 {
    input
        .iter()
        .enumerate()
        .map(|line| {
            line.1
                .split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .collect::<Vec<&str>>()
        })
        .map(Card::from)
        .map(|c| c.points().1)
        .sum()
}

pub fn part2(input: Vec<String>) -> i64 {
    let mut cards: Vec<Card> = input
        .iter()
        .enumerate()
        .map(|line| {
            line.1
                .split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .collect::<Vec<&str>>()
        })
        .map(Card::from)
        .collect();

    for i in 0..cards.len() - 1 {
        let card = cards.get(i).unwrap();
        let copies = card.copies;
        let (wins, points) = card.points();

        if points > 0 && i < cards.len() - 1 {
            let end = std::cmp::min(cards.len(), i + 1 + wins);
            let _ = &cards[i + 1..end].iter_mut().for_each(|c| {
                c.copies += copies;
            });
        }
    }

    cards.iter().map(|c| c.copies).sum()
}

struct Card {
    copies: i64,
    winning: Vec<i64>,
    played: Vec<i64>,
}

impl Card {
    fn points(&self) -> (usize, i64) {
        let wins = self
            .winning
            .iter()
            .filter(|w| self.played.contains(w))
            .count();
        if wins == 0 {
            return (0, 0);
        }
        (
            wins,
            2_i64.pow(u32::try_from(wins - 1).unwrap()) * self.copies,
        )
    }
}

impl From<Vec<&str>> for Card {
    fn from(value: Vec<&str>) -> Self {
        Card {
            copies: 1,
            winning: extract_values(value.first().unwrap()),
            played: extract_values(value.last().unwrap()),
        }
    }
}

fn extract_values(str: &str) -> Vec<i64> {
    str.split(' ')
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.parse::<i64>().unwrap())
        .collect()
}

#[test]
fn part1_example() {
    let res = part1(aoc23::parse_lines(
        r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
    ));
    assert_eq!(res, 13)
}

#[test]
fn part2_example() {
    let res = part2(aoc23::parse_lines(
        r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
    ));
    assert_eq!(res, 30)
}
