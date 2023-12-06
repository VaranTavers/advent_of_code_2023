use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

// s.parse::<i32>()

fn validate_1(line: &str) -> Option<usize> {
    let row_split = line.split(':').collect::<Vec<&str>>();
    let game_split = row_split[0].split(' ').collect::<Vec<&str>>();
    let game_num = game_split[1].parse::<usize>().unwrap();

    let runs_split = row_split[1].trim().split(';').collect::<Vec<&str>>();
    for run in runs_split {
        let colors = run.split(',').collect::<Vec<&str>>();
        for color in colors {
            let color_split = color.trim().split(' ').collect::<Vec<&str>>();
            let num = color_split[0].parse::<usize>().unwrap();
            if (color_split[1] == "red" && num > MAX_RED)
                || (color_split[1] == "green" && num > MAX_GREEN)
                || (color_split[1] == "blue" && num > MAX_BLUE)
            {
                return None;
            }
        }
    }

    Some(game_num)
}

pub fn solution_1(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    Ok(reader.lines().filter_map(|x| validate_1(&x.unwrap())).sum())
}

fn validate_2(line: &str) -> Option<usize> {
    let row_split = line.split(':').collect::<Vec<&str>>();
    let game_split = row_split[0].split(' ').collect::<Vec<&str>>();
    let game_num = game_split[1].parse::<usize>().unwrap();

    let runs_split = row_split[1].trim().split(';').collect::<Vec<&str>>();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for run in runs_split {
        let colors = run.split(',').collect::<Vec<&str>>();
        for color in colors {
            let color_split = color.trim().split(' ').collect::<Vec<&str>>();
            let num = color_split[0].parse::<usize>().unwrap();
            if color_split[1] == "red" && num > max_red {
                max_red = num;
            }
            if color_split[1] == "green" && num > max_green {
                max_green = num;
            }
            if color_split[1] == "blue" && num > max_blue {
                max_blue = num;
            }
        }
    }

    Some(max_blue * max_green * max_red)
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    Ok(reader.lines().filter_map(|x| validate_2(&x.unwrap())).sum())
}
