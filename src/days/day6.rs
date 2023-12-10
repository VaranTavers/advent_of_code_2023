use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_one((time, record): (usize, usize)) -> usize {
    // (m - x)*x > record
    // x^2 - mx + record < 0
    // d = b^2 - 4ac = m^2-4record
    // x_1 = -b - sqrt(d) / 2a
    // x_2 = -b + sqrt(d) / 2a
    let delta = time.pow(2) - 4 * record;
    let mut x1 = ((time as f64 - (delta as f64).sqrt()) / 2.0).ceil() as usize;
    let mut x2 = ((time as f64 + (delta as f64).sqrt()) / 2.0).floor() as usize;
    if (time - x1) * x1 == record {
        x1 += 1;
    }
    if (time - x2) * x2 == record {
        x2 -= 1;
    }
    println!("{time} - {record}: d: {delta} x1: {x1}, x2: {x2}");

    x2.max(x1) - x1.min(x2) + 1
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader.lines().flatten().collect::<Vec<String>>();

    let times_line = lines[0].split(':').collect::<Vec<&str>>();

    let times = times_line[1]
        .trim()
        .split(' ')
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<usize>>();

    let records_line = lines[1].split(':').collect::<Vec<&str>>();

    let records = records_line[1]
        .trim()
        .split(' ')
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<usize>>();

    println!("{:?}", times);
    println!("{:?}", records);

    let calcs = times
        .iter()
        .zip(records.iter())
        .map(|(x, y)| calculate_one((*x, *y)))
        .collect::<Vec<usize>>();

    println!("{:?}", calcs);

    Ok(calcs.iter().fold(1, |x, y| x * (*y)))
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader.lines().flatten().collect::<Vec<String>>();

    let times_line = lines[0].split(':').collect::<Vec<&str>>();

    let times = times_line[1].replace(' ', "").parse::<usize>();

    let records_line = lines[1].split(':').collect::<Vec<&str>>();

    let records = records_line[1].replace(' ', "").parse::<usize>();

    println!("{:?}", times);
    println!("{:?}", records);

    let calcs = calculate_one((times.unwrap(), records.unwrap()));

    println!("{:?}", calcs);

    Ok(calcs)
}
