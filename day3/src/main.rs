use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 3!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day3/inputday3.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut ones = [0; 12];
    let mut zeros = [0; 12];

    for line in reader{

        let binary_string = line.unwrap();
        let num: u16 = u16::from_str_radix(&binary_string, 2).unwrap();

        for i in 0..12{
            let bit = num & (1 << i) != 0;
            if bit {
                ones[i] += 1;
            } else {
                zeros[i] += 1;
            }
        }
    }

    let mut gamma_binary = [0; 12];
    let mut gamma = 0;
    let mut epison = 0;

    for i in 0..12{
        if ones[i] > zeros[i]{
            gamma_binary[i] = 1;
        }
        gamma += gamma_binary[i]*(1 << i);
        epison += (1-gamma_binary[i])*(1 << i);
    }

    let result = gamma*epison;
    println!("Part 1: {}", result);
}

fn part_two(){
    let file = File::open("./day3/inputday3.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut binary_nums_oxygen = Vec::new();
    let mut binary_nums_carbon = Vec::new();
    let mut ones_oxygen = [0; 12];
    let mut zeros_oxygen = [0; 12];
    let mut ones_carbon = [0; 12];
    let mut zeros_carbon = [0; 12];

    for line in reader{
        let binary_string = line.unwrap();
        let num: u32 = u32::from_str_radix(&binary_string, 2).unwrap();
        binary_nums_oxygen.push(num);
        binary_nums_carbon.push(num);
    }

    for i in 0..12{

        for x in binary_nums_oxygen.clone(){
            let bit = x & (1 << (11-i)) != 0;
            if bit {
                ones_oxygen[i] += 1;
            } else {
                zeros_oxygen[i] += 1;
            }
        }

        let keep_oxygen = if ones_oxygen[i] >= zeros_oxygen[i]{1} else {0};

        binary_nums_oxygen.retain(|x| {
            let bit_bool = x & (1 << (11-i)) != 0;
            let mut bit = 0;
            if bit_bool{
                bit = 1;
            }
            return bit == keep_oxygen;
        });

        if binary_nums_oxygen.len() == 1{
            break;
        }
    }


    for i in 0..12{
        for x in binary_nums_carbon.clone(){
            let bit = x & (1 << (11-i)) != 0;
            if bit {
                ones_carbon[i] += 1;
            } else {
                zeros_carbon[i] += 1;
            }
        }

        let keep_carbon = if ones_carbon[i] >= zeros_carbon[i]{0} else {1};
        binary_nums_carbon.retain(|x| {
            let bit_bool = x & (1 << (11-i)) != 0;
            let mut bit = 0;
            if bit_bool{
                bit = 1;
            }
            return bit == keep_carbon;
        });

        if binary_nums_carbon.len() == 1{
            break;
        }
    }

    let result = (binary_nums_oxygen.get(0).unwrap()) * (binary_nums_carbon.get(0).unwrap());
    println!("Part 2: {}", result);
}