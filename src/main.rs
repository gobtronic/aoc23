mod days;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "aoc23")]
#[command(author = "gobtronic")]
struct Cli {
    #[arg()]
    day: usize,
}

fn main() {
    let cli = Cli::parse();
    let days_fns = days_fns![day1, day2];

    match days_fns.len().cmp(&cli.day) {
        std::cmp::Ordering::Less => {
            println!("This day has not been solved yet :(");
            std::process::exit(1)
        }
        _ => {
            let fns = days_fns.get(cli.day - 1).unwrap();
            println!("{}", fns.0(aoc23::parse_input(cli.day)));
            println!("{}", fns.1(aoc23::parse_input(cli.day)));
        }
    }
}

#[macro_export]
macro_rules! days_fns {
    ($($day:ident),*) => {
        vec![
            $(
                (
                    days::$day::part1 as fn(input: Vec<String>) -> i64,
                    days::$day::part2 as fn(input: Vec<String>) -> i64,
                )
            ),*
        ]
    };
}
