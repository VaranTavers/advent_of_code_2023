use std::{
    collections::{BinaryHeap, VecDeque},
    fs::File,
    io::BufReader,
};

use crate::utils::{CharMap, From};

#[derive(Debug, PartialEq, Eq)]
pub struct Way {
    pub length: usize,
    pub row: usize,
    pub col: usize,
    pub last_few: VecDeque<From>,
}

impl PartialOrd for Way {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Way {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.length.cmp(&self.length)
    }
}

pub fn return_next_ways(last: &VecDeque<From>) -> Vec<From> {
    let mut res = Vec::new();

    if last.len() == 0
        || (last[0] != From::Bottom && last.iter().take(3).filter(|x| x == &&From::Top).count() < 3)
    {
        res.push(From::Top);
    }
    if last.len() > 0
        && (last[0] != From::Top && last.iter().take(3).filter(|x| x == &&From::Bottom).count() < 3)
    {
        res.push(From::Bottom);
    }
    if last.len() > 0
        && (last[0] != From::Left && last.iter().take(3).filter(|x| x == &&From::Right).count() < 3)
    {
        res.push(From::Right);
    }
    if last.len() == 0
        || (last[0] != From::Right && last.iter().take(3).filter(|x| x == &&From::Left).count() < 3)
    {
        res.push(From::Left);
    }

    res
}

pub fn get_next_coord(
    map: &Vec<Vec<u32>>,
    (row, col): (usize, usize),
    from: &From,
) -> Option<(usize, usize)> {
    let next_row;
    let next_col;

    match from {
        From::Bottom => {
            if row == 0 {
                return None;
            }
            next_row = row - 1;
            next_col = col;
        }
        From::Left => {
            if col == map[row].len() - 1 {
                return None;
            }
            next_row = row;
            next_col = col + 1;
        }
        From::Right => {
            if col == 0 {
                return None;
            }
            next_row = row;
            next_col = col - 1;
        }
        From::Top => {
            if row == map.len() - 1 {
                return None;
            }
            next_row = row + 1;
            next_col = col;
        }
    }

    Some((next_row, next_col))
}

pub fn find_way(map: &Vec<Vec<u32>>) -> usize {
    let mut visited = vec![vec![vec![vec![false; 4]; 4]; map[0].len()]; map.len()];
    let mut ways: BinaryHeap<Way> = BinaryHeap::new();

    ways.push(Way {
        length: 0,
        row: 0,
        col: 0,
        last_few: VecDeque::new(),
    });

    loop {
        //println!("{:?}", ways);
        let node = ways.pop().unwrap();
        //println!("{:?}", node);

        let first = node.last_few.get(0).unwrap_or(&From::Right);
        let mut same = 0;
        let last_iter = node.last_few.iter().take(3);
        for val in last_iter {
            if val != first {
                break;
            }
            same += 1;
        }

        if node.last_few.len() > 0 && visited[node.row][node.col][node.last_few[0].to_usize()][same]
        {
            continue;
        }

        if node.last_few.len() > 0 {
            visited[node.row][node.col][node.last_few[0].to_usize()][same] = true;
        }
        //println!("{:?}", node);

        if node.row == map.len() - 1 && node.col == map[map.len() - 1].len() - 1 {
            println!("{:?}", node);

            return node.length;
        }

        let next_last_few = node.last_few.clone();

        for from in return_next_ways(&node.last_few) {
            //println!("{:?}", from);
            let next = get_next_coord(map, (node.row, node.col), &from);

            for from in return_next_ways(&node.last_few) {
                //println!("{:?}", from);
                let next = get_next_coord(map, (node.row, node.col), &from);

                if let Some(next) = next {
                    /*println!(
                        "SOME {:?} {:?} {:?} {}",
                        from, next, node, map[next.0][next.1]
                    );*/
                    let mut last_few = next_last_few.clone();

                    last_few.push_front(from);
                    //println!("{:?}", last_few);
                    ways.push(Way {
                        length: node.length + map[next.0][next.1] as usize,
                        row: next.0,
                        col: next.1,
                        last_few,
                    });
                }
            }
            if let Some(next) = next {
                /*println!(
                    "SOME {:?} {:?} {:?} {}",
                    from, next, node, map[next.0][next.1]
                );*/
                let mut last_few = next_last_few.clone();

                last_few.push_front(from);
                //println!("{:?}", last_few);
                ways.push(Way {
                    length: node.length + map[next.0][next.1] as usize,
                    row: next.0,
                    col: next.1,
                    last_few,
                });
            }
        }
    }
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let map = cmap.map_to(|x| x.to_digit(10).unwrap());

    Ok(find_way(&map))
}

pub fn count_last_consecutive_x(x: &From, xs: &VecDeque<From>, num: usize) -> usize {
    let mut same = 0;
    for val in xs.iter().take(num) {
        if val != x {
            break;
        }
        same += 1;
    }
    //println!("{:?}, {:?}, {}, {}", x, xs, num, same);

    same
}

pub fn return_next_ways_2(last: &VecDeque<From>) -> Vec<From> {
    let mut res = Vec::new();

    if last.len() == 0
        || (last[0] != From::Bottom && count_last_consecutive_x(&From::Top, last, 10) < 10)
    {
        res.push(From::Top);
    }
    if last.len() > 0
        && (last[0] != From::Top && count_last_consecutive_x(&From::Bottom, last, 10) < 10)
    {
        res.push(From::Bottom);
    }
    if last.len() > 0
        && (last[0] != From::Left && count_last_consecutive_x(&From::Right, last, 10) < 10)
    {
        res.push(From::Right);
    }
    if last.len() == 0
        || (last[0] != From::Right && count_last_consecutive_x(&From::Left, last, 10) < 10)
    {
        //println!("LEFFFFT");
        res.push(From::Left);
    }

    res
}

pub fn find_way_2(map: &Vec<Vec<u32>>) -> usize {
    let mut visited = vec![vec![vec![vec![false; 11]; 4]; map[0].len()]; map.len()];
    let mut ways: BinaryHeap<Way> = BinaryHeap::new();

    ways.push(Way {
        length: 0,
        row: 0,
        col: 0,
        last_few: VecDeque::new(),
    });

    loop {
        //println!("{:?}", ways);
        let node = ways.pop().unwrap();
        //println!("{:?}", node);

        let first = node.last_few.get(0).unwrap_or(&From::Right);
        let same = count_last_consecutive_x(first, &node.last_few, 10);

        if node.last_few.len() > 0 && visited[node.row][node.col][node.last_few[0].to_usize()][same]
        {
            continue;
        }

        if node.last_few.len() > 0 {
            visited[node.row][node.col][node.last_few[0].to_usize()][same] = true;
        }
        //println!("{:?}", node);

        if node.row == map.len() - 1 && node.col == map[map.len() - 1].len() - 1 && same >= 4 {
            println!("{:?}", node);

            return node.length;
        }

        if node.last_few.len() == 0 || same >= 4 {
            for from in return_next_ways_2(&node.last_few) {
                //println!("{:?}", from);
                let next = get_next_coord(map, (node.row, node.col), &from);

                if let Some(next) = next {
                    /*println!(
                        "SOME {:?} {:?} {:?} {}",
                        from, next, node, map[next.0][next.1]
                    );*/
                    let mut last_few = node.last_few.clone();

                    last_few.push_front(from);
                    //println!("{:?}", last_few);
                    ways.push(Way {
                        length: node.length + map[next.0][next.1] as usize,
                        row: next.0,
                        col: next.1,
                        last_few,
                    });
                }
            }
        } else {
            let from = node.last_few[0].clone();
            let next = get_next_coord(map, (node.row, node.col), &from);

            if let Some(next) = next {
                let mut last_few = node.last_few.clone();

                last_few.push_front(from);
                ways.push(Way {
                    length: node.length + map[next.0][next.1] as usize,
                    row: next.0,
                    col: next.1,
                    last_few,
                });
            }
        }
    }
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let map = cmap.map_to(|x| x.to_digit(10).unwrap());

    Ok(find_way_2(&map))
}
