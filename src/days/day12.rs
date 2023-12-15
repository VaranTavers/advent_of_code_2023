use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Pattern {
    pattern: String,
    nums: Vec<usize>,
}

impl Pattern {
    pub fn from_line(line: &str) -> Pattern {
        let parts = line.split(' ').collect::<Vec<&str>>();
        Pattern {
            pattern: parts[0].to_owned(),
            nums: parts[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }
    pub fn from_line_unfolded(line: &str) -> Pattern {
        let parts = line.split(' ').collect::<Vec<&str>>();

        let mut pattern = (parts[0].to_owned() + "?").repeat(5);
        pattern.pop();

        let nums = parts[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .repeat(5);

        Pattern { pattern, nums }
    }
    pub fn match_str_n(&self, s: &str, n: usize) -> bool {
        let mut runs = 0;
        let mut run_i = 0;
        for (c, pc) in s.chars().zip(self.pattern.chars()).take(n) {
            if c != pc && pc != '?' {
                return false;
            }
            if c == '#' {
                runs += 1;
            } else if runs > 0 {
                if run_i >= self.nums.len() || runs != self.nums[run_i] {
                    return false;
                }
                runs = 0;
                run_i += 1;
            }
        }

        if runs > 0 {
            if run_i >= self.nums.len()
                || runs > self.nums[run_i]
                || (runs != self.nums[run_i] && n == s.len())
            {
                return false;
            }
            run_i += 1;
        }
        if n == s.len() && run_i < self.nums.len() {
            return false;
        }
        true
    }
}

fn backtrack(s: &mut String, i: usize, pattern: &Pattern, sum: &mut usize) {
    if i == s.len() {
        if pattern.match_str_n(&s, i) {
            //println!("{s}");
            *sum += 1;
        }
        return;
    }

    if s.chars().nth(i).unwrap() != '?' {
        backtrack(s, i + 1, pattern, sum);
        return;
    }

    s.replace_range(i..(i + 1), "#");
    if pattern.match_str_n(&s, i) {
        backtrack(s, i + 1, pattern, sum);
    }

    s.replace_range(i..(i + 1), ".");
    if pattern.match_str_n(&s, i) {
        backtrack(s, i + 1, pattern, sum);
    }

    s.replace_range(i..(i + 1), "?");
}

fn run_backtrack(pattern: &Pattern) -> usize {
    let mut sum = 0;
    let mut s = pattern.pattern.clone();

    backtrack(&mut s, 0, pattern, &mut sum);

    //println!("{} {sum}", pattern.pattern);

    sum
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader
        .lines()
        .map(|x| Pattern::from_line(&x.unwrap()))
        .collect::<Vec<Pattern>>();

    Ok(lines.iter().map(run_backtrack).sum())
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader
        .lines()
        .map(|x| Pattern::from_line_unfolded(&x.unwrap()))
        .collect::<Vec<Pattern>>();

    Ok(lines.iter().map(run_backtrack).sum())
}
