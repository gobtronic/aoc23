pub fn part1(input: Vec<String>) -> i64 {
    let matrix = matrix(input);
    let numbers = numbers(&matrix);
    parts_of_engine(numbers, &matrix).iter().sum()
}

fn parts_of_engine(nums: Vec<Number>, matrix: &Vec<Vec<char>>) -> Vec<i64> {
    nums.iter()
        .filter(|n| touches_symbol(n, matrix))
        .map(|n| n.into())
        .collect()
}

pub fn part2(input: Vec<String>) -> i64 {
    let matrix = matrix(input);
    let numbers = numbers(&matrix);
    0
}

fn matrix(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn numbers<'a>(matrix: &'a Vec<Vec<char>>) -> Vec<Number<'a>> {
    let mut numbers: Vec<Number> = vec![];
    for line in matrix.iter().enumerate() {
        let mut number = Number::new(line.0);
        let mut iter = line.1.iter().enumerate().peekable();
        while let Some(chars) = iter.peek() {
            if chars.1.is_numeric() {
                number.chars_indices.push(*chars);
                if chars.0 == line.1.len() - 1 {
                    numbers.push(number);
                    number = Number::new(line.0);
                }
            } else if !number.chars_indices.is_empty() {
                numbers.push(number);
                number = Number::new(line.0);
            }

            iter.next();
        }
    }
    numbers
}

struct Number<'a> {
    line: usize,
    chars_indices: Vec<(usize, &'a char)>,
}

impl Number<'_> {
    fn new(line: usize) -> Self {
        Number {
            line,
            chars_indices: vec![],
        }
    }
}

impl From<&Number<'_>> for i64 {
    fn from(value: &Number<'_>) -> Self {
        value
            .chars_indices
            .iter()
            .map(|c_indice| c_indice.1.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i64>()
            .unwrap()
    }
}

fn touches_symbol(num: &Number, matrix: &Vec<Vec<char>>) -> bool {
    let matrix_w = matrix.first().unwrap().len();
    let matrix_h = matrix.len();

    let start = num.chars_indices.first().unwrap().0;
    let end = num.chars_indices.last().unwrap().0;

    let bounds: (usize, usize, usize, usize) = (
        // Top
        if num.line == 0 { 0 } else { num.line - 1 },
        // Right
        if end == matrix_w - 1 { end } else { end + 1 },
        // Bottom
        if num.line == matrix_h - 1 {
            num.line
        } else {
            num.line + 1
        },
        // Left
        if start == 0 { 0 } else { start - 1 },
    );

    for i in bounds.0..bounds.2 + 1 {
        let line_chars = matrix.get(i).unwrap();
        if line_chars[bounds.3..bounds.1 + 1]
            .iter()
            .any(|c| is_symbol(*c))
        {
            return true;
        }
    }

    false
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

#[test]
fn part1_example() {
    let res = part1(aoc23::parse_lines(
        r#"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56"#,
    ));
    assert_eq!(res, 925);
}

#[test]
fn part2_example() {
    let res = part2(aoc23::parse_lines(
        r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
    ));
    assert_eq!(res, 467835);
}
