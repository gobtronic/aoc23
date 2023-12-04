pub fn parse_input(day: usize, part: u8) -> Vec<String> {
    let input_str = std::fs::read_to_string(format!("inputs/{}_{}.txt", day, part))
        .unwrap_or_else(|_| panic!("Could not read file {}_{}.txt", day, part));
    input_str.lines().map(|s| s.to_string()).collect()
}
