use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> i64 {
    let hands = input.iter().map(|val| Hand::from(val.as_str()));
    let hands_map: HashMap<HandType, Vec<Hand>> = hands.fold(HashMap::new(), |mut acc, h| {
        let type_hands = acc.entry(h.hand_type()).or_default();
        type_hands.push(h);
        acc
    });

    let mut sorted_map = hands_map
        .into_iter()
        .collect::<Vec<(HandType, Vec<Hand>)>>();
    sorted_map.sort();
    sorted_map
        .iter_mut()
        .flat_map(|(_, v)| {
            if v.len() > 1 {
                v.sort();
            }

            v
        })
        .enumerate()
        .fold(0_i64, |mut acc, (i, h)| {
            acc += h.bid * ((i as i64) + 1);
            acc
        })
}

pub fn part2(input: Vec<String>) -> i64 {
    0
}

#[derive(Eq, Hash, Debug, Ord)]
struct Hand {
    cards: Vec<char>,
    bid: i64,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let parts = value.split(' ').collect::<Vec<&str>>();
        Hand {
            cards: parts.first().unwrap().chars().collect(),
            bid: parts.last().unwrap().parse().unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() == other.hand_type()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => {
                for (i, c) in self.cards.iter().enumerate() {
                    let other_c = other.cards.get(i).unwrap();
                    match card_val(other_c).cmp(&card_val(c)) {
                        std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Greater),
                        std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Less),
                        std::cmp::Ordering::Equal => {}
                    };
                }

                None
            }
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let cards_map: HashMap<char, i64> = self.cards.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        });
        let mut sorted_occurences = cards_map.iter().collect::<Vec<(&char, &i64)>>();
        sorted_occurences.sort_by(|a, b| b.1.cmp(a.1));

        let highest = sorted_occurences.first().unwrap();
        if *highest.1 == 5 {
            return HandType::FiveOAK;
        }
        let sec_highest = sorted_occurences.get(1).unwrap();
        match (*highest.1, *sec_highest.1) {
            (4, _) => HandType::FourOAK,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOAK,
            (2, 2) => HandType::TwoP,
            (1, 1) => HandType::High,
            _ => HandType::OneP,
        }
    }
}

fn card_val(char: &char) -> i64 {
    match char {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    High,
    OneP,
    TwoP,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

#[test]
fn part1_example() {
    let res = part1(aoc23::parse_lines(
        r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#,
    ));
    assert_eq!(res, 6440)
}

#[test]
fn part2_example() {
    let res = part2(aoc23::parse_lines(
        r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#,
    ));
    assert_eq!(res, 0)
}
