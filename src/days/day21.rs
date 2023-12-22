use std::{collections::VecDeque, fs::File, io::BufReader};

use crate::utils::CharMap;

pub fn simulate_moves(map: &CharMap, max_steps: usize) -> Vec<Vec<bool>> {
    let mut visited = map.map_to_val(false);
    let mut reachable = map.map_to_val(false);

    let start_point = map.find_first('S').unwrap();

    let mut v: VecDeque<(usize, usize, usize)> = VecDeque::new();
    v.push_back((start_point.0, start_point.1, 0));

    visited[start_point.0][start_point.1] = true;

    while let Some((row, col, steps)) = v.pop_front() {
        if max_steps < steps {
            break;
        }
        if steps % 2 == max_steps % 2 {
            reachable[row][col] = true;
        }
        if row > 0 && map.map[row - 1][col] != '#' && !visited[row - 1][col] {
            v.push_back((row - 1, col, steps + 1));
            visited[row - 1][col] = true;
        }
        if row < map.map.len() - 1 && map.map[row + 1][col] != '#' && !visited[row + 1][col] {
            v.push_back((row + 1, col, steps + 1));
            visited[row + 1][col] = true;
        }
        if col > 0 && map.map[row][col - 1] != '#' && !visited[row][col - 1] {
            v.push_back((row, col - 1, steps + 1));
            visited[row][col - 1] = true;
        }
        if col < map.map[row].len() - 1 && map.map[row][col + 1] != '#' && !visited[row][col + 1] {
            v.push_back((row, col + 1, steps + 1));
            visited[row][col + 1] = true;
        }
    }

    reachable
}

pub fn pprint(reachable: &Vec<Vec<bool>>) {
    for row in 0..reachable.len() {
        if row % 3 == 0 {
            println!();
        }
        for col in 0..reachable[row].len() {
            if col % 3 == 0 {
                print!(" ");
            }
            if reachable[row][col] {
                print!("0");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let map = CharMap::parse_map(reader);

    let reachable = simulate_moves(&map, 19); //64);

    pprint(&reachable);

    Ok(reachable
        .iter()
        .map(|x| x.iter().filter(|y| **y).count())
        .sum())
}

pub fn count_moves(reachable: &Vec<Vec<bool>>) -> usize {
    reachable
        .iter()
        .map(|x| x.iter().filter(|y| **y).count())
        .sum()
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let map = CharMap::parse_map(reader);

    const MAX_STEPS: usize = 26501365;

    let width = map.map[0].len();

    let start_point = map.find_first('S').unwrap();

    //Has to be simulated to width * 2 moves, cannot be calculated!!!
    let width_to_use = if MAX_STEPS % 2 == 0 { width + 1 } else { width };

    let even_visit = count_moves(&simulate_moves(&map, width_to_use));
    let odd_visit = count_moves(&simulate_moves(&map, width_to_use + 1));

    let mut map_ver = map.clone();
    map_ver.map[start_point.0][start_point.1] = '.';
    map_ver.map[map.map.len() - 1][0] = 'S';
    /*let q1_m = simulate_moves(&map, width / 2 - 1);
    pprint(&q1_m);*/
    let q1_visit = count_moves(&simulate_moves(&map_ver, width / 2 - 1));

    map_ver.map[map.map.len() - 1][0] = '.';
    map_ver.map[map.map.len() - 1][map.map[map.map.len() - 1].len() - 1] = 'S';
    let q2_visit = count_moves(&simulate_moves(&map_ver, width / 2 - 1));

    map_ver.map[map.map.len() - 1][map.map[map.map.len() - 1].len() - 1] = '.';
    map_ver.map[0][map.map[0].len() - 1] = 'S';
    let q3_visit = count_moves(&simulate_moves(&map_ver, width / 2 - 1));

    map_ver.map[0][map.map[0].len() - 1] = '.';
    map_ver.map[0][0] = 'S';
    let q4_visit = count_moves(&simulate_moves(&map_ver, width / 2 - 1));

    println!("{q1_visit} {q2_visit} {q3_visit} {q4_visit}");
    let maps_left = (MAX_STEPS - width / 2) / width;
    //let maps_left_rest = (MAX_STEPS - width / 2) % width;
    //let maps_quart_rest = width / 2 - 1;

    let mut sum = even_visit;

    println!("{sum} maps_left {maps_left}");

    for vals in 1..(maps_left + 1) {
        //println!("Sajat Sor ({maps_left}):");
        if vals % 2 == 1 {
            sum += 2 * odd_visit;
        } else {
            sum += 2 * even_visit;
        }
        //println!("{sum}");
    }

    let diff = 4 * odd_visit + 4 * even_visit;
    let start_sum;
    let second_sum;
    if maps_left % 2 == 0 {
        let start = 2 * even_visit + q1_visit + q2_visit + q3_visit + q4_visit;
        let second = 2 * odd_visit + 4 * even_visit + q1_visit + q2_visit + q3_visit + q4_visit;
        start_sum = (start + start + ((maps_left / 2).max(1) - 1) * diff) * (maps_left / 2) / 2;
        second_sum = (second + second + ((maps_left / 2).max(1) - 1) * diff) * (maps_left / 2) / 2;
        println!("{start} {second} {start_sum} {second_sum}");
    } else {
        let start = 2 * odd_visit + q1_visit + q2_visit + q3_visit + q4_visit;
        let second = 2 * even_visit + 4 * odd_visit + q1_visit + q2_visit + q3_visit + q4_visit;
        start_sum = (start + start + (maps_left / 2) * diff) * (maps_left / 2 + 1) / 2;
        second_sum = (second + second + ((maps_left / 2).max(1) - 1) * diff) * (maps_left / 2) / 2;
        println!("{start} {second} {start_sum} {second_sum}");
    }

    sum += start_sum + second_sum;

    let mut map_ver = map.clone();
    map_ver.map[start_point.0][start_point.1] = '.';
    map_ver.map[map.map.len() - 1][start_point.1] = 'S';
    /*let q1_m = simulate_moves(&map, width / 2 - 1);
    pprint(&q1_m);*/
    let q1_visit = count_moves(&simulate_moves(&map_ver, width - 1));

    map_ver.map[map.map.len() - 1][start_point.1] = '.';
    map_ver.map[0][start_point.1] = 'S';
    let q2_visit = count_moves(&simulate_moves(&map_ver, width - 1));

    map_ver.map[0][start_point.1] = '.';
    map_ver.map[start_point.0][map.map[start_point.0].len() - 1] = 'S';
    let q3_visit = count_moves(&simulate_moves(&map_ver, width - 1));

    map_ver.map[start_point.0][map.map[start_point.0].len() - 1] = '.';
    map_ver.map[start_point.0][0] = 'S';
    let q4_visit = count_moves(&simulate_moves(&map_ver, width - 1));

    println!(
        "{even_visit} {odd_visit} Lent: {q1_visit} Fent: {q2_visit} Jobb {q3_visit} Bal {q4_visit}"
    );

    if maps_left % 2 == 0 {
        //Diamond - Corners
        sum -= even_visit * 4 - q1_visit - q2_visit - q3_visit - q4_visit;
    } else {
        //Corners
        sum -= odd_visit * 4 - q1_visit - q2_visit - q3_visit - q4_visit;
    }

    let mut map_ver = map.clone();
    map_ver.map[start_point.0][start_point.1] = '.';
    map_ver.map[map.map.len() - 1][0] = 'S';
    /*let q1_m = simulate_moves(&map, width / 2 - 1);
    pprint(&q1_m);*/
    let q1_visit = count_moves(&simulate_moves(&map_ver, width + width / 2 - 1));

    map_ver.map[map.map.len() - 1][0] = '.';
    map_ver.map[map.map.len() - 1][map.map[map.map.len() - 1].len() - 1] = 'S';
    let q2_visit = count_moves(&simulate_moves(&map_ver, width + width / 2 - 1));

    map_ver.map[map.map.len() - 1][map.map[map.map.len() - 1].len() - 1] = '.';
    map_ver.map[0][map.map[0].len() - 1] = 'S';
    let q3_visit = count_moves(&simulate_moves(&map_ver, width + width / 2 - 1));

    map_ver.map[0][map.map[0].len() - 1] = '.';
    map_ver.map[0][0] = 'S';
    let q4_visit = count_moves(&simulate_moves(&map_ver, width + width / 2 - 1));

    println!("{q1_visit} {q2_visit} {q3_visit} {q4_visit}");

    // Got inspiration
    if maps_left % 2 == 0 {
        //Diamond - Corners
        sum -= (maps_left - 1) * (even_visit * 4 - q1_visit - q2_visit - q3_visit - q4_visit);
    } else {
        //Corners
        sum -= (maps_left - 1) * (odd_visit * 4 - q1_visit - q2_visit - q3_visit - q4_visit);
    }

    Ok(sum)
}
