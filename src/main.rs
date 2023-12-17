use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::days::{
    day1, day10, day11, day12, day13, day14, day15, day16, day17, day2, day3, day4, day5, day6,
    day7, day8, day9,
};

mod days;
mod utils;

fn main() -> Result<(), io::Error> {
    let day = 17;
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
                8 => println!("{}", day8::solution(reader).unwrap()),
                9 => println!("{}", day9::solution(reader).unwrap()),
                10 => println!("{}", day10::solution(reader).unwrap()),
                11 => println!("{}", day11::solution(reader).unwrap()),
                12 => println!("{}", day12::solution(reader).unwrap()),
                13 => println!("{}", day13::solution(reader).unwrap()),
                14 => println!("{}", day14::solution(reader).unwrap()),
                15 => println!("{}", day15::solution(reader).unwrap()),
                16 => println!("{}", day16::solution(reader).unwrap()),
                17 => println!("{}", day17::solution(reader).unwrap()),
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
                8 => println!("{}", day8::solution_2(reader).unwrap()),
                9 => println!("{}", day9::solution_2(reader).unwrap()),
                10 => println!("{}", day10::solution_2(reader).unwrap()),
                11 => println!("{}", day11::solution_2(reader).unwrap()),
                12 => println!("{}", day12::solution_2(reader).unwrap()),
                13 => println!("{}", day13::solution_2(reader).unwrap()),
                14 => println!("{}", day14::solution_2(reader).unwrap()),
                15 => println!("{}", day15::solution_2(reader).unwrap()),
                16 => println!("{}", day16::solution_2(reader).unwrap()),
                17 => println!("{}", day17::solution_2(reader).unwrap()),
                _ => println!("What?"),
            };
        }
    };

    Ok(())
}
