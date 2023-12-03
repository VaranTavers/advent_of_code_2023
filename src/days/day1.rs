use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const S_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn word_into_i64(word: &str) -> i64 {
    S_DIGITS.iter().position(|x| *x == word).unwrap_or(0) as i64
}

pub fn get_first_digit_word(line: &str) -> (usize, i64) {
    S_DIGITS
        .iter()
        .filter_map(|x| line.find(x).map(|y| (x, y)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .map(|(x, y)| (y + 1, word_into_i64(x)))
        .unwrap_or((usize::MAX, -1))
}

pub fn get_last_digit_word(line: &str) -> (usize, i64) {
    S_DIGITS
        .iter()
        .filter_map(|x| line.rfind(x).map(|y| (x, y)))
        .max_by(|x, y| x.1.cmp(&y.1))
        .map(|(x, y)| (y + 1, word_into_i64(x)))
        .unwrap_or((usize::MIN, -1))
}

pub fn get_first_digit(line: &str) -> (usize, i64) {
    let pos = line.chars().position(|x| x.is_numeric());

    match pos {
        Some(pos) => (pos + 1, line.chars().nth(pos).unwrap() as i64 - '0' as i64),
        None => (usize::MAX, -1),
    }
}

pub fn get_last_digit(line: &str) -> (usize, i64) {
    let pos = line
        .chars()
        .rev()
        .position(|x| x.is_numeric())
        .map(|x| line.len() - 1 - x);

    match pos {
        Some(pos) => (pos + 1, line.chars().nth(pos).unwrap() as i64 - '0' as i64),
        None => (usize::MIN, -1),
    }
}

pub fn get_number_from_line(line: &str) -> i64 {
    let f_digit = [get_first_digit(line), get_first_digit_word(line)]
        .iter()
        .min_by(|x, y| x.0.cmp(&y.0))
        .cloned()
        .unwrap();
    let l_digit = [get_last_digit(line), get_last_digit_word(line)]
        .iter()
        .max_by(|x, y| x.0.cmp(&y.0))
        .cloned()
        .unwrap();

    f_digit.1 * 10 + l_digit.1
}

pub fn solution() -> Result<i64, std::io::Error> {
    let f = File::open("input1")?;
    let reader = BufReader::new(f);

    Ok(reader
        .lines()
        .map(|x| get_number_from_line(&x.unwrap()))
        .sum())
}
