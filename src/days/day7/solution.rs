use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader
        .lines()
        .flatten()
        .map(|x| super::model1::Hand::from_str(&x).unwrap())
        .collect::<Vec<super::model1::Hand>>();

    lines.sort();

    Ok(lines
        .iter()
        .enumerate()
        .fold(0, |acc, (num, hand)| acc + (num + 1) * hand.bet as usize))
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader
        .lines()
        .flatten()
        .map(|x| super::model2::Hand::from_str(&x).unwrap())
        .collect::<Vec<super::model2::Hand>>();

    lines.sort();

    Ok(lines
        .iter()
        .enumerate()
        .fold(0, |acc, (num, hand)| acc + (num + 1) * hand.bet as usize))
}
