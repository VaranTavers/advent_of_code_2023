use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::BufReader,
};

use crate::utils::CharMap;

pub fn longest_route_slippery(map: &CharMap, (s_row, s_col): (usize, usize)) -> usize {
    let mut ways_end: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut ways_working: VecDeque<Vec<(usize, usize)>> = VecDeque::new();

    let start = vec![(s_row, s_col)];
    ways_working.push_back(start);

    while let Some(way) = ways_working.pop_front() {
        let last = way.last().unwrap();

        if last.0 == map.map.len() - 1 {
            ways_end.push(way);
            continue;
        }

        if last.0 > 0
            && !way.iter().any(|(x, y)| *x == last.0 - 1 && *y == last.1)
            && map.map[last.0 - 1][last.1] != '#'
            && map.map[last.0 - 1][last.1] != 'v'
        {
            let mut clone = way.clone();
            clone.push((last.0 - 1, last.1));
            ways_working.push_back(clone);
        }
        if last.0 < map.map.len() - 1
            && !way.iter().any(|(x, y)| *x == last.0 + 1 && *y == last.1)
            && map.map[last.0 + 1][last.1] != '#'
            && map.map[last.0 + 1][last.1] != '^'
        {
            let mut clone = way.clone();
            clone.push((last.0 + 1, last.1));
            ways_working.push_back(clone);
        }
        if last.1 > 0
            && !way.iter().any(|(x, y)| *x == last.0 && *y == last.1 - 1)
            && map.map[last.0][last.1 - 1] != '#'
            && map.map[last.0][last.1 - 1] != '>'
        {
            let mut clone = way.clone();
            clone.push((last.0, last.1 - 1));
            ways_working.push_back(clone);
        }
        if last.1 < map.map[last.0].len() - 1
            && !way.iter().any(|(x, y)| *x == last.0 && *y == last.1 + 1)
            && map.map[last.0][last.1 + 1] != '#'
            && map.map[last.0][last.1 + 1] != '<'
        {
            let mut clone = way.clone();
            clone.push((last.0, last.1 + 1));
            ways_working.push_back(clone);
        }
    }

    ways_end.iter().map(|x| x.len()).max().unwrap() - 1
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let map = CharMap::parse_map(reader);

    let start_pos = map.find_first('.').unwrap();

    Ok(longest_route_slippery(&map, start_pos))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub length: usize,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

pub fn get_number_of_neighbors(map: &CharMap, last: &(usize, usize)) -> usize {
    let mut sum = 0;

    if last.0 > 0 && map.map[last.0 - 1][last.1] != '#' {
        sum += 1;
    }
    if last.0 < map.map.len() - 1 && map.map[last.0 + 1][last.1] != '#' {
        sum += 1;
    }
    if last.1 > 0 && map.map[last.0][last.1 - 1] != '#' {
        sum += 1;
    }
    if last.1 < map.map[last.0].len() - 1 && map.map[last.0][last.1 + 1] != '#' {
        sum += 1;
    }

    sum
}

pub fn route_to_node(
    map: &CharMap,
    (s_row, s_col): &(usize, usize),
    from: &(usize, usize),
) -> Node {
    let mut ways_working: VecDeque<Vec<(usize, usize)>> = VecDeque::new();
    let mut ways_end = Vec::new();

    let start = vec![*from, (*s_row, *s_col)];
    ways_working.push_back(start);

    while let Some(way) = ways_working.pop_front() {
        let last = way.last().unwrap();
        //println!("{:?}", way);

        if get_number_of_neighbors(map, last) > 2 || last.0 == map.map.len() - 1 || last.0 == 0 {
            ways_end = way;
            break;
        }

        if last.0 > 0
            && !way.iter().any(|(x, y)| *x == last.0 - 1 && *y == last.1)
            && map.map[last.0 - 1][last.1] != '#'
        {
            let mut clone = way.clone();
            clone.push((last.0 - 1, last.1));
            ways_working.push_back(clone);
        }
        if last.0 < map.map.len() - 1
            && !way.iter().any(|(x, y)| *x == last.0 + 1 && *y == last.1)
            && map.map[last.0 + 1][last.1] != '#'
        {
            let mut clone = way.clone();
            clone.push((last.0 + 1, last.1));
            ways_working.push_back(clone);
        }
        if last.1 > 0
            && !way.iter().any(|(x, y)| *x == last.0 && *y == last.1 - 1)
            && map.map[last.0][last.1 - 1] != '#'
        {
            let mut clone = way.clone();
            clone.push((last.0, last.1 - 1));
            ways_working.push_back(clone);
        }
        if last.1 < map.map[last.0].len() - 1
            && !way.iter().any(|(x, y)| *x == last.0 && *y == last.1 + 1)
            && map.map[last.0][last.1 + 1] != '#'
        {
            let mut clone = way.clone();
            clone.push((last.0, last.1 + 1));
            ways_working.push_back(clone);
        }
    }

    //println!("{s_row} {s_col} {:?}  {:?}", from, ways_end);
    Node {
        length: ways_end.len(),
        start: *from,
        end: (*ways_end.last().unwrap()),
    }
}

pub fn longest_route_dry_to_nodes(map: &CharMap, (s_row, s_col): (usize, usize)) -> Vec<Node> {
    let mut nodes_end: Vec<Node> = Vec::new();
    let mut crossroads: VecDeque<(usize, usize)> = VecDeque::new();

    let res = route_to_node(map, &(s_row + 1, s_col), &(s_row, s_col));
    crossroads.push_back(res.end);

    while let Some(last) = crossroads.pop_front() {
        if last.0 == map.map.len() - 1 {
            continue;
        }

        if last.0 > 0 && map.map[last.0 - 1][last.1] != '#' {
            let res = route_to_node(map, &(last.0 - 1, last.1), &last);

            if !nodes_end.iter().any(|x| x.start == res.end) {
                if !crossroads.iter().any(|x| x == &res.end) {
                    crossroads.push_back(res.end);
                }
                nodes_end.push(res);
            }
        }
        if last.0 < map.map.len() - 1 && map.map[last.0 + 1][last.1] != '#' {
            let res = route_to_node(map, &(last.0 + 1, last.1), &last);

            if !nodes_end.iter().any(|x| x.start == res.end) {
                if !crossroads.iter().any(|x| x == &res.end) {
                    crossroads.push_back(res.end);
                }
                nodes_end.push(res);
            }
        }
        if last.1 > 0 && map.map[last.0][last.1 - 1] != '#' {
            let res = route_to_node(map, &(last.0, last.1 - 1), &last);

            if !nodes_end.iter().any(|x| x.start == res.end) {
                if !crossroads.iter().any(|x| x == &res.end) {
                    crossroads.push_back(res.end);
                }
                nodes_end.push(res);
            }
        }
        if last.1 < map.map[last.0].len() - 1 && map.map[last.0][last.1 + 1] != '#' {
            let res = route_to_node(map, &(last.0, last.1 + 1), &last);

            if !nodes_end.iter().any(|x| x.start == res.end) {
                if !crossroads.iter().any(|x| x == &res.end) {
                    crossroads.push_back(res.end);
                }
                nodes_end.push(res);
            }
        }
    }

    nodes_end
}

pub fn get_longest_backtrack(
    nodes: &Vec<Node>,
    route: Vec<(usize, usize)>,
    end: &(usize, usize),
) -> usize {
    let last = route.last().unwrap();

    if last == end {
        return route
            .windows(2)
            .map(|x| {
                nodes
                    .iter()
                    .filter(|y| {
                        (y.start == x[0] && y.end == x[1]) || (y.end == x[0] && y.start == x[1])
                    })
                    .map(|x| x.length)
                    .max()
                    .unwrap()
                    - 1
            })
            .sum();
    }

    let mut max = 0;

    for node in nodes.iter().filter(|x| x.start == *last) {
        if !route.iter().any(|x| &node.end == x) {
            let mut route_c = route.clone();
            route_c.push(node.end);

            let res = get_longest_backtrack(nodes, route_c, end);

            if res > max {
                max = res;
            }
        }
    }

    for node in nodes.iter().filter(|x| x.end == *last) {
        if !route.iter().any(|x| &node.start == x) {
            let mut route_c = route.clone();
            route_c.push(node.start);

            let res = get_longest_backtrack(nodes, route_c, end);

            if res > max {
                max = res;
            }
        }
    }

    max
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let map = CharMap::parse_map(reader);

    let start_pos = map.find_first('.').unwrap();

    let nodes = longest_route_dry_to_nodes(&map, start_pos);
    println!("We got nodes {}", nodes.len());

    Ok(get_longest_backtrack(
        &nodes,
        vec![start_pos],
        &(
            map.map.len() - 1,
            map.map[map.map.len() - 1]
                .iter()
                .position(|x| *x == '.')
                .unwrap(),
        ),
    ))
}
