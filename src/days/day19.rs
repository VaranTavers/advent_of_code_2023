use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Decision {
    NextWorkflow(String),
    Accepted,
    Rejected,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Rule {
    pub property: char,
    pub num: i64,
    pub rule: Ordering,
    pub if_true: Either<Box<Self>, Decision>,
    pub if_false: Either<Box<Self>, Decision>,
}

impl Clone for Rule {
    fn clone(&self) -> Self {
        Self {
            property: self.property.clone(),
            num: self.num.clone(),
            rule: self.rule.clone(),
            if_true: self.if_true.clone(),
            if_false: self.if_false.clone(),
        }
    }
}

impl Rule {
    fn parse_into_either(s: &str) -> Either<Box<Self>, Decision> {
        if s == "A" {
            return Either::Right(Decision::Accepted);
        }
        if s == "R" {
            return Either::Right(Decision::Rejected);
        }
        if s.find([':']).is_none() {
            return Either::Right(Decision::NextWorkflow(s.to_owned()));
        }

        let parts = s.split_once(',').unwrap();
        let rule = if parts.0.contains('<') {
            Ordering::Less
        } else if parts.0.contains('>') {
            Ordering::Greater
        } else {
            Ordering::Equal
        };

        let query_and_true = parts.0.split_once(":").unwrap();
        let query_parts = query_and_true.0.split_once(['>', '<', '=']).unwrap();

        let if_true = Rule::parse_into_either(query_and_true.1);
        let if_false = Rule::parse_into_either(parts.1);

        Either::Left(Box::new(Rule {
            property: query_parts.0.chars().nth(0).unwrap(),
            num: query_parts.1.parse::<i64>().unwrap(),
            rule,
            if_true,
            if_false,
        }))
    }

    pub fn verify_rule_once(&self, part: &Part) -> &Either<Box<Rule>, Decision> {
        let cmp_res = match self.property {
            'x' => part.x.cmp(&self.num),
            'm' => part.m.cmp(&self.num),
            'a' => part.a.cmp(&self.num),
            's' => part.s.cmp(&self.num),
            _ => part.x.cmp(&self.num),
        };

        if cmp_res == self.rule {
            return &self.if_true;
        }

        &self.if_false
    }

    pub fn verify_rule(&self, part: &Part) -> Decision {
        let mut res = self.verify_rule_once(part);

        while let Either::Left(rule) = res {
            res = rule.verify_rule_once(part);
        }

        match res {
            Either::Right(dec) => dec.clone(),
            _ => Decision::Rejected,
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = Rule::parse_into_either(s);

        if let Either::Left(x) = val {
            return Ok(*x);
        }
        Err(())
    }
}

pub struct Part {
    pub x: i64,
    pub m: i64,
    pub a: i64,
    pub s: i64,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s[1..s.len() - 1]
            .split(',')
            .map(|x| {
                x.split('=').collect::<Vec<&str>>()[1]
                    .parse::<i64>()
                    .unwrap()
            })
            .collect::<Vec<i64>>();

        Ok(Part {
            x: parts[0],
            m: parts[1],
            a: parts[2],
            s: parts[3],
        })
    }
}

fn lines_to_workflows(lines: &mut Lines<BufReader<File>>) -> HashMap<String, Rule> {
    let mut map = HashMap::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() {
            return map;
        }

        let parts = line.split(['{', '}']).collect::<Vec<&str>>();

        map.insert(parts[0].to_owned(), parts[1].parse::<Rule>().unwrap());
    }

    map
}

fn lines_into_parts(lines: &mut Lines<BufReader<File>>) -> Vec<Part> {
    let mut res = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() {
            return res;
        }

        res.push(line.parse::<Part>().unwrap())
    }
    res
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let mut lines = reader.lines();
    let rules = lines_to_workflows(&mut lines);
    let parts = lines_into_parts(&mut lines);

    let mut sum = 0;
    for part in parts {
        let mut resp = rules["in"].verify_rule(&part);
        while let Decision::NextWorkflow(s) = resp {
            resp = rules[&s].verify_rule(&part);
        }

        if resp == Decision::Accepted {
            sum += part.x + part.m + part.a + part.s;
        }
    }

    Ok(sum)
}

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub x: (i64, i64),
    pub m: (i64, i64),
    pub a: (i64, i64),
    pub s: (i64, i64),
}

impl Limits {
    pub fn get_combinations(&self) -> i64 {
        (self.x.1 - self.x.0 + 1).max(0)
            * (self.m.1 - self.m.0 + 1).max(0)
            * (self.a.1 - self.a.0 + 1).max(0)
            * (self.s.1 - self.s.0 + 1).max(0)
    }
}

fn limit_the_limit(rule: &Rule, limits: &Limits) -> (Limits, Limits) {
    let mut limits_true = limits.clone();
    let mut limits_false = limits.clone();

    let min_1;
    let max_1;
    let min_2;
    let max_2;
    match rule.property {
        'x' => {
            min_1 = &mut limits_true.x.0;
            max_1 = &mut limits_true.x.1;
            min_2 = &mut limits_false.x.0;
            max_2 = &mut limits_false.x.1;
        }
        'm' => {
            min_1 = &mut limits_true.m.0;
            max_1 = &mut limits_true.m.1;
            min_2 = &mut limits_false.m.0;
            max_2 = &mut limits_false.m.1;
        }
        'a' => {
            min_1 = &mut limits_true.a.0;
            max_1 = &mut limits_true.a.1;
            min_2 = &mut limits_false.a.0;
            max_2 = &mut limits_false.a.1;
        }
        's' | _ => {
            min_1 = &mut limits_true.s.0;
            max_1 = &mut limits_true.s.1;
            min_2 = &mut limits_false.s.0;
            max_2 = &mut limits_false.s.1;
        }
    }
    match rule.rule {
        Ordering::Greater => {
            if *min_1 < rule.num {
                *min_1 = rule.num + 1;
            }
            if *max_2 > rule.num {
                *max_2 = rule.num;
            }
        }
        Ordering::Less => {
            if *max_1 > rule.num {
                *max_1 = rule.num - 1;
            }
            if *min_2 < rule.num {
                *min_2 = rule.num;
            }
        }
        Ordering::Equal => {
            if *min_1 < rule.num {
                *min_1 = rule.num;
            }
            if *max_1 > rule.num {
                *max_1 = rule.num;
            }
        }
    }

    (limits_true, limits_false)
}

fn dei(rules: &HashMap<String, Rule>, rule: &Either<Box<Rule>, Decision>, limits: Limits) -> usize {
    if let Either::Right(dec) = rule {
        match dec {
            Decision::NextWorkflow(s) => {
                return dei(
                    rules,
                    &Either::Left(Box::new(rules.get(s).unwrap().clone())),
                    limits,
                )
            }
            Decision::Accepted => return limits.get_combinations() as usize,
            Decision::Rejected => return 0,
        }
    }

    if let Either::Left(rule) = rule {
        let (limits_true, limits_false) = limit_the_limit(&rule, &limits);
        let if_true = dei(rules, &rule.if_true, limits_true);
        let if_false = dei(rules, &rule.if_false, limits_false);
        return if_true + if_false;
    }

    0
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();
    let rules = lines_to_workflows(&mut lines);

    Ok(dei(
        &rules,
        &Either::Left(Box::new(rules.get("in").unwrap().clone())),
        Limits {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    ))
}
