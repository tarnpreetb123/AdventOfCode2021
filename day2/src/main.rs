
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 2!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day2/inputday2.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut num_horizontal = 0;
    let mut num_depth = 0;

    for line in reader{

        let word = line.unwrap();
        let mut words = word.split_whitespace();
        let command  = words.next().unwrap();
        let num: u32 = words.next().unwrap().parse().unwrap();
        // println!("{} tstin {}", command, num);

        if command == "forward"{
            num_horizontal += num;
        } else if command == "up"{
            num_depth -= num;
        } else if command == "down" {
            num_depth += num;
        }

    }

    let total = num_depth*num_horizontal;
    println!("Part 1: {}", total);
}

fn part_two() {
    let file = File::open("./day2/inputday2.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut num_horizontal = 0;
    let mut num_depth = 0;
    let mut aim = 0;

    for line in reader{

        let word = line.unwrap();
        let mut words = word.split_whitespace();
        let command  = words.next().unwrap();
        let num: u32 = words.next().unwrap().parse().unwrap();
        // println!("{} tstin {}", command, num);

        if command == "forward"{
            num_horizontal += num;
            num_depth += aim*num;
        } else if command == "up"{
            aim -= num;
        } else if command == "down" {
            aim += num;
        }

    }

    let total = num_depth*num_horizontal;
    println!("Part 2: {}", total);
}