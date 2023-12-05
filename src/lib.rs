pub fn parse_input(day: usize) -> Vec<String> {
    let input_str = std::fs::read_to_string(format!("inputs/{}.txt", day))
        .unwrap_or_else(|_| panic!("Could not read file {}.txt", day));
    input_str.lines().map(|s| s.to_string()).collect()
}
