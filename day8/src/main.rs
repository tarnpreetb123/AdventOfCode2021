use std::collections::HashMap;
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 8!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day8/inputday8.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut count_nums = 0;
    for line in reader{
        let current_line = line.unwrap();
        let a: Vec<&str> = current_line.split(" | ").collect();
        let _pattern: Vec<&str> = a[0].split_whitespace().collect();
        let output: Vec<&str> = a[1].split_whitespace().collect();

        count_nums += output.iter().filter(|x| [2,3,4,7].contains(&x.len())).count();
    }

    println!("Part 1: Count: {}", count_nums);
}

fn part_two() {
    let file = File::open("./day8/inputday8.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut count_nums = 0;
    // a 8;
    // b 6;
    // c 8;
    // d 7;
    // e 4;
    // f 9;
    // g 7;

    let mut look_up_table = HashMap::new();
    look_up_table.insert(42, 0);
    look_up_table.insert(17, 1);
    look_up_table.insert(34, 2);
    look_up_table.insert(39, 3);
    look_up_table.insert(30, 4);
    look_up_table.insert(37, 5);
    look_up_table.insert(41, 6);
    look_up_table.insert(25, 7);
    look_up_table.insert(49, 8);
    look_up_table.insert(45, 9);

    for line in reader{
        let current_line = line.unwrap();
        let a: Vec<&str> = current_line.split(" | ").collect();
        let pattern: Vec<&str> = a[0].split_whitespace().collect();
        let output: Vec<&str> = a[1].split_whitespace().collect();
        let mut character_table = HashMap::new();

        for digit in pattern{
            for char in digit.chars(){
                let entry = character_table.entry(char).or_insert(0);
                *entry += 1
            }
        }

        let mut i = 3;
        for out in output.clone() {
            let mut sum = 0;
            for char in out.chars() {
                sum += character_table.get(&char).unwrap();
            }
            let digit = look_up_table.get(&sum).unwrap();
            count_nums += digit*(10_i32.pow(i));
            i -= 1;
        }
    }

    println!("Part 2: Count: {}", count_nums);
}


