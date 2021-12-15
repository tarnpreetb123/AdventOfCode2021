use std::collections::{HashMap, VecDeque};
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 12!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day12/inputday12.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader {
        let input = line.clone();
        let mut pair = input.split("-");
        let key = pair.next().unwrap().to_string();
        let value = pair.next().unwrap().to_string();
        map.entry(key.clone()).or_insert(Vec::new()).push(value.clone());
        map.entry(value.clone()).or_insert(vec![]).push(key.clone());
    }

    let mut neighbours: VecDeque<Vec<String>> = VecDeque::new();
    neighbours.push_back(vec![String::from("start")]);

    let mut num_paths = 0;

    while !neighbours.is_empty() {
        let current_path = neighbours.pop_front().unwrap();
        let current_node = current_path.last().unwrap();

        if current_node == "end"{
            num_paths += 1;
            continue;
        }

        for connection in map.get(current_node).unwrap().iter(){
            if connection == "start"{
                continue;
            }

            if connection.chars().next().unwrap().is_ascii_lowercase() && current_path.contains(connection){
                continue;
            }
            let mut new_path = current_path.clone();
            new_path.push(connection.clone());
            neighbours.push_back(new_path.clone());
        }
    }

    println!("Part 1: paths:{}", num_paths);
}

fn part_two() {
    let file = File::open("./day12/inputday12.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader {
        let input = line.clone();
        let mut pair = input.split("-");
        let key = pair.next().unwrap().to_string();
        let value = pair.next().unwrap().to_string();
        map.entry(key.clone()).or_insert(Vec::new()).push(value.clone());
        map.entry(value.clone()).or_insert(vec![]).push(key.clone());
    }

    let mut neighbours: VecDeque<(Vec<String>, bool)> = VecDeque::new();
    neighbours.push_back((vec![String::from("start")], false));

    let mut num_paths = 0;

    while !neighbours.is_empty() {
        let (current_path, twice) = neighbours.pop_front().unwrap();
        let current_node = current_path.last().unwrap();

        if current_node == "end"{
            num_paths += 1;
            continue;
        }

        for connection in map.get(current_node).unwrap().iter(){
            let mut is_twice = twice;
            if connection == "start"{
                continue;
            }

            if connection.chars().next().unwrap().is_ascii_lowercase(){
                if current_path.contains(connection){
                    if twice{
                        continue;
                    }
                    is_twice = true;
                }
            }

            let mut new_path = current_path.clone();
            new_path.push(connection.clone());
            neighbours.push_back((new_path.clone(), is_twice));
        }
    }

    println!("Part 2: paths:{}", num_paths);
}