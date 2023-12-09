use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn predict_next(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut new = Vec::new();

    numbers.windows(2).for_each(|x| new.push(x[1] - x[0]));

    numbers[numbers.len() - 1] + predict_next(&new)
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let lines = reader
        .lines()
        .flatten()
        .map(|line| line.split(' ').map(|y| y.parse::<i64>().unwrap()).collect())
        .collect::<Vec<Vec<i64>>>();

    Ok(lines.iter().map(|x| predict_next(x)).sum())
}

fn predict_prev(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut new = Vec::new();

    numbers.windows(2).for_each(|x| new.push(x[1] - x[0]));

    numbers[0] - predict_prev(&new)
}

pub fn solution_2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let lines = reader
        .lines()
        .flatten()
        .map(|line| line.split(' ').map(|y| y.parse::<i64>().unwrap()).collect())
        .collect::<Vec<Vec<i64>>>();

    Ok(lines.iter().map(|x| predict_prev(x)).sum())
}
