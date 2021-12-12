use std::collections::{HashSet};
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 11!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day11/inputday11.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());
    let mut map = reader.map(|s| s.bytes().map(|x| x - b'0').collect::<Vec<u8>>()).collect::<Vec<_>>();

    let mut score: u64 = 0;
    for _ in 0..100{
        let mut update_octo: Vec<(usize, usize)> = vec![];
        let mut flashed_octo = HashSet::new();

        //Update map
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                map[i][j] += 1;
            }
        }

        //Flash
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] > 9{
                    flashed_octo.insert((i, j));
                    update_octo.push((i-1, j-1));
                    update_octo.push((i-1, j));
                    update_octo.push((i-1, j+1));
                    update_octo.push((i, j-1));
                    update_octo.push((i, j+1));
                    update_octo.push((i+1, j-1));
                    update_octo.push((i+1, j));
                    update_octo.push((i+1, j+1));
                }
            }
        }

        //Flash update
        while !update_octo.is_empty(){
            let (i, j) = update_octo.pop().unwrap();
            if !flashed_octo.contains(&(i, j)){
                if map.get(i).and_then(|row| row.get(j)).is_none() {
                    continue;
                }
                map[i][j] += 1;
                if map[i][j] > 9{
                    flashed_octo.insert((i, j));
                    update_octo.push((i-1, j-1));
                    update_octo.push((i-1, j));
                    update_octo.push((i-1, j+1));
                    update_octo.push((i, j-1));
                    update_octo.push((i, j+1));
                    update_octo.push((i+1, j-1));
                    update_octo.push((i+1, j));
                    update_octo.push((i+1, j+1));
                }
            }
        }

        //Update map for next step
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] > 9{
                    map[i][j] = 0;
                }
            }
        }
        score += flashed_octo.len() as u64;
    }

    println!("Part 1: Score: {}", score);
}

fn part_two(){
    let file = File::open("./day11/inputday11.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());
    let mut map = reader.map(|s| s.bytes().map(|x| x - b'0').collect::<Vec<u8>>()).collect::<Vec<_>>();

    let mut step_count: u64 = 0;
    let mut flashed_octo = HashSet::new();
    while flashed_octo.len() != 100{
        let mut update_octo: Vec<(usize, usize)> = vec![];
        flashed_octo = HashSet::new();

        //Update map
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                map[i][j] += 1;
            }
        }

        //Flash
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] > 9{
                    flashed_octo.insert((i, j));
                    update_octo.push((i-1, j-1));
                    update_octo.push((i-1, j));
                    update_octo.push((i-1, j+1));
                    update_octo.push((i, j-1));
                    update_octo.push((i, j+1));
                    update_octo.push((i+1, j-1));
                    update_octo.push((i+1, j));
                    update_octo.push((i+1, j+1));
                }
            }
        }


        //Flash update
        while !update_octo.is_empty(){
            let (i, j) = update_octo.pop().unwrap();
            if !flashed_octo.contains(&(i, j)){
                if map.get(i).and_then(|row| row.get(j)).is_none() {
                    continue;
                }
                map[i][j] += 1;
                if map[i][j] > 9{
                    flashed_octo.insert((i, j));
                    update_octo.push((i-1, j-1));
                    update_octo.push((i-1, j));
                    update_octo.push((i-1, j+1));
                    update_octo.push((i, j-1));
                    update_octo.push((i, j+1));
                    update_octo.push((i+1, j-1));
                    update_octo.push((i+1, j));
                    update_octo.push((i+1, j+1));
                }
            }
        }

        //Update map for next step
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] > 9{
                    map[i][j] = 0;
                }
            }
        }
        step_count += 1;
    }



    println!("Part 2: Step number: {}", step_count);
}