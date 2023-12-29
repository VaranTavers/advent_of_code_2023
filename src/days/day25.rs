use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn connections_between_components(map: &Vec<Vec<bool>>, components: &Vec<bool>) -> usize {
    let mut res = 0;

    for (i, node) in components.iter().enumerate() {
        res += neighbors_with_component(map, components, i, !node);
    }

    res / 2
}

pub fn conns_between_comps_n(map: &Vec<Vec<bool>>, components: &Vec<bool>, n: usize) -> usize {
    let mut res = 0;

    for (i, node) in components.iter().enumerate().take(n) {
        res += neighbors_with_component_n(map, components, i, !node, n);
    }

    res / 2
}

pub fn neighbors_with_component(
    map: &Vec<Vec<bool>>,
    components: &Vec<bool>,
    i: usize,
    component: bool,
) -> usize {
    map[i]
        .iter()
        .enumerate()
        .filter(|(j, x)| **x && components[*j] == component)
        .count()
}

pub fn neighbors_with_component_n(
    map: &Vec<Vec<bool>>,
    components: &Vec<bool>,
    i: usize,
    component: bool,
    n: usize,
) -> usize {
    map[i]
        .iter()
        .take(n)
        .enumerate()
        .filter(|(j, x)| **x && components[*j] == component)
        .count()
}

pub fn backtrack(
    map: &Vec<Vec<bool>>,
    components: &mut Vec<bool>,
    i: usize,
    j: &mut usize,
) -> Option<usize> {
    let conns = conns_between_comps_n(&map, components, i);
    if i == components.len() {
        let comp_a = components.iter().filter(|x| **x).count();
        let comp_b = components.len() - comp_a;
        *j += 1;

        if *j % 10 == 0 {
            println!("{j}");
        }

        if conns == 3 {
            return Some(comp_a * comp_b);
        }

        return None;
    } else if conns > 3 {
        *j += 1;
        if *j % 10 == 0 {
            println!("{j} CUT {i}");
        }
        return None;
    }

    components[i] = true;
    let left = backtrack(map, components, i + 1, j);

    components[i] = false;
    let right = backtrack(map, components, i + 1, j);

    if left.is_some() && right.is_none() {
        return left;
    } else if left.is_none() && right.is_some() {
        return right;
    } else if left.is_none() && right.is_none() {
        return None;
    }

    if left.unwrap() > right.unwrap() {
        return left;
    }

    right
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let edge_list = reader.lines().flatten().collect::<Vec<String>>();

    let mut nodes = HashSet::new();

    for edges in edge_list.iter() {
        let parts = edges.split_once(": ").unwrap();
        nodes.insert(parts.0.to_owned());
        let other_nodes = parts.1.split(' ');
        for other in other_nodes {
            nodes.insert(other.to_owned());
        }
    }

    let mut map = HashMap::new();

    for (i, node) in nodes.iter().enumerate() {
        map.insert(node.as_str(), i);
    }

    let mut incidence_mat = (0..nodes.len())
        .map(|_x| (0..nodes.len()).map(|_y| false).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    for edges in edge_list.iter() {
        let parts = edges.split_once(": ").unwrap();
        let from_id = map.get(parts.0).unwrap();
        let other_nodes = parts.1.split(' ');
        for other in other_nodes {
            let to_id = map.get(other).unwrap();
            incidence_mat[*from_id][*to_id] = true;
            incidence_mat[*to_id][*from_id] = true;
        }
    }

    let mut components = nodes.iter().map(|_| false).collect::<Vec<bool>>();

    println!("{}", components.len());

    let mut j = 0;

    // Too slow, and I'm lazy to implement minimum cut, so Julia, here we go
    //let res = backtrack(&incidence_mat, &mut components, 0, &mut j);

    for row in incidence_mat.iter() {
        for val in row.iter() {
            print!("{val}\t");
        }

        println!();
    }
    //println!("{:?}", incidence_mat);

    Ok(0)
}
