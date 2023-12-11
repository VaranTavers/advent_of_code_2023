use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Debug)]
enum From {
    BOTTOM,
    LEFT,
    TOP,
    RIGHT,
}

fn handle_start(
    map: &mut Vec<Vec<char>>,
    pos: (usize, usize),
    points: &mut VecDeque<(usize, usize, usize, From)>,
) {
    if pos.0 > 0 {
        let c = map[pos.0 - 1][pos.1];

        if c == '|' || c == '7' || c == 'F' {
            points.push_back((pos.0 - 1, pos.1, 1, From::BOTTOM));
        }
    }

    if pos.1 > 0 {
        let c = map[pos.0][pos.1 - 1];

        if c == '-' || c == 'L' || c == 'F' {
            points.push_back((pos.0, pos.1 - 1, 1, From::RIGHT));
        }
    }
    if pos.0 < map.len() - 1 {
        let c = map[pos.0 + 1][pos.1];

        if c == '|' || c == 'J' || c == 'L' {
            points.push_back((pos.0 + 1, pos.1, 1, From::TOP));
        }
    }

    if pos.1 < map[pos.0].len() - 1 {
        let c = map[pos.0][pos.1 + 1];

        if c == '-' || c == 'J' || c == '7' {
            points.push_back((pos.0, pos.1 + 1, 1, From::LEFT));
        }
    }

    if points[0].3 == From::BOTTOM {
        if points[1].3 == From::RIGHT {
            map[pos.0][pos.1] = 'J';
        } else if points[1].3 == From::TOP {
            map[pos.0][pos.1] = '|';
        } else {
            map[pos.0][pos.1] = 'L';
        }
    } else if points[0].3 == From::RIGHT {
        if points[1].3 == From::TOP {
            map[pos.0][pos.1] = '7';
        } else {
            map[pos.0][pos.1] = '-';
        }
    } else {
        map[pos.0][pos.1] = 'F';
    }

    println!("{}", map[pos.0][pos.1]);
}

pub fn flow(map: &mut Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    let mut points = VecDeque::new();

    handle_start(map, pos, &mut points);

    while let Some((row, col, num, way)) = points.pop_front() {
        if row == points[0].0 && col == points[0].1 {
            return num.min(points[0].2);
        }

        let c = map[row][col];
        //println!("{row} {col} {num} {:?}: {}", way, c);

        match c {
            '|' => {
                if way == From::BOTTOM {
                    points.push_back((row - 1, col, num + 1, way));
                } else {
                    points.push_back((row + 1, col, num + 1, way));
                }
            }
            '-' => {
                if way == From::LEFT {
                    points.push_back((row, col + 1, num + 1, way));
                } else {
                    points.push_back((row, col - 1, num + 1, way));
                }
            }
            'J' => {
                if way == From::TOP {
                    points.push_back((row, col - 1, num + 1, From::RIGHT));
                } else {
                    points.push_back((row - 1, col, num + 1, From::BOTTOM));
                }
            }
            'L' => {
                if way == From::TOP {
                    points.push_back((row, col + 1, num + 1, From::LEFT));
                } else {
                    points.push_back((row - 1, col, num + 1, From::BOTTOM));
                }
            }
            '7' => {
                if way == From::BOTTOM {
                    points.push_back((row, col - 1, num + 1, From::RIGHT));
                } else {
                    points.push_back((row + 1, col, num + 1, From::TOP));
                }
            }
            'F' => {
                if way == From::BOTTOM {
                    points.push_back((row, col + 1, num + 1, From::LEFT));
                } else {
                    points.push_back((row + 1, col, num + 1, From::TOP));
                }
            }
            _ => panic!("What? {}", c),
        }
    }

    0
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut map = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let start_pos = map
        .iter()
        .enumerate()
        .flat_map(|(x, y)| y.iter().position(|z| z == &'S').map(|z| (x, z)))
        .collect::<Vec<(usize, usize)>>()[0];

    Ok(flow(&mut map, start_pos))
}

pub fn partition(map: &mut Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    let mut points = VecDeque::new();

    let mut map2 = vec![vec![0; map[0].len()]; map.len()];

    let mut map3 = vec![vec![false; map[0].len()]; map.len()];

    handle_start(map, pos, &mut points);

    map2[pos.0][pos.1] = 1;

    while let Some((row, col, num, way)) = points.pop_front() {
        let c = map[row][col];

        map2[row][col] = 1;

        if row == points[0].0 && col == points[0].1 {
            break;
        }
        //println!("{row} {col} {num} {:?}: {}", way, c);

        match c {
            '|' => {
                if way == From::BOTTOM {
                    points.push_back((row - 1, col, num + 1, way));
                } else {
                    points.push_back((row + 1, col, num + 1, way));
                }
            }
            '-' => {
                if way == From::LEFT {
                    points.push_back((row, col + 1, num + 1, way));
                } else {
                    points.push_back((row, col - 1, num + 1, way));
                }
            }
            'J' => {
                if way == From::TOP {
                    points.push_back((row, col - 1, num + 1, From::RIGHT));
                } else {
                    points.push_back((row - 1, col, num + 1, From::BOTTOM));
                }
            }
            'L' => {
                if way == From::TOP {
                    points.push_back((row, col + 1, num + 1, From::LEFT));
                } else {
                    points.push_back((row - 1, col, num + 1, From::BOTTOM));
                }
            }
            '7' => {
                if way == From::BOTTOM {
                    points.push_back((row, col - 1, num + 1, From::RIGHT));
                } else {
                    points.push_back((row + 1, col, num + 1, From::TOP));
                }
            }
            'F' => {
                if way == From::BOTTOM {
                    points.push_back((row, col + 1, num + 1, From::LEFT));
                } else {
                    points.push_back((row + 1, col, num + 1, From::TOP));
                }
            }
            _ => panic!("What? {}", c),
        }
    }

    let mut is_in;

    for (i, cols) in map.iter().enumerate() {
        is_in = false;
        for (j, c) in cols.iter().enumerate() {
            if map2[i][j] == 1 {
                if c == &'|' || c == &'J' || c == &'L' {
                    is_in = !is_in;
                }
            } else {
                map3[i][j] = is_in;
            }
        }
    }

    let mut num = 0;
    for (i, cols) in map.iter().enumerate() {
        for (j, c) in cols.iter().enumerate() {
            if map3[i][j]
            /*&& map4[i][j] && map5[i][j] && map6[i][j]*/
            {
                println!("{i} {j}");
                num += 1;
            }
        }
    }

    num
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    // Solved, but took inspiration from a solution
    let mut map = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let start_pos = map
        .iter()
        .enumerate()
        .flat_map(|(x, y)| y.iter().position(|z| z == &'S').map(|z| (x, z)))
        .collect::<Vec<(usize, usize)>>()[0];

    Ok(partition(&mut map, start_pos))
}
