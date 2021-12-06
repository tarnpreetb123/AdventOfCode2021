use std::collections::HashMap;
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 5!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day5/inputday5.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let filtered = reader.map(|s| s.unwrap()
        .split(" -> ")
        .map(|i| i.split(','))
        .flatten()
        .map(|j| j.parse().unwrap())
        .collect::<Vec<i32>>()
    )
        .filter(|line| {
            let (x1, y1) = (*line.get(0).unwrap(), *line.get(1).unwrap());
            let (x2, y2) = (*line.get(2).unwrap(), *line.get(3).unwrap());
            x1 == x2 || y1 == y2
        })
        .collect::<Vec<_>>();

    let mut grid = HashMap::new();

    for line in filtered{
        let (x1, y1) = (*line.get(0).unwrap(), *line.get(1).unwrap());
        let (x2, y2) = (*line.get(2).unwrap(), *line.get(3).unwrap());

        let dx = (x2-x1).signum();
        let dy = (y2-y1).signum();

        let mut x = x1;
        let mut y = y1;

        while (x,y) != (x2+dx, y2+dy){
            let counter = grid.entry((x,y)).or_insert(0);
            *counter += 1;
            x += dx;
            y += dy;
        }
    }
    let number_overlaps = grid.values().filter(|&&x| x > 1).count();
    println!("Part 1: {:?}", number_overlaps);

}

fn part_two(){

    let file = File::open("./day5/inputday5.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let filtered = reader.map(|s| s.unwrap()
        .split(" -> ")
        .map(|i| i.split(','))
        .flatten()
        .map(|j| j.parse().unwrap())
        .collect::<Vec<i32>>()
    ).collect::<Vec<_>>();

    let mut grid = HashMap::new();

    for line in filtered{
        let (x1, y1) = (*line.get(0).unwrap(), *line.get(1).unwrap());
        let (x2, y2) = (*line.get(2).unwrap(), *line.get(3).unwrap());

        let dx = (x2-x1).signum();
        let dy = (y2-y1).signum();

        let mut x = x1;
        let mut y = y1;

        while (x,y) != (x2+dx, y2+dy){
            let counter = grid.entry((x,y)).or_insert(0);
            *counter += 1;
            x += dx;
            y += dy;
        }
    }
    let number_overlaps = grid.values().filter(|&&x| x > 1).count();
    println!("Part 2: {:?}", number_overlaps);
}