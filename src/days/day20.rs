use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Module {
    ConjModule(Vec<(String, Pulse)>, Vec<String>),
    FlipFlop(State, Vec<String>),
    Broadcast(Vec<String>),
}

pub fn read_modules(lines: &mut Lines<BufReader<File>>) -> HashMap<String, Module> {
    let mut ret = HashMap::new();
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let parts = line.split_once(" -> ").unwrap();
        let outgoing = parts
            .1
            .split(',')
            .map(|x| x.trim().to_owned())
            .collect::<Vec<String>>();

        let module;
        let name;
        if parts.0 == "broadcaster" {
            module = Module::Broadcast(outgoing.clone());
            name = "broadcaster".to_owned();
        } else if parts.0.starts_with('&') {
            module = Module::ConjModule(Vec::new(), outgoing.clone());
            name = parts.0[1..].to_owned();
        } else {
            module = Module::FlipFlop(State::Off, outgoing.clone());
            name = parts.0[1..].to_owned();
        }

        for node in outgoing.iter() {
            let v = incoming.entry(node.clone()).or_insert(Vec::new());
            v.push(name.clone());
        }

        println!("{name} {:?}", module);
        ret.insert(name, module);
    }

    let keys = ret.keys().cloned().collect::<Vec<String>>();
    for key in keys {
        if let Module::ConjModule(v, _) = ret.get_mut(&key).unwrap() {
            let mut inc = incoming
                .get(&key)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|x| (x.clone(), Pulse::Low))
                .collect::<Vec<(String, Pulse)>>();

            v.append(&mut inc);
        }
    }

    ret
}

pub fn push_button(map: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut signs: VecDeque<(String, String, Pulse)> = VecDeque::new();
    let mut high = 0;
    let mut low = 0;

    signs.push_back(("".to_owned(), "broadcaster".to_owned(), Pulse::Low));

    while let Some((from, name, pulse)) = signs.pop_front() {
        match pulse {
            Pulse::High => high += 1,
            Pulse::Low => low += 1,
        }
        println!("{from} -{:?}-> {name}", pulse);

        if let Some(module) = map.get_mut(&name) {
            match module {
                Module::ConjModule(memory, nexts) => {
                    let vals = memory.iter_mut().find(|(x, _)| x == &from).unwrap();
                    (*vals).1 = pulse;

                    let pulse_to_send = if memory.iter().any(|(_, y)| y == &Pulse::Low) {
                        Pulse::High
                    } else {
                        Pulse::Low
                    };

                    for next in nexts.iter() {
                        signs.push_back((name.clone(), next.clone(), pulse_to_send));
                    }
                }
                Module::FlipFlop(state, nexts) => {
                    if pulse == Pulse::Low {
                        if state == &State::On {
                            *state = State::Off;
                            for next in nexts.iter() {
                                signs.push_back((name.clone(), next.clone(), Pulse::Low));
                            }
                        } else {
                            *state = State::On;
                            for next in nexts.iter() {
                                signs.push_back((name.clone(), next.clone(), Pulse::High));
                            }
                        }
                    }
                }
                Module::Broadcast(nexts) => {
                    for next in nexts.iter() {
                        signs.push_back((name.clone(), next.clone(), pulse));
                    }
                }
            }
        }
    }

    (low, high)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();
    let mut map = read_modules(&mut lines);

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let res = push_button(&mut map);
        low += res.0;
        high += res.1;
    }

    Ok(low * high)
}

pub fn push_button_check_rx(map: &mut HashMap<String, Module>) -> bool {
    let mut signs: VecDeque<(String, String, Pulse)> = VecDeque::new();

    signs.push_back(("".to_owned(), "broadcaster".to_owned(), Pulse::Low));

    while let Some((from, name, pulse)) = signs.pop_front() {
        if name == "rx" && pulse == Pulse::Low {
            return true;
        }

        if let Some(module) = map.get_mut(&name) {
            match module {
                Module::ConjModule(memory, nexts) => {
                    let vals = memory.iter_mut().find(|(x, _)| x == &from).unwrap();
                    (*vals).1 = pulse;

                    let pulse_to_send = if memory.iter().any(|(_, y)| y == &Pulse::Low) {
                        Pulse::High
                    } else {
                        Pulse::Low
                    };

                    for next in nexts.iter() {
                        signs.push_back((name.clone(), next.clone(), pulse_to_send));
                    }
                }
                Module::FlipFlop(state, nexts) => {
                    if pulse == Pulse::Low {
                        if state == &State::On {
                            *state = State::Off;
                            for next in nexts.iter() {
                                signs.push_back((name.clone(), next.clone(), Pulse::Low));
                            }
                        } else {
                            *state = State::On;
                            for next in nexts.iter() {
                                signs.push_back((name.clone(), next.clone(), Pulse::High));
                            }
                        }
                    }
                }
                Module::Broadcast(nexts) => {
                    for next in nexts.iter() {
                        signs.push_back((name.clone(), next.clone(), pulse));
                    }
                }
            }
        }
    }

    false
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();
    let mut map = read_modules(&mut lines);

    let mut i = 1;

    loop {
        //println!("{:?}", map);
        //println!();
        if push_button_check_rx(&mut map) {
            return Ok(i);
        }

        i += 1;
    }
}
