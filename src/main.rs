use std::env;
mod solutions;
use crate::solutions::Solution;

struct Args {
    day: Option<i32>,
    input_file: Option<String>,
}

fn parse_args(mut acc: Args, args: &[String]) -> Args {
    assert_eq!(args.len(), 2);
    match args[0].as_str() {
        "-d" | "--day" => acc.day = args[1].parse::<i32>().ok(),
        "-i" | "--input" => acc.input_file = Some(args[1].clone()),
        _ => println!("Unknown argument {:?}", &args[0])
    }
    acc
}

fn main() {
    let arg_list : Vec<_> = env::args().collect();
    let arguments = Args{
        day: None,
        input_file: None
    };

    let args: Args = arg_list[1..].chunks(2).fold(arguments, parse_args);
    match args.day {
        None => solutions::Day0{solution: "Day 0?"}.solve(),
        Some(1) => solutions::Day1{}.solve(),
        Some(2) => solutions::Day2{}.solve(),
        Some(3) => solutions::Day3{}.solve(),
        Some(4) => solutions::Day4{}.solve(),
        Some(5) => solutions::Day5{}.solve(),
        Some(6) => solutions::Day6{}.solve(),
        Some(7) => solutions::Day7{}.solve(),
        Some(8) => solutions::Day8{
            file: args.input_file.unwrap_or(format!("inputs/8-1.txt"))
        }.solve(),
        Some(9) => solutions::Day9{
            file: args.input_file.unwrap_or(format!("inputs/9-1.txt"))
        }.solve(),
        Some(10) => solutions::Day10 {
            file: args.input_file.unwrap_or(format!("inputs/10-1.txt"))
        }.solve(),
        Some(i) => println!("No solution for day {}", i)
    }
}
