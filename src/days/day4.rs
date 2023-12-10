use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn chars_to_nums(chars: &str) -> Vec<i64> {
    let mut res = Vec::new();

    let mut num = 0;

    for (i, c) in chars.chars().enumerate() {
        match i % 3 {
            0 => num += c.to_digit(10).unwrap_or(0) * 10,
            1 => num += c.to_digit(10).unwrap_or(0),
            _ => {
                res.push(num as i64);
                num = 0;
            }
        }
    }
    if num != 0 {
        res.push(num as i64);
    }

    res
}

fn process_line(line: &str) -> i64 {
    let mid_split = line.split(" | ").collect::<Vec<&str>>();

    let game_split = mid_split[0].split(": ").collect::<Vec<&str>>();

    let card_correct = chars_to_nums(game_split[1]);

    let card_elf = chars_to_nums(mid_split[1]);

    card_elf
        .iter()
        .filter(|x| card_correct.contains(x))
        .fold(1, |acc, _x| acc * 2)
        / 2
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let s = reader.lines().map(|x| process_line(&x.unwrap())).sum();

    Ok(s)
}

fn process_line_2(line: &str) -> usize {
    let mid_split = line.split(" | ").collect::<Vec<&str>>();

    let game_split = mid_split[0].split(": ").collect::<Vec<&str>>();

    let card_correct = chars_to_nums(game_split[1]);

    let card_elf = chars_to_nums(mid_split[1]);

    card_elf.iter().filter(|x| card_correct.contains(x)).count()
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let s = reader
        .lines()
        .map(|x| process_line_2(&x.unwrap()))
        .collect::<Vec<usize>>();

    let mut numbers = vec![1; s.len()];

    for (i, win) in s.iter().enumerate() {
        for j in i + 1..(i + 1 + win).min(s.len()) {
            numbers[j] += win.min(&1) * numbers[i];
        }
    }

    Ok(numbers.iter().sum())
}
