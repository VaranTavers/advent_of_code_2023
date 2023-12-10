use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct Mapper {
    pub rules: Vec<(usize, usize, usize)>,
}

impl Mapper {
    pub fn map_val(&self, num: usize) -> usize {
        // Could be binary search
        let vals = self
            .rules
            .iter()
            .find(|(_, y, z)| num >= *y && num < *y + *z);

        match vals {
            Some((x, y, _)) => num - y + x,
            None => num,
        }
    }

    pub fn new() -> Mapper {
        Mapper { rules: Vec::new() }
    }

    pub fn sort_rules(&mut self) {
        self.rules.sort_by(|(_, y1, _), (_, y2, _)| y1.cmp(y2))
    }

    pub fn parse_rule(&mut self, line: &str) {
        let nums = line
            .split(' ')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        self.rules.push((nums[0], nums[1], nums[2]))
    }

    pub fn get_inverse(&self) -> Mapper {
        Mapper {
            rules: self.rules.iter().map(|(x, y, z)| (*y, *x, *z)).collect(),
        }
    }
}

fn get_data(file: BufReader<File>) -> (Vec<usize>, Vec<Mapper>) {
    let mut lines = file.lines();

    let first_line = lines.next().unwrap().unwrap();
    let nums_str = first_line.split(':').collect::<Vec<&str>>();
    println!("{}", nums_str[1]);
    let seeds = nums_str[1]
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut mapper = Mapper::new();
    let mut mappers = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.contains("map:") {
            mappers.push(mapper);
            mapper = Mapper::new();
        } else if line.len() > 2 {
            mapper.parse_rule(&line);
        }
    }
    mappers.push(mapper);

    (seeds, mappers)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let (seeds, mappers) = get_data(reader);

    let results = mappers.iter().fold(seeds, |acc, mapper| {
        acc.iter().map(|x| mapper.map_val(*x)).collect()
    });

    Ok(*results.iter().min().unwrap())
}

fn list_to_pairs<T>(vec: Vec<T>) -> Vec<(T, T)> {
    let mut seed_intervals = Vec::new();
    let mut pair = None;

    for val in vec {
        match pair {
            Some(a) => {
                seed_intervals.push((a, val));
                pair = None;
            }
            None => {
                pair = Some(val);
            }
        }
    }

    seed_intervals
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let (seeds, mut mappers) = get_data(reader);

    mappers = mappers.iter_mut().rev().map(|x| x.get_inverse()).collect();

    let seeds = list_to_pairs(seeds);

    let mut i = 0;

    loop {
        if i % 10000 == 0 {
            println!("{}", i);
        }
        let x = mappers.iter().fold(i, |acc, mapper| mapper.map_val(acc));

        let vals = seeds.iter().find(|(y, z)| x >= *y && x < *y + *z);

        if vals.is_some() {
            return Ok(i);
        }

        i += 1;
    }
}
