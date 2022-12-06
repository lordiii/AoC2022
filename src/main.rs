extern crate core;

mod day_01;
mod day_02;

use clap::Parser;

/// Solutions for advent of code 2022
#[derive(Parser)]
struct Cli {
    /// Day of advent of code (1-24)
    day: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let day: &u8 = &args.day;

    println!("Getting results for day {}", day);

    let path: String = format!("./days/day_{:02}.txt", day);
    let file_contents: std::io::Result<String> = std::fs::read_to_string(format!("./days/day_{:02}.txt", day));
    let puzzle_input: String = file_contents.expect(&format!("{} {}", path, "not found"));

    match day {
        1 => day_01::main(&puzzle_input),
        2 => day_02::main(&puzzle_input),
        _ => println!("Could not find day!")
    }

    println!("Done");

    return Ok(());
}