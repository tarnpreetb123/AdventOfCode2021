use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 7!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day7/inputday7.txt").unwrap();
    let mut reader = BufReader::new(file).lines();

    let line = reader.next().unwrap().unwrap();
    let mut crab_pos: Vec<i32> = line.split(',').map(|j| j.parse().unwrap()).collect();

    crab_pos.sort();
    let mid = crab_pos.len()/2;
    let median: i32 = crab_pos[mid];

    //Calculate fuel
    let mut fuel: i32 = 0;
    for x in crab_pos{
        fuel += (x-median).abs();
    }

    println!("Part 1: Median: {} Fuel: {}", median, fuel);
}

fn part_two() {
    let file = File::open("./day7/inputday7.txt").unwrap();
    let mut reader = BufReader::new(file).lines();

    let line = reader.next().unwrap().unwrap();
    let crab_pos: Vec<i32> = line.split(',').map(|j| j.parse().unwrap()).collect();

    let sum: i32 = crab_pos.iter().sum();
    let mean: f32 = (sum as f32/crab_pos.len() as f32).floor();

    //Calculate fuel
    let mut fuel: i32 = 0;
    for x in crab_pos{
        let n = (x-mean as i32).abs();
        fuel += (n*(n+1))/2;
    }

    println!("Part 2: Mean: {} Fuel: {}", mean, fuel);
}


