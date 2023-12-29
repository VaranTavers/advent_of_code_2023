use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
pub struct Line2D {
    pub start: (f64, f64),
    pub x_way: f64,
    pub m: f64,
    pub n: f64,
}

impl Line2D {
    pub fn intersection(&self, other: &Self) -> Option<(f64, f64)> {
        if (self.m - other.m).abs() < 0.000000000000000001 {
            return None;
        }

        let x = (other.n - self.n) / (self.m - other.m);
        let y = self.m * x + self.n;

        Some((x, y))
    }
}

impl FromStr for Line2D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(" @ ").unwrap();
        let point = parts
            .0
            .split(", ")
            .flat_map(|x| x.parse::<f64>())
            .collect::<Vec<f64>>();
        let velocity = parts
            .1
            .split(",")
            .flat_map(|x| x.trim().parse::<f64>())
            .collect::<Vec<f64>>();

        let m = velocity[1] / velocity[0];
        Ok(Line2D {
            start: (point[0], point[1]),
            x_way: velocity[0].signum(),
            m,
            n: point[1] - m * point[0],
        })
    }
}

const MIN_X: f64 = 200000000000000.0;
const MIN_Y: f64 = 200000000000000.0;
const MIN_Z: f64 = 100.0;
const MAX_X: f64 = 400000000000000.0;
const MAX_Y: f64 = 400000000000000.0;
const MAX_Z: f64 = 100.0;

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader
        .lines()
        .map(|x| x.unwrap().parse::<Line2D>().unwrap())
        .collect::<Vec<Line2D>>();

    println!("{:?}", lines);

    let mut intersections = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for j in i..lines.len() {
            let other = &lines[j];
            if let Some(point) = line.intersection(other) {
                println!("{i} {j} {:?}", point);
                if point.0 > MIN_X
                    && point.0 < MAX_X
                    && point.1 > MIN_Y
                    && point.1 < MAX_Y
                    && ((point.0 > line.start.0 && line.x_way > 0.0)
                        || (point.0 < line.start.0 && line.x_way < 0.0))
                    && ((point.0 > other.start.0 && other.x_way > 0.0)
                        || (point.0 < other.start.0 && other.x_way < 0.0))
                {
                    intersections.push(point);
                }
            }
        }
    }

    Ok(intersections.len())
}

#[derive(Debug)]
pub struct Line3D {
    pub start: (f64, f64, f64),
    pub velocity: (f64, f64, f64),
}

impl FromStr for Line3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(" @ ").unwrap();
        let point = parts
            .0
            .split(", ")
            .flat_map(|x| x.parse::<f64>())
            .collect::<Vec<f64>>();
        let velocity = parts
            .1
            .split(", ")
            .flat_map(|x| x.parse::<f64>())
            .collect::<Vec<f64>>();

        Ok(Line3D {
            start: (point[0], point[1], point[2]),
            velocity: (velocity[0], velocity[1], velocity[2]),
        })
    }
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader
        .lines()
        .map(|x| x.unwrap().parse::<Line3D>().unwrap())
        .collect::<Vec<Line3D>>();

    todo!();

    Ok(0)
}
