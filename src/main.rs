use std::env;
mod solutions;
use crate::solutions::Solution;

struct Args {
    day: i32,
    input_file: String,
}

fn parse_args(mut acc: Args, args: &[String]) -> Args {
    assert_eq!(args.len(), 2);
    match args[0].as_str() {
        "-d" | "--day" => acc.day = args[1].parse::<i32>().unwrap(),
        "-i" | "--input" => acc.input_file = args[1].clone(),
        _ => println!("Unknown argument {:?}", &args[0])
    }
    acc
}

fn main() {
    let arg_list : Vec<_> = env::args().collect();
    let arguments = Args{
        day: 0,
        input_file: "".to_string()
    };

    let args: Args = arg_list[1..].chunks(2).fold(arguments, parse_args);
    match args.day {
        0 => solutions::Day0{solution: "Day 0?"}.solve(),
        1 => solutions::Day1{}.solve(),
        2 => solutions::Day2{}.solve(),
        3 => solutions::Day3{}.solve(),
        4 => solutions::Day4{}.solve(),
        5 => solutions::Day5{}.solve(),
        6 => solutions::Day6{}.solve(),
        _ => println!("No solution for day {}", args.day)
    }
}
