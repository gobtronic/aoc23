pub fn parse_input(day: usize) -> Vec<String> {
    let input_str = std::fs::read_to_string(format!("inputs/{}.txt", day))
        .unwrap_or_else(|_| panic!("Could not read file {}.txt", day));
    parse_lines(&input_str)
}

pub fn parse_lines(input: &str) -> Vec<String> {
    input.lines().map(|s: &str| s.to_string()).collect()
}
