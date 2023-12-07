use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::days::{day1, day2, day3, day4, day5, day6, day7};

mod days;

fn main() -> Result<(), io::Error> {
    let day = 7;
    let part = 2;

    let f = File::open(format!("inputs/input{}", day))?;
    let reader = BufReader::new(f);

    print!("Day {day} (Part: {part}): ");
    match part {
        1 => {
            match day {
                1 => println!("{}", day1::solution(reader).unwrap()),
                2 => println!("{}", day2::solution_1(reader).unwrap()),
                3 => println!("{}", day3::solution(reader).unwrap()),
                4 => println!("{}", day4::solution(reader).unwrap()),
                5 => println!("{}", day5::solution(reader).unwrap()),
                6 => println!("{}", day6::solution(reader).unwrap()),
                7 => println!("{}", day7::solution(reader).unwrap()),
                _ => println!("What?"),
            };
        }
        _ => {
            match day {
                1 => println!("{}", day1::solution(reader).unwrap()),
                2 => println!("{}", day2::solution_2(reader).unwrap()),
                3 => println!("{}", day3::solution_2(reader).unwrap()),
                4 => println!("{}", day4::solution_2(reader).unwrap()),
                5 => println!("{}", day5::solution_2(reader).unwrap()),
                6 => println!("{}", day6::solution_2(reader).unwrap()),
                7 => println!("{}", day7::solution_2(reader).unwrap()),
                _ => println!("What?"),
            };
        }
    };

    Ok(())
}
