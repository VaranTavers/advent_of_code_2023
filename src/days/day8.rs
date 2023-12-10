use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();

    let ways = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let _ = lines.next();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line_r in lines {
        let line_str = line_r.unwrap();
        let line = line_str.split(" = ").collect::<Vec<&str>>();

        let left_right = line[1]
            .split(',')
            .map(|x| x.trim().replace([')', '('], ""))
            .collect::<Vec<String>>();

        map.insert(
            line[0].to_owned(),
            (left_right[0].clone(), left_right[1].clone()),
        );
    }

    let mut moves = 0;
    let mut start = "AAA".to_owned();
    while start != "ZZZ" {
        if ways[moves % ways.len()] == 'L' {
            start = map.get(&start).unwrap().0.clone();
        } else {
            start = map.get(&start).unwrap().1.clone();
        }
        moves += 1;
    }

    Ok(moves)
}

fn lcm(num1: usize, num2: &usize) -> usize {
    let mut n1 = num1;
    let mut n2 = *num2;

    while n1 % n2 != 0 {
        let m = n1 % n2;
        n1 = n2;
        n2 = m;
    }

    num1 * num2 / n2
}

fn find_cycle(
    ways: &Vec<char>,
    map: &HashMap<String, (String, String)>,
    start: &str,
) -> (usize, usize) {
    let mut moves = 0;
    let mut start = start.to_owned();
    while !start.ends_with('Z') {
        if ways[moves % ways.len()] == 'L' {
            start = map.get(&start).unwrap().0.clone();
        } else {
            start = map.get(&start).unwrap().1.clone();
        }
        moves += 1;
    }

    let phase = moves % ways.len();
    let first = moves;

    if ways[moves % ways.len()] == 'L' {
        start = map.get(&start).unwrap().0.clone();
    } else {
        start = map.get(&start).unwrap().1.clone();
    }
    moves += 1;

    while !start.ends_with('Z') || phase != moves % ways.len() {
        if ways[moves % ways.len()] == 'L' {
            start = map.get(&start).unwrap().0.clone();
        } else {
            start = map.get(&start).unwrap().1.clone();
        }
        moves += 1;
    }

    (first, moves - first)
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();

    let ways = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let _ = lines.next();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut starts = Vec::new();

    for line_r in lines {
        let line_str = line_r.unwrap();
        let line = line_str.split(" = ").collect::<Vec<&str>>();
        if line[0].ends_with('A') {
            starts.push(line[0].to_owned());
        }
        let left_right = line[1]
            .split(',')
            .map(|x| x.trim().replace([')', '('], ""))
            .collect::<Vec<String>>();

        map.insert(
            line[0].to_owned(),
            (left_right[0].clone(), left_right[1].clone()),
        );
    }

    let cylces = starts
        .iter()
        .map(|x| find_cycle(&ways, &map, x))
        .collect::<Vec<(usize, usize)>>();

    Ok(cylces.iter().map(|x| &x.1).fold(1, lcm))
}
