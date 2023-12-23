use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Block {
    pub min_z: usize,
    pub min_x: usize,
    pub min_y: usize,
    pub max_z: usize,
    pub max_x: usize,
    pub max_y: usize,
    pub id: usize,
}

impl FromStr for Block {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("{}", s);
        let parts = s.split_once('~').unwrap();
        let one = parts
            .0
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        //println!("ONE: {}, {:?}", parts.0, one);
        let two = parts
            .1
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        //println!("TWO: {}, {:?}", parts.1, two);
        if one[2] < two[2] {
            Ok(Block {
                min_z: one[2],
                min_x: one[0],
                min_y: one[1],
                max_z: two[2],
                max_x: two[0],
                max_y: two[1],
                id: 0,
            })
        } else {
            Ok(Block {
                min_z: two[2],
                min_x: two[0],
                min_y: two[1],
                max_z: one[2],
                max_x: one[0],
                max_y: one[1],
                id: 0,
            })
        }
    }
}

impl Block {
    pub fn intersect_in_x(&self, other: &Self) -> bool {
        let min_1 = self.min_x.min(self.max_x);
        let min_2 = other.min_x.min(other.max_x);
        let max_1 = self.min_x.max(self.max_x);
        let max_2 = other.min_x.max(other.max_x);

        //println!("{} {}, {} {}", min_1, max_1, min_2, max_2);
        !(max_1 < min_2 || min_1 > max_2)
    }

    pub fn intersect_in_y(&self, other: &Self) -> bool {
        let min_1 = self.min_y.min(self.max_y);
        let min_2 = other.min_y.min(other.max_y);
        let max_1 = self.min_y.max(self.max_y);
        let max_2 = other.min_y.max(other.max_y);

        !(max_1 < min_2 || min_1 > max_2)
    }

    pub fn intersect(&self, other: &Self) -> bool {
        self.intersect_in_x(other) && self.intersect_in_y(other)
    }
}

pub fn get_put_down_block(block: Block, blocks: &Vec<Block>) -> (usize, Block) {
    let below = blocks
        .iter()
        .rev()
        .filter(|x| x.intersect(&block))
        .collect::<Vec<&Block>>();

    //println!("Block: {} Intersected by: {:?}", block.id, below);

    let below_block = below.get(0).unwrap_or(&&Block {
        min_z: 0,
        min_x: 0,
        min_y: 0,
        max_z: 0,
        max_x: 0,
        max_y: 0,
        id: usize::MAX,
    });

    let mut copy = block.clone();
    copy.min_z = below_block.max_z + 1;
    copy.max_z = block.max_z - block.min_z + copy.min_z;

    let real_supports = below.iter().filter(|x| x.max_z == below_block.max_z);

    //println!("Block: {} Supported by: {:?}", block.id, real_supports);
    if real_supports.count() > 1 {
        return (usize::MAX, copy);
    }

    (below_block.id, copy)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut blocks = reader
        .lines()
        .enumerate()
        .map(|(i, x)| {
            let mut block = x.unwrap().parse::<Block>().unwrap();
            block.id = i;
            block
        })
        .collect::<Vec<Block>>();

    blocks.sort();

    let mut blocked = blocks.iter().map(|_| false).collect::<Vec<bool>>();

    let mut put_down = Vec::new();

    for block in blocks {
        let (below, b2) = get_put_down_block(block, &put_down);
        if below < usize::MAX {
            blocked[below] = true;
        }

        put_down.push(b2);
        put_down.sort_by(|x, y| x.max_z.cmp(&y.max_z));
    }

    //println!("{:?}", blocked);

    Ok(blocked.iter().filter(|x| !**x).count())
}

pub fn get_put_down_block_2(block: Block, blocks: &Vec<Block>) -> (Vec<usize>, Block) {
    let below = blocks
        .iter()
        .rev()
        .filter(|x| x.intersect(&block))
        .collect::<Vec<&Block>>();

    //println!("Block: {} Intersected by: {:?}", block.id, below);

    let below_block = below.get(0).unwrap_or(&&Block {
        min_z: 0,
        min_x: 0,
        min_y: 0,
        max_z: 0,
        max_x: 0,
        max_y: 0,
        id: usize::MAX,
    });

    let mut copy = block.clone();
    copy.min_z = below_block.max_z + 1;
    copy.max_z = block.max_z - block.min_z + copy.min_z;

    let real_supports: Vec<usize> = below
        .iter()
        .filter(|x| x.max_z == below_block.max_z)
        .map(|x| x.id)
        .collect();

    //println!("{} supported_by {:?}", copy.id, real_supports);

    (real_supports, copy)
}

pub fn find_out_blocks_that_would_move(supported_by: &Vec<Vec<usize>>, i: usize) -> usize {
    let mut is_present = supported_by.iter().map(|_| true).collect::<Vec<bool>>();

    is_present[i] = false;
    let mut changed = true;
    while changed {
        changed = false;
        for (j, v) in supported_by.iter().enumerate() {
            if is_present[j] {
                if v.iter().all(|x| !is_present[*x]) && v.len() != 0 {
                    //println!("All of {} supports have fallen: {:?}", j, v);
                    is_present[j] = false;
                    changed = true;
                }
            }
        }
    }

    is_present.iter().filter(|x| !**x).count() - 1
}

pub fn solution_2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut blocks = reader
        .lines()
        .enumerate()
        .map(|(i, x)| {
            let mut block = x.unwrap().parse::<Block>().unwrap();
            block.id = i;
            block
        })
        .collect::<Vec<Block>>();

    blocks.sort();

    let mut blocked = blocks
        .iter()
        .map(|_| Vec::new())
        .collect::<Vec<Vec<usize>>>();

    let mut supports = blocks
        .iter()
        .map(|_| Vec::new())
        .collect::<Vec<Vec<usize>>>();

    let mut put_down = Vec::new();

    for block in blocks {
        let (mut below, b2) = get_put_down_block_2(block, &put_down);
        for supp in below.iter() {
            //println!("{} supported by {}", b2.id, supp);
            blocked[*supp].push(b2.id);
        }

        supports[b2.id].append(&mut below);

        put_down.push(b2);
        put_down.sort_by(|x, y| x.max_z.cmp(&y.max_z));
    }

    //println!("{:?}", supports);

    let mut sum = 0;
    for i in 0..supports.len() {
        sum += find_out_blocks_that_would_move(&supports, i);
        //println!("{i} {sum}");
    }

    Ok(sum)
}
