use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn hash(s: &str) -> usize {
    let mut res = 0;

    for c in s.chars() {
        res += c as usize;
        res *= 17;
        res %= 256;
    }

    res
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();
    let line = lines.next().unwrap().unwrap();

    let codes = line.split(',').collect::<Vec<&str>>();

    Ok(codes.iter().map(|x| hash(*x)).sum())
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();
    let line = lines.next().unwrap().unwrap();

    let codes = line.split(',').collect::<Vec<&str>>();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    for code in codes.iter() {
        let label = code.split(['-', '=']).collect::<Vec<&str>>();
        let label_hash = hash(label[0]);
        if code.contains('=') {
            let focal_length = label[1].parse::<usize>().unwrap();
            let pos = boxes[label_hash].iter().position(|x| x.0 == label[0]);
            if let Some(pos) = pos {
                boxes[label_hash][pos] = (label[0], focal_length);
            } else {
                boxes[label_hash].push((label[0], focal_length));
            }
        } else {
            boxes[label_hash] = boxes[label_hash]
                .iter()
                .filter(|x| x.0 != label[0])
                .cloned()
                .collect();
        }
    }

    for (i, boxx) in boxes.iter().enumerate() {
        if boxx.len() > 0 {
            println!("{i}: {:?}", boxx);
        }
    }

    Ok(boxes
        .iter()
        .enumerate()
        .map(|(i, x)| {
            (i + 1)
                * x.iter()
                    .enumerate()
                    .map(|(j, y)| (j + 1) * y.1)
                    .sum::<usize>()
        })
        .sum::<usize>())
}
