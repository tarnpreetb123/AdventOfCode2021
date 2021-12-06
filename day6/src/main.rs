use std::collections::HashMap;
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 6!");
    get_lanternfish(80, "Part 1");
    get_lanternfish(256, "Part 2");
}

fn get_lanternfish(day:usize, part: &str) {
    let file = File::open("./day6/inputday6.txt").unwrap();
    let mut reader = BufReader::new(file).lines();

    let line = reader.next().unwrap().unwrap();
    let filtered: Vec<u64> = line.split(',').map(|j| j.parse().unwrap()).collect();

    let mut fish = HashMap::new();

    for i in filtered{
        let counter = fish.entry(i).or_insert(0);
        *counter += 1;
    }

    for _ in 0..day{
        let mut fish_next = HashMap::new();
        for j in 0..9{
            if j == 0 {
                let num_fish = fish.entry(j).or_insert(0);
                let counter = fish_next.entry(6).or_insert(0);
                *counter += *num_fish;
                let counter = fish_next.entry(8).or_insert(0);
                *counter += *num_fish;
            } else {
                let num_fish = fish.entry(j).or_insert(0);
                let counter = fish_next.entry(j-1).or_insert(0);
                *counter += *num_fish;
            }
        }
        fish = fish_next.clone();
    }

    let fish_total: u64 = fish.values().sum();
    println!("{}: There are: {:?} lantern fish", part, fish_total);
}
