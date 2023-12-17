use std::{collections::VecDeque, fs::File, io::BufReader};

use crate::utils::CharMap;

pub fn ray_trace(map: &CharMap, start: ((i32, i32), (i32, i32))) -> usize {
    let mut energy = map.map_to_val(false);
    let mut from_way = map.clone_to_val(vec![false, false, false, false]);

    let mut lights: VecDeque<((i32, i32), (i32, i32))> = VecDeque::new();

    lights.push_back(start);

    while lights.len() > 0 {
        let light = lights.pop_front().unwrap();
        let row = light.0 .0;
        let col = light.0 .1;
        let way_index = if light.1 .0 < 0 {
            0
        } else if light.1 .0 > 0 {
            1
        } else if light.1 .1 < 0 {
            2
        } else {
            3
        };

        if row < 0
            || row as usize >= map.map.len()
            || col as usize >= map.map[0].len()
            || col < 0
            || from_way[row as usize][col as usize][way_index]
        {
            continue;
        }

        let c = map.map[light.0 .0 as usize][light.0 .1 as usize];
        energy[light.0 .0 as usize][light.0 .1 as usize] = true;
        from_way[row as usize][col as usize][way_index] = true;
        let way;
        match c {
            '.' => {
                way = light.1;
            }
            '/' => {
                way = (0 - light.1 .1, 0 - light.1 .0);
            }
            '\\' => {
                way = (light.1 .1, light.1 .0);
            }
            '-' => {
                if light.1 .0 == 0 {
                    way = light.1;
                } else {
                    let way2 = (light.1 .1, light.1 .0);
                    lights.push_back(((light.0 .0 + way2.0, light.0 .1 + way2.1), way2));
                    way = (0 - light.1 .1, 0 - light.1 .0);
                }
            }
            '|' => {
                if light.1 .1 == 0 {
                    way = light.1;
                } else {
                    let way2 = (light.1 .1, light.1 .0);
                    lights.push_back(((light.0 .0 + way2.0, light.0 .1 + way2.1), way2));
                    way = (0 - light.1 .1, 0 - light.1 .0);
                }
            }
            _ => {
                way = (1000, 1000);
                eprintln!("WHAT?");
            }
        }
        lights.push_back(((light.0 .0 + way.0, light.0 .1 + way.1), way));
    }

    energy
        .iter()
        .map(|x| x.iter().filter(|x| **x).count())
        .sum()
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let map = CharMap::parse_map(reader);

    Ok(ray_trace(&map, ((0, 0), (0, 1))))
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let map = CharMap::parse_map(reader);

    let mut max = 0;

    for row in 0..map.map.len() {
        let val = ray_trace(&map, ((row as i32, 0), (0, 1)));
        if val > max {
            max = val;
        }
        let val = ray_trace(&map, ((row as i32, map.map[row].len() as i32 - 1), (0, -1)));
        if val > max {
            max = val;
        }
    }

    for col in 0..map.map[0].len() {
        let val = ray_trace(&map, ((0, col as i32), (1, 0)));
        if val > max {
            max = val;
        }
        let val = ray_trace(&map, ((map.map.len() as i32 - 1, col as i32), (-1, 0)));
        if val > max {
            max = val;
        }
    }

    Ok(max)
}
