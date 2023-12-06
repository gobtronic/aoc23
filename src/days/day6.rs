pub fn part1(input: Vec<String>) -> i64 {
    let records = extract_records(&input);
    records
        .iter()
        .map(|r| {
            let quads = quads(r);
            let mut possibilities: i64 = 0;
            for quad in quads {
                for i in quad + 1.. {
                    let distance = (r.time as i64 - i) * i;
                    if distance > r.distance as i64 && i < r.time as i64 {
                        possibilities += 1;
                    } else {
                        break;
                    }
                }
            }

            possibilities
        })
        .product()
}

pub fn part2(input: Vec<String>) -> i64 {
    let record = extract_record(&input);
    let quads = quads(&record);
    let mut possibilities: i64 = 0;
    for quad in quads {
        for i in quad + 1.. {
            let distance = (record.time as i64 - i) * i;
            if distance > record.distance as i64 && i < record.time as i64 {
                possibilities += 1;
            } else {
                break;
            }
        }
    }

    possibilities
}

fn quads(rec: &Record) -> Vec<i64> {
    let first =
        (rec.time + (((rec.time as i64).pow(2) - 4 * rec.distance as i64) as f64).sqrt()) / 2_f64;
    let second =
        (rec.time - (((rec.time as i64).pow(2) - 4 * rec.distance as i64) as f64).sqrt()) / 2_f64;
    let mut quads = vec![first.floor() as i64, second.floor() as i64];
    quads.sort();
    quads
}

fn extract_records(input: &[String]) -> Vec<Record> {
    let time_line = input.first().unwrap();
    let times: Vec<f64> = extract_values(time_line);

    let distance_line = input.last().unwrap();
    let distances: Vec<f64> = extract_values(distance_line);

    times
        .iter()
        .enumerate()
        .map(|t| Record {
            time: *t.1,
            distance: *distances.get(t.0).unwrap(),
        })
        .collect()
}

fn extract_record(input: &[String]) -> Record {
    let time_line = input.first().unwrap();
    let time: f64 = extract_value(time_line);

    let distance_line = input.last().unwrap();
    let distance: f64 = extract_value(distance_line);

    Record { time, distance }
}

fn extract_values(line: &str) -> Vec<f64> {
    line.replace("Distance:", "")
        .replace("Time:", "")
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|v| v.parse::<f64>().unwrap())
        .collect()
}

fn extract_value(line: &str) -> f64 {
    line.replace("Distance:", "")
        .replace("Time:", "")
        .trim()
        .replace(' ', "")
        .parse::<f64>()
        .unwrap()
}

#[derive(Debug)]
struct Record {
    time: f64,
    distance: f64,
}

#[test]
fn part1_example() {
    let res = part1(aoc23::parse_lines(
        r#"Time:      7  15   30
Distance:  9  40  200"#,
    ));
    assert_eq!(res, 288)
}

#[test]
fn part2_example() {
    let res = part2(aoc23::parse_lines(
        r#"Time:      7  15   30
Distance:  9  40  200"#,
    ));
    assert_eq!(res, 71503)
}
