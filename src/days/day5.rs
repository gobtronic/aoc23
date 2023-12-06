use std::iter::Peekable;
use std::slice::Iter;

use rayon::prelude::*;

pub fn part1(input: Vec<String>) -> i64 {
    let seeds = extracts_all_seeds(&input);
    let map_groups = extract_map_groups(&input);
    let mut seed_locations: Vec<(i64, i64)> = vec![];
    seeds.into_iter().for_each(|seed| {
        seed_locations.push((seed, browse(seed, map_groups.iter().peekable())));
    });
    seed_locations.sort_by(|a, b| a.1.cmp(&b.1));
    seed_locations.first().unwrap().1
}

pub fn part2(input: Vec<String>) -> i64 {
    let seeds = extract_seed_from_pairs(&input);
    let map_groups = extract_map_groups(&input);
    *seeds
        .par_iter()
        .map(|seed| browse(*seed, map_groups.iter().peekable()))
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
}

fn browse(source: i64, mut map_groups: Peekable<Iter<'_, Vec<Map>>>) -> i64 {
    let maps = map_groups.next().unwrap();
    let destination = if let Some(map) = maps
        .iter()
        .find(|map| (map.source..map.source + map.length).contains(&source))
    {
        map.destination + (source - map.source)
    } else {
        source
    };

    if map_groups.peek().is_some() {
        return browse(destination, map_groups);
    }

    destination
}

type Seeds = Vec<i64>;

#[derive(Clone, Debug)]
struct Map {
    destination: i64,
    source: i64,
    length: i64,
}

impl Map {
    fn extract(from: Vec<&str>) -> Vec<Map> {
        from.into_iter().skip(1).map(Map::from).collect()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(' ').collect();
        Map {
            destination: parts.first().unwrap().parse::<i64>().unwrap(),
            source: parts.get(1).unwrap().parse::<i64>().unwrap(),
            length: parts.last().unwrap().parse::<i64>().unwrap(),
        }
    }
}

fn extracts_all_seeds(input: &[String]) -> Seeds {
    let seed_line = input.first().unwrap();
    seed_line
        .replace("seeds: ", "")
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn extract_seed_from_pairs(input: &[String]) -> Seeds {
    let seed_line = input.first().unwrap();
    let seeds: Seeds = seed_line
        .replace("seeds: ", "")
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let chunks: Vec<Seeds> = seeds.chunks(2).map(|seeds| seeds.to_vec()).collect();
    chunks
        .iter()
        .flat_map(|seeds| {
            let start = seeds.first().unwrap();
            let end = start + seeds.last().unwrap();
            (*start..end).collect::<Seeds>()
        })
        .collect()
}

fn extract_map_groups(input: &[String]) -> Vec<Vec<Map>> {
    input
        .iter()
        .skip(2)
        .map(|l| l.as_str())
        .collect::<Vec<&str>>()
        .join("\n")
        .split("\n\n")
        .map(|map| map.split('\n').collect::<Vec<&str>>())
        .map(|raw| Map::extract(raw))
        .collect()
}

#[test]
fn part1_example() {
    let res = part1(aoc23::parse_lines(
        r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
    ));
    assert_eq!(res, 35)
}

#[test]
fn part2_example() {
    let res = part2(aoc23::parse_lines(
        r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
    ));
    assert_eq!(res, 46)
}
