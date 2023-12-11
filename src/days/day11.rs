use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn d(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((b.1 - a.1).powi(2) + (b.0 - a.0).powi(2)).sqrt()
}

pub fn squares_between(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let start = (a.0 as f64 + 0.5, a.1 as f64 + 0.5);
    let end = (b.0 as f64 + 0.5, b.1 as f64 + 0.5);

    if end.0 == start.0 {
        return (end.1 - start.1).abs() as usize;
    }
    if end.1 == start.1 {
        return (end.0 - start.0).abs() as usize;
    }

    let mut point = start.clone();
    let m = (end.0 - start.0) / (end.1 - start.1);

    if m == 1.0 || m == -1.0 {
        return 2 * b.0.abs_diff(a.0);
    }

    let n = point.0 - m * point.1;

    let mut moves = 0;
    loop {
        //println!("{:?} {m}", point);
        let next1_y = if end.0 > start.0 {
            point.0.ceil() + 0.00001
        } else {
            point.0.floor() - 0.00001
        };
        let next2_x = if end.1 > start.1 {
            point.1.ceil() + 0.00001
        } else {
            point.1.floor() - 0.00001
        };

        if (point.1.round() - point.1).abs() < 0.001 && (point.0.round() - point.0).abs() < 0.001 {
            moves += 1;
        }

        let next1_x = (next1_y - n) / m;
        let next2_y = m * next2_x + n;

        //println!("NEXT1? {next1_y} {next1_x}");
        //println!("NEXT2? {next2_y} {next2_x}");
        let dn3 = d(point, end);
        let dn2 = d(point, (next2_y, next2_x));
        let dn1 = d(point, (next1_y, next1_x));
        //println!("d1: {dn1} d2: {dn2} d3: {dn3}");
        if dn3 < dn1 && dn3 < dn2 {
            break;
        }
        if dn1 < dn2 {
            point = (next1_y, next1_x);
        } else {
            point = (next2_y, next2_x);
        }

        moves += 1;
    }

    moves
}

fn preprocess(reader: BufReader<File>) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let map = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let galaxies = map
        .iter()
        .enumerate()
        .flat_map(|(y, cs)| {
            cs.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'#')
                .map(|(x, _)| (y, x))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    let row_expansions = map
        .iter()
        .map(|x| !x.iter().any(|y| y == &'#'))
        .collect::<Vec<bool>>();

    let col_expansions = (0..map[0].len())
        .map(|y| !(0..map.len()).any(|x| map[x][y] == '#'))
        .collect::<Vec<bool>>();

    let mut row_exp_cs = Vec::with_capacity(row_expansions.len());
    row_expansions.iter().fold(0, |acc, new| {
        row_exp_cs.push(acc);

        if *new {
            return acc + 1;
        }

        acc
    });

    let mut col_exp_cs = Vec::with_capacity(col_expansions.len());
    col_expansions.iter().fold(0, |acc, new| {
        col_exp_cs.push(acc);

        if *new {
            return acc + 1;
        }

        acc
    });

    (galaxies, row_exp_cs, col_exp_cs)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut sum = 0;

    let (galaxies, row_exp_cs, col_exp_cs) = preprocess(reader);

    for (i, galaxy1) in galaxies.iter().enumerate() {
        //println!("{:?}", galaxy1);
        for galaxy2 in galaxies[i + 1..].iter() {
            //println!("\r{:?}", galaxy2);
            let expanded1 = (
                galaxy1.0 + row_exp_cs[galaxy1.0],
                galaxy1.1 + col_exp_cs[galaxy1.1],
            );
            let expanded2 = (
                galaxy2.0 + row_exp_cs[galaxy2.0],
                galaxy2.1 + col_exp_cs[galaxy2.1],
            );
            sum += squares_between(&expanded1, &expanded2);
        }
    }

    Ok(sum)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while a % b > 0 {
        let m = a % b;
        a = b;
        b = m;
    }

    b
}

pub fn squares_between_2(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    let a = (p2.0).abs_diff(p1.0);
    let b = (p2.1).abs_diff(p1.1);

    if a * b == 0 {
        return a + b;
    }
    a + b //- gcd(a, b)
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    // Got inspiration from math exchange, wrong inspiration, figured out
    let mut sum = 0;

    let (galaxies, row_exp_cs, col_exp_cs) = preprocess(reader);

    for (i, galaxy1) in galaxies.iter().enumerate() {
        //println!("{:?}", galaxy1);
        for galaxy2 in galaxies[i + 1..].iter() {
            //println!("\r{:?}", galaxy2);
            let expanded1 = (
                galaxy1.0 + row_exp_cs[galaxy1.0] * 999999,
                galaxy1.1 + col_exp_cs[galaxy1.1] * 999999,
            );
            let expanded2 = (
                galaxy2.0 + row_exp_cs[galaxy2.0] * 999999,
                galaxy2.1 + col_exp_cs[galaxy2.1] * 999999,
            );

            sum += squares_between_2(&expanded1, &expanded2);
        }
    }

    Ok(sum)
}
