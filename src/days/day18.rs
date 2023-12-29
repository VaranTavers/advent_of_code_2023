use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s;
        if s.starts_with('(') {
            s = &s[1..s.len() - 1];
        }
        if s.starts_with('#') {
            s = &s[1..]
        }

        let vals = s
            .chars()
            .map(|x| x.to_digit(16).unwrap())
            .collect::<Vec<u32>>();

        Ok(Color {
            red: (vals[0] * 16 + vals[1]) as u8,
            green: (vals[2] * 16 + vals[3]) as u8,
            blue: (vals[4] * 16 + vals[5]) as u8,
        })
    }
}

pub fn map_and_start_from_lines(
    lines: &[(char, usize, Color)],
) -> (Vec<Vec<Option<Color>>>, (usize, usize)) {
    let mut min_row: i64 = 0;
    let mut max_row: i64 = 0;
    let mut min_col: i64 = 0;
    let mut max_col: i64 = 0;
    let mut row: i64 = 0;
    let mut col: i64 = 0;

    for (c, len, _) in lines {
        match c {
            'U' => row -= *len as i64,
            'D' => row += *len as i64,
            'L' => col -= *len as i64,
            'R' => col += *len as i64,
            _ => {}
        }
        if row < min_row {
            min_row = row;
        }
        if row > max_row {
            max_row = row;
        }
        if col > max_col {
            max_col = col;
        }
        if col < min_col {
            min_col = col;
        }
    }
    let cols = (max_col - min_col) as usize + 1;
    let rows = (max_row - min_row) as usize + 1;

    let start_row: usize = (0 - min_row) as usize;
    let start_col: usize = (0 - min_col) as usize;

    let mut res: Vec<Vec<Option<Color>>> = (0..rows)
        .map(|_| (0..cols).map(|_| None).collect())
        .collect();

    res[start_row][start_col] = Some(Color {
        red: 0,
        green: 0,
        blue: 0,
    });
    (res, (start_row, start_col))
}

pub fn outline_map(
    map: &mut Vec<Vec<Option<Color>>>,
    start: (usize, usize),
    lines: &[(char, usize, Color)],
) {
    let mut row = start.0;
    let mut col = start.1;

    for (c, len, color) in lines {
        for _ in 0..*len {
            match c {
                'U' => row -= 1,
                'D' => row += 1,
                'L' => col -= 1,
                'R' => col += 1,
                _ => {}
            }
            map[row][col] = Some(*color);
        }
    }
}

fn is_line(map: &Vec<Vec<Option<Color>>>, row: usize, col: usize) -> bool {
    if row == 0 || row == map.len() - 1 {
        return false;
    }

    map[row - 1][col].is_some() && map[row + 1][col].is_some()
}

fn is_J(map: &Vec<Vec<Option<Color>>>, row: usize, col: usize) -> bool {
    if row == 0 || col == 0 {
        return false;
    }

    map[row - 1][col].is_some() && map[row][col - 1].is_some()
}

fn is_L(map: &Vec<Vec<Option<Color>>>, row: usize, col: usize) -> bool {
    if row == 0 || col == map[row].len() - 1 {
        return false;
    }

    map[row - 1][col].is_some() && map[row][col + 1].is_some()
}

pub fn fill_map(map: &mut Vec<Vec<Option<Color>>>) {
    let ref_map = map.clone();

    for row in 0..map.len() {
        let mut color: Option<Color> = None;
        for col in 0..map[row].len() {
            if color.is_none() && map[row][col].is_some() {
                if is_line(&ref_map, row, col)
                    || is_J(&ref_map, row, col)
                    || is_L(&ref_map, row, col)
                {
                    color = map[row][col];
                }
            } else if color.is_some() && map[row][col].is_some() {
                if is_line(&ref_map, row, col)
                    || is_J(&ref_map, row, col)
                    || is_L(&ref_map, row, col)
                {
                    color = None;
                }
            } else if color.is_some() {
                map[row][col] = color;
            }
        }
    }
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader
        .lines()
        .flat_map(|x| {
            x.map(|y| {
                let fields = y.split(' ').collect::<Vec<&str>>();
                let c = *(fields[0].chars().collect::<Vec<char>>().get(0).unwrap());
                let length = fields[1].parse::<usize>().unwrap();
                let color = fields[2].parse::<Color>().unwrap();
                (c, length, color)
            })
        })
        .collect::<Vec<(char, usize, Color)>>();

    let (mut map, start) = map_and_start_from_lines(&lines);

    outline_map(&mut map, start, &lines);
    for row in map.iter() {
        for col in row.iter() {
            if col.is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    fill_map(&mut map);
    println!();
    for row in map.iter() {
        for col in row.iter() {
            if col.is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    Ok(map.iter().map(|x| x.iter().flatten().count()).sum())
}

pub fn map_bounds_from_lines(lines: &[(u8, usize)]) -> ((usize, usize), (usize, usize)) {
    let mut min_row: i64 = 0;
    let mut max_row: i64 = 0;
    let mut min_col: i64 = 0;
    let mut max_col: i64 = 0;
    let mut row: i64 = 0;
    let mut col: i64 = 0;

    for (c, len) in lines {
        match c {
            1 => row -= *len as i64,
            3 => row += *len as i64,
            2 => col -= *len as i64,
            0 => col += *len as i64,
            _ => {}
        }
        if row < min_row {
            min_row = row;
        }
        if row > max_row {
            max_row = row;
        }
        if col > max_col {
            max_col = col;
        }
        if col < min_col {
            min_col = col;
        }
    }
    let cols = (max_col - min_col) as usize + 1;
    let rows = (max_row - min_row) as usize + 1;

    let start_row: usize = (0 - min_row) as usize;
    let start_col: usize = (0 - min_col) as usize;

    ((rows, cols), (start_row, start_col))
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    /*let lines = reader
    .lines()
    .flat_map(|x| {
        x.map(|y| {
            let fields = y.split(' ').collect::<Vec<&str>>();
            let color = fields[2].parse::<Color>().unwrap();
            (
                match &fields[0] {
                    &"U" => 3,
                    &"D" => 1,
                    &"R" => 0,
                    &"L" | _ => 2,
                },
                fields[1].parse::<usize>().unwrap(),
            )
        })
    })
    .collect::<Vec<(u8, usize)>>();*/
    let lines = reader
        .lines()
        .flat_map(|x| {
            x.map(|y| {
                let fields = y.split(' ').collect::<Vec<&str>>();
                let color = fields[2].parse::<Color>().unwrap();
                (
                    color.blue % 16,
                    color.red as usize * 256 * 16
                        + color.green as usize * 16
                        + color.blue as usize / 16,
                )
            })
        })
        .collect::<Vec<(u8, usize)>>();

    let (bounds, start) = map_bounds_from_lines(&lines);
    println!("{:?}", lines);

    let mut row = start.0;
    let mut col = start.1;
    let mut area: i64 = 0;

    let (mut last_way, _) = lines.last().unwrap();

    for (i, (way, len)) in lines.iter().enumerate() {
        println!("Area: {area}, Way: {way} Len: {len}");
        println!("{row}, {col}");
        let mut row_next = row;
        let mut col_next = col;
        match way {
            1 => {
                row_next -= *len;
                //area += (row - row_next + 1) as i64;
                //println!("DOWN PLUS: {}", (row - row_next + 1));
            }
            3 => {
                row_next += *len;
                //area += (row_next - row + 1) as i64;
                //println!("UP PLUS: {}", (row_next - row + 1));
            }
            2 => {
                col_next -= *len;

                if last_way == 1 {
                    if lines[i + 1].0 == 1 {
                        println!(
                            "Minus F--J: {} * {} = {}",
                            row,
                            (col - col_next),
                            row as i64 * (col - col_next) as i64
                        );
                        area -= row as i64 * (col - col_next) as i64;
                    } else {
                        println!(
                            "Minus L--J: {} * {} = {}",
                            row,
                            (col - col_next),
                            row as i64 * (col - col_next + 1) as i64
                        );
                        area -= row as i64 * (col - col_next + 1) as i64;
                    }
                } else {
                    if lines[i + 1].0 == 1 {
                        println!(
                            "Minus F--7: {} * {} = {}",
                            row,
                            (col - col_next),
                            row as i64 * (col - col_next) as i64
                        );
                        area -= row as i64 * (col - col_next - 1) as i64;
                    } else {
                        println!(
                            "Minus L--7: {} * {} = {}",
                            row,
                            (col - col_next),
                            row as i64 * (col - col_next) as i64
                        );
                        area -= row as i64 * (col - col_next) as i64;
                    }
                }
            }
            0 => {
                col_next += *len;

                if last_way == 1 {
                    if lines[i + 1].0 == 3 {
                        area += (row + 1) as i64 * (col_next - col - 1) as i64;

                        println!(
                            "Plus L--J: {}",
                            (row + 1) as i64 * (col_next - col - 1) as i64
                        );
                    } else {
                        area += (row + 1) as i64 * (col_next - col) as i64;

                        println!("Plus L--7: {}", (row + 1) as i64 * (col_next - col) as i64);
                    }
                } else {
                    if lines[i + 1].0 == 3 {
                        area += (row + 1) as i64 * (col_next - col) as i64;
                        println!("Plus F--J: {}", (row + 1) as i64 * (col_next - col) as i64);
                    } else {
                        area += (row + 1) as i64 * (col_next - col + 1) as i64;

                        println!(
                            "Plus F--7: {}",
                            (row + 1) as i64 * (col_next - col + 1) as i64
                        );
                    }
                }
            }
            _ => {}
        }
        col = col_next;
        row = row_next;
        last_way = *way;
    }

    Ok(area as usize)
}
