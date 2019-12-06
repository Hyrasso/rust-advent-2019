use std::fs;


fn main() -> Result<(), std::io::Error> {
    let text = fs::read_to_string("inputs/2-1.txt")?;
    let mut program: Vec<_> = text.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    program[1] = 12;
    program[2] = 2;
    let mut ip = 0;
    loop {
        let dest = program[ip + 3] as usize;
        match program[ip] {
            1 => program[dest] = program[program[ip + 1] as usize] + program[program[ip + 2] as usize],
            2 => program[dest] = program[program[ip + 1] as usize] * program[program[ip + 2] as usize],
            99 => break,
            _ => println!("Unknown op code {}", program[ip])
        }
        ip += 4;
    }
    println!("{:?}", program);
    Ok(())
}