use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct CharMap {
    pub map: Vec<Vec<char>>,
}

impl CharMap {
    pub fn parse_map(reader: BufReader<File>) -> CharMap {
        let map = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        CharMap { map }
    }

    pub fn parse_maps(reader: BufReader<File>) -> Vec<CharMap> {
        let mut res = Vec::new();
        let mut map = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                if !map.is_empty() {
                    res.push(CharMap { map });
                    map = Vec::new();
                }
            } else {
                map.push(line.chars().collect());
            }
        }

        if !map.is_empty() {
            res.push(CharMap { map });
        }

        res
    }

    pub fn map_to_val<T: Copy>(&self, val: T) -> Vec<Vec<T>> {
        vec![vec![val; self.map[0].len()]; self.map.len()]
    }

    pub fn clone_to_val<T: Clone>(&self, val: T) -> Vec<Vec<T>> {
        vec![vec![val.clone(); self.map[0].len()]; self.map.len()]
    }

    pub fn map_to<F, T>(&self, f: F) -> Vec<Vec<T>>
    where
        F: Fn(&char) -> T,
    {
        let mut res = Vec::new();

        for line in self.map.iter() {
            res.push(line.iter().map(&f).collect::<Vec<T>>());
        }

        res
    }
}

impl Display for CharMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.map.iter() {
            for c in line.iter() {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
