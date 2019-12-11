use aoc::day1;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;

    if let Ok(lines) = read_lines("data/day1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mass: u32 = ip.parse().unwrap();
                sum += day1::compute_fuel(mass);
                sum2 += day1::compute_fuel2(mass);
            }
        }
    }

    println!("Day 1 Part 1");
    println!("Fuel requirement: {}", sum);

    println!("Day 1 Part 2");
    println!("Fuel requirement: {}", sum2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
