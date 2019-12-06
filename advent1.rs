use std::fs;

fn get_fuel(mut fuel: i32) -> i32 {
    let mut res = 0;
    while fuel / 3 - 2 >= 0 {
        fuel = fuel / 3 - 2;
        res += fuel;
    }
    res
}

fn main() -> Result<(), std::io::Error> {
    let text = fs::read_to_string("inputs/1-1.txt")?;
    let res : i32 = text.lines().map(|line| line.parse::<i32>().unwrap()).map(get_fuel).sum();
    println!("{}", res);
    Ok(())
}