use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 15!");
    part_one();
    part_two();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: u32,
    position: (usize, usize)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn part_one() {
    let file = File::open("./day15/inputday15.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());
    let map = reader.map(|s| s.bytes().map(|x| x - b'0').collect::<Vec<u8>>()).collect::<Vec<_>>();

    let mut risk_level: HashMap<(usize, usize), u32> = HashMap::new();
    let mut neighbours = BinaryHeap::new();
    let mut lowest_risk = 0;

    neighbours.push(State{cost: 0, position: (0, 0)});

    while let Some(State {cost, position}) = neighbours.pop(){
        if position == (map.len() - 1, map[0].len() - 1){
            lowest_risk = cost;
            break;
        }

        let x: usize = position.0;
        let y: usize = position.1;

        let mut surrounding_nums: Vec<(usize, usize)> = vec![];
        if x != 0{
            surrounding_nums.push((x-1, y));
        }
        if x != map.len() - 1{
            surrounding_nums.push((x + 1, y));
        }
        if y != 0{
            surrounding_nums.push((x,y -1));
        }
        if y != map[x].len() - 1 {
            surrounding_nums.push((x,y + 1));
        }

        for (x1,y1) in surrounding_nums {
            if map.get(x1).and_then(|row| row.get(y1)).is_none() {
                continue;
            }

            let cost_new: u32 = cost + map[x1][y1] as u32;
            let cost_current = risk_level.entry((x1, y1)).or_insert(u32::MAX);
            if cost_new < *cost_current {
                *cost_current = cost_new;
                neighbours.push(State {cost: cost_new, position: (x1, y1)})
            }
        }
    }
    println!("Part 1: risk:{}", lowest_risk);
}

fn part_two(){

    let file = File::open("./day15/inputday15.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());
    let map = reader.map(|s| s.bytes().map(|x| x - b'0').collect::<Vec<u8>>()).collect::<Vec<_>>();

    let mut expanded_map: Vec<Vec<u8>> = vec![vec![0; 5*map.len()]; 5*map[0].len()];

    for i in 0..5*map.len(){
        for j in 0..5*map[i % map.len()].len(){
            let x_tile: u8 = (i / map.len()) as u8;
            let y_tile: u8 = (j / map.len()) as u8;
            let mut cost = map[i % map.len()][j % map[i % map.len()].len()] + x_tile + y_tile;
            if cost > 9 {
                cost -= 9;
            };
            expanded_map[i][j] = cost;
        }
    }

    let mut risk_level: HashMap<(usize, usize), u32> = HashMap::new();
    let mut neighbours = BinaryHeap::new();
    let mut lowest_risk = 0;

    neighbours.push(State{cost: 0, position: (0, 0)});

    while let Some(State {cost, position}) = neighbours.pop(){
        if position == (expanded_map.len() - 1, expanded_map[0].len() - 1){
            lowest_risk = cost;
            break;
        }

        let x: usize = position.0;
        let y: usize = position.1;

        let mut surrounding_nums: Vec<(usize, usize)> = vec![];
        if x != 0{
            surrounding_nums.push((x-1, y));
        }
        if x != expanded_map.len() - 1{
            surrounding_nums.push((x + 1, y));
        }
        if y != 0{
            surrounding_nums.push((x,y -1));
        }
        if y != expanded_map[x].len() - 1 {
            surrounding_nums.push((x,y + 1));
        }

        for (x1,y1) in surrounding_nums {
            if expanded_map.get(x1).and_then(|row| row.get(y1)).is_none() {
                continue;
            }

            let cost_new: u32 = cost + expanded_map[x1][y1] as u32;
            let cost_current = risk_level.entry((x1, y1)).or_insert(u32::MAX);
            if cost_new < *cost_current {
                *cost_current = cost_new;
                neighbours.push(State {cost: cost_new, position: (x1, y1)})
            }
        }
    }
    println!("Part 2: risk:{}", lowest_risk);

}