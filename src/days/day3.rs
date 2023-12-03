use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_gear_ratio((x, y): (usize, usize), vs: &Vec<Vec<i64>>) -> i64 {
    if vs[x][y] != -2 {
        return 0;
    }

    let mut prod = 1;
    let mut num = 0;
    let mut skip;

    println!("{} {}", x, y);
    for i in (1.max(x) - 1)..((vs.len() - 1).min(x + 1) + 1) {
        skip = 0;
        for j in (1.max(y) - 1)..((vs[i].len() - 1).min(y + 1) + 1) {
            if vs[i][j] > 0 && vs[i][j] != skip {
                println!("{} ON ({}, {}) FROM ({}, {})", vs[x][y], x, y, i, j);

                prod *= vs[i][j];
                num += 1;
            }
            skip = vs[i][j];
        }
    }

    if num != 2 {
        return 0;
    }

    prod
}

fn num_if_part((x, y): (usize, usize), vs: &Vec<Vec<i64>>) -> i64 {
    if vs[x][y] < 1 {
        return 0;
    }
    println!("{} {}", x, y);
    for i in (1.max(x) - 1)..((vs.len() - 1).min(x + 1) + 1) {
        for j in (1.max(y) - 1)..((vs[i].len() - 1).min(y + 1) + 1) {
            println!(
                "TRY {} ON ({}, {}) FROM ({}, {}) = {}",
                vs[x][y], x, y, i, j, vs[i][j]
            );
            if vs[i][j] < 0 {
                println!("{} ON ({}, {}) FROM ({}, {})", vs[x][y], x, y, i, j);
                return vs[x][y];
            }
        }
    }

    0
}

fn turn_charvec_in_numvec(cv: Vec<char>) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::new();
    let mut num = 0;
    let mut len = 0;

    for (i, c) in cv.iter().enumerate() {
        if !c.is_numeric() {
            if len > 0 {
                for j in 1..(len + 1) {
                    res[i - j] = num;
                }
            }
            len = 0;
            num = 0;
            if c == &'.' {
                res.push(0);
            } else if c == &'*' {
                res.push(-2);
            } else {
                res.push(-1);
            }
        } else {
            len += 1;
            num = num * 10 + c.to_digit(10).unwrap() as i64;
            res.push(0);
        }
    }
    let res_len = res.len();
    if len > 0 {
        for j in 1..(len + 1) {
            res[res_len - j] = num;
        }
    }

    res
}

pub fn solution() -> Result<i64, std::io::Error> {
    let f = File::open("inputs/input3")?;
    let reader = BufReader::new(f);

    let chars = reader
        .lines()
        .filter_map(|x| x.map(|y| y.chars().collect::<Vec<char>>()).ok())
        .map(turn_charvec_in_numvec)
        .collect::<Vec<Vec<i64>>>();

    println!("{:?}", chars);

    let mut sum = 0;
    let mut skip;

    for i in 0..chars.len() {
        skip = 0;
        for j in 0..chars[i].len() {
            let num = num_if_part((i, j), &chars);
            if skip != num {
                if num > 0 {
                    println!("ADDED {}", num);
                }
                sum += num;
                skip = num;
            }
        }
    }

    Ok(sum)
}

pub fn solution_2() -> Result<i64, std::io::Error> {
    let f = File::open("inputs/input3")?;
    let reader = BufReader::new(f);

    let chars = reader
        .lines()
        .filter_map(|x| x.map(|y| y.chars().collect::<Vec<char>>()).ok())
        .map(turn_charvec_in_numvec)
        .collect::<Vec<Vec<i64>>>();

    let mut sum = 0;

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            let num = check_gear_ratio((i, j), &chars);
            sum += num;
        }
    }

    Ok(sum)
}
