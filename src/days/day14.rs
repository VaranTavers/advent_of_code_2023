use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn tilt_north(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut empty = vec![0; map[0].len()];
    let mut new_map = map.clone();

    for (line_i, line) in map.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            if c == &'.' {
                empty[i] += 1;
            } else if c == &'#' {
                empty[i] = 0;
            } else {
                new_map[line_i][i] = '.';
                new_map[line_i - empty[i]][i] = 'O';
            }
        }
    }

    new_map
}

pub fn load_north(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(i, line)| (line.iter().filter(|x| x == &&'O').count()) * (map.len() - i))
        .sum()
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let map = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let map = tilt_north(&map);

    Ok(load_north(&map))
}

pub fn tilt_south(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut empty = vec![0; map[0].len()];
    let mut new_map = map.clone();

    for (line_i, line) in map.iter().enumerate().rev() {
        for (i, c) in line.iter().enumerate() {
            if c == &'.' {
                empty[i] += 1;
            } else if c == &'#' {
                empty[i] = 0;
            } else {
                new_map[line_i][i] = '.';
                new_map[line_i + empty[i]][i] = 'O';
            }
        }
    }

    new_map
}

pub fn tilt_east(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut empty = vec![0; map.len()];
    let mut new_map = map.clone();

    for i in (0..map[0].len()).rev() {
        for line_i in 0..map.len() {
            let c = map[line_i][i];
            if c == '.' {
                empty[line_i] += 1;
            } else if c == '#' {
                empty[line_i] = 0;
            } else {
                println!("{:?}", empty);
                println!("{i}");
                new_map[line_i][i] = '.';
                new_map[line_i][i + empty[line_i]] = 'O';
            }
        }
    }

    new_map
}

pub fn tilt_west(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut empty = vec![0; map.len()];
    let mut new_map = map.clone();

    for i in 0..map[0].len() {
        for line_i in 0..map.len() {
            let c = map[line_i][i];
            if c == '.' {
                empty[line_i] += 1;
            } else if c == '#' {
                empty[line_i] = 0;
            } else {
                new_map[line_i][i] = '.';
                new_map[line_i][i - empty[line_i]] = 'O';
            }
        }
    }

    new_map
}

pub fn cycle(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let map = tilt_north(&map);

    let map: Vec<Vec<char>> = tilt_west(&map);

    let map: Vec<Vec<char>> = tilt_south(&map);

    let map: Vec<Vec<char>> = tilt_east(&map);

    map
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let map = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut loads: Vec<usize> = Vec::new();
    let mut i = 0;
    let mut maps: Vec<Vec<Vec<char>>> = Vec::new();
    let mut map = cycle(map);
    let mut new_load = load_north(&map);
    while !maps.iter().any(|x| x == &map) && i < 1_000_000_000 {
        loads.push(new_load);
        maps.push(map.clone());
        map = cycle(map);
        new_load = load_north(&map);
        i += 1;
    }

    let old_pos = maps.iter().position(|x| x == &map).unwrap();
    let cur_pos = maps.len();
    let diff = cur_pos - old_pos;
    let modd = (1_000_000_000 - old_pos) % diff;

    for (map, load) in maps.iter().zip(loads.iter()) {
        for line in map.iter() {
            for c in line.iter() {
                print!("{c}");
            }
            println!();
        }
        println!("LOAD={load}");
    }

    Ok(load_north(&maps[old_pos + modd - 1]))
}
