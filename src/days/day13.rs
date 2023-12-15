use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_maps(reader: BufReader<File>) -> Vec<Vec<Vec<char>>> {
    let mut res = Vec::new();
    let mut map = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if !map.is_empty() {
                res.push(map);
                map = Vec::new();
            }
        } else {
            map.push(line.chars().collect());
        }
    }

    if !map.is_empty() {
        res.push(map);
    }

    res
}

pub fn transpose_copy<T: Copy>(mat: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res = Vec::with_capacity(mat[0].len());
    for j in 0..mat[0].len() {
        let mut line = Vec::with_capacity(mat.len());
        for i in 0..mat.len() {
            line.push(mat[i][j]);
        }
        res.push(line);
    }

    res
}

pub fn equals(xs: &[char], ys: &[char]) -> bool {
    for (x, y) in xs.iter().zip(ys.iter()) {
        if x != y {
            return false;
        }
    }

    true
}

pub fn is_mirror(map: &Vec<Vec<char>>, i: usize) -> bool {
    let mut i = i + 1;
    let mut j = i;
    while i > 0 && j < map.len() && equals(&map[i - 1], &map[j]) {
        i -= 1;
        j += 1;
    }
    i == 0 || j == map.len()
}

pub fn find_mirror(map: &Vec<Vec<char>>) -> Vec<usize> {
    let mut i = 0;
    let mut res = Vec::new();

    while i < map.len() - 1 {
        println!("{i} {:?}, {:?}", map[i], map[i + 1]);

        if is_mirror(&map, i) {
            for line in map.iter() {
                for c in line.iter() {
                    print!("{c}");
                }
                println!();
            }
            println!("{i} {:?}, {:?}", map[i], map[i + 1]);
            res.push(i + 1);
        }
        i += 1;
    }

    res
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let maps = get_maps(reader);

    let maps_t = maps
        .iter()
        .map(transpose_copy)
        .collect::<Vec<Vec<Vec<char>>>>();

    /*for line in &maps[0] {
        for c in line {
            print!("{c}");
        }
        println!();
    }*/

    Ok(maps.iter().flat_map(find_mirror).sum::<usize>() * 100
        + maps_t.iter().flat_map(find_mirror).sum::<usize>())
}

pub fn diff(xs: &[char], ys: &[char]) -> (usize, usize) {
    let mut diff = 0;
    let mut last = 0;
    for (z, (x, y)) in xs.iter().zip(ys.iter()).enumerate() {
        if x != y {
            diff += 1;
            last = z;
        }
    }

    (diff, last)
}

pub fn can_be_mirror(map: &Vec<Vec<char>>, i_orig: usize) -> Option<(usize, usize, usize)> {
    let mut i = i_orig + 1;
    let mut j = i;

    let mut diff_g = 0;
    let mut last_g = 0;
    let mut last_i = 0;
    while i > 0 && j < map.len() {
        let (diff_c, last_c) = diff(&map[i - 1], &map[j]);
        diff_g += diff_c;
        if diff_c == 1 {
            last_g = last_c;
            last_i = i;
        }
        i -= 1;
        j += 1;
    }

    if diff_g == 1 {
        return Some((last_i - 1, last_g, i_orig));
    }

    None
}

pub fn find_smudge(map: &Vec<Vec<char>>) -> Option<(usize, usize, usize)> {
    let mut i = 0;

    while i < map.len() - 1 {
        if let Some((line, col, i_orig)) = can_be_mirror(&map, i) {
            return Some((line, col, i_orig));
        }
        i += 1;
    }

    None
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut maps = get_maps(reader);

    let maps_t = maps
        .iter()
        .map(transpose_copy)
        .collect::<Vec<Vec<Vec<char>>>>();

    let smudges_map = maps
        .iter()
        .enumerate()
        .flat_map(|(i, x)| find_smudge(x).map(|y| (i, y)))
        .collect::<Vec<(usize, (usize, usize, usize))>>();
    let smudges_map_t = maps_t
        .iter()
        .enumerate()
        .flat_map(|(i, x)| find_smudge(x).map(|y| (i, y)))
        .collect::<Vec<(usize, (usize, usize, usize))>>();

    let mirrors_maps = maps
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            find_mirror(x)
                .iter()
                .map(|y| (i, *y))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();
    let mirrors_maps_t = maps_t
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            find_mirror(x)
                .iter()
                .map(|y| (i, *y))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    println!(
        "{}, {}, {}",
        smudges_map.len(),
        smudges_map_t.len(),
        maps.len()
    );

    println!("{:?}", smudges_map);
    println!("{:?}", smudges_map_t);

    for (mapi, (line, num, _)) in smudges_map {
        //println!("{mapi} {line} {num}");
        if maps[mapi][line][num] == '#' {
            maps[mapi][line][num] = '.';
        } else {
            maps[mapi][line][num] = '#';
        }
    }

    for (mapi, (line, num, _)) in smudges_map_t {
        if maps[mapi][num][line] == '#' {
            maps[mapi][num][line] = '.';
        } else {
            maps[mapi][num][line] = '#';
        }
    }

    /*println!(
        "{:?}",
        maps_t.iter().flat_map(find_mirror).collect::<Vec<usize>>()
    );*/

    let maps_t = maps
        .iter()
        .map(transpose_copy)
        .collect::<Vec<Vec<Vec<char>>>>();

    for line in maps_t[7].iter() {
        for c in line.iter() {
            print!("{c}");
        }
        println!();
    }
    println!();
    println!();

    println!("{:?}", find_mirror(&maps_t[7]));

    let new_maps_mirrors = maps
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            find_mirror(x)
                .iter()
                .map(|y| (i, *y))
                .collect::<Vec<(usize, usize)>>()
        })
        .filter(|res| !mirrors_maps.iter().any(|y| res.0 == y.0 && res.1 == y.1))
        .collect::<Vec<(usize, usize)>>();

    let new_maps_t_mirrors = maps_t
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            find_mirror(x)
                .iter()
                .map(|y| (i, *y))
                .collect::<Vec<(usize, usize)>>()
        })
        .filter(|res| !mirrors_maps_t.iter().any(|y| res.0 == y.0 && res.1 == y.1))
        .collect::<Vec<(usize, usize)>>();

    Ok(new_maps_mirrors.iter().map(|x| x.1).sum::<usize>() * 100
        + new_maps_t_mirrors.iter().map(|x| x.1).sum::<usize>())
}
