use std::collections::VecDeque;
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 1!");

    part_one();
    part_two();

}

fn part_one(){
    let file = File::open("./day1/input.txt").unwrap();
    let mut reader = BufReader::new(file).lines();

    let first_line = reader.next().unwrap().unwrap().parse().unwrap();

    let mut num_past: u32 = first_line;
    let mut num_increase = 0;

    for line in reader{
        let num_current = line.unwrap().parse().unwrap();

        if num_current > num_past {
            num_increase += 1;
        }

        num_past = num_current

    }

    println!("Part 1: {}", num_increase);
}

fn part_two(){
    let file = File::open("./day1/input.txt").unwrap();
    let mut reader = BufReader::new(file).lines();

    // let first_line = reader.next().unwrap().unwrap().parse().unwrap();
    // let window_first_value: u32 = reader.next().unwrap().unwrap().parse().unwrap();
    let mut window: VecDeque<u32> = VecDeque::new();
    let mut num_increase = 0;

    // window_past[1] = window_first_value;

    for _ in 0..3{
        window.push_back(reader.next().unwrap().unwrap().parse().unwrap());
        // println!("{}", i);
        // println!("{}", window.get(i).unwrap());
    }

    for line in reader{
        let num_current = line.unwrap().parse().unwrap();
        let window_past = window.get(0).unwrap() + window.get(1).unwrap() + window.get(2).unwrap();
        window.pop_front();
        window.push_back(num_current);
        let window_current = window.get(0).unwrap() + window.get(1).unwrap() + window.get(2).unwrap();
        if window_current > window_past {
            num_increase += 1;
        }

    }

    println!("Part 2: {}", num_increase);
}

