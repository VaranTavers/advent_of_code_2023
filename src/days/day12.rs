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

    pub fn match_str_j_n_l(&self, s: &Vec<char>, j: usize, l: usize) -> bool {
        //println!("{:?}, {j} {l}", &s[j..s.len()]);
        let mut runs = 0;
        let mut runs_min = 0;
        let mut run_i = l;
        for (_index, c) in s.iter().enumerate().skip(j) {
            if *c == '#' {
                runs += 1;
                runs_min = runs;
            } else if *c == '?' {
                if runs_min > 0 && run_i >= self.nums.len() {
                    return false;
                } else if run_i < self.nums.len() && runs == self.nums[run_i] {
                    return true;
                }
                runs += 1;
            } else if runs > 0 {
                if run_i >= self.nums.len()
                    || runs < self.nums[run_i]
                    || runs_min > self.nums[run_i]
                {
                    return false;
                }
                return true;
            } else {
                return false;
            }
        }

        if runs > 0 {
            if run_i >= self.nums.len() || runs < self.nums[run_i] || runs_min > self.nums[run_i] {
                return false;
            }
            run_i += 1;
        }
        if run_i < self.nums.len() {
            return false;
        }
        true
    }
}

fn dynamic(pattern: &Pattern) -> usize {
    let table_rows = pattern.nums.len() + 1;
    let table_cols = pattern.pattern.chars().count() + 1;

    let mut table: Vec<Vec<usize>> = (0..table_rows)
        .map(|_| (0..table_cols).map(|_| 0).collect())
        .collect();

    let cs = pattern.pattern.chars().collect::<Vec<char>>();
    table[0][table_cols - 1] = 1;
    for (i, c) in cs.iter().enumerate().rev() {
        if *c == '#' {
            break;
        }
        table[0][i] = 1;
    }
    for row in 1..table_rows {
        for col in (0..table_cols - 1).rev() {
            if cs[col] == '.' {
                table[row][col] = table[row][col + 1];
            } else if cs[col] == '#' {
                if pattern.match_str_j_n_l(&cs, col, table_rows - 1 - row) {
                    table[row][col] = table[row - 1]
                        [(col + pattern.nums[table_rows - 1 - row] + 1).min(table_cols - 1)];
                } else {
                    table[row][col] = 0;
                }
            } else {
                table[row][col] = table[row][col + 1];

                if pattern.match_str_j_n_l(&cs, col, table_rows - 1 - row) {
                    table[row][col] += table[row - 1]
                        [(col + pattern.nums[table_rows - 1 - row] + 1).min(table_cols - 1)]
                }
            }
        }
    }

    //Sprintln!("{:?}", table);

    table[table_rows - 1][0]
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
    // Inspired by: https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd6p5gm/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    let lines = reader
        .lines()
        .map(|x| Pattern::from_line_unfolded(&x.unwrap()))
        .collect::<Vec<Pattern>>();

    let res = lines.iter().map(dynamic).collect::<Vec<usize>>();

    //println!("{:?}", res);
    Ok(res.iter().sum())
}
