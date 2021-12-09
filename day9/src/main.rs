use std::collections::{HashSet, VecDeque};
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 9!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day9/inputday9.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());
    let map = reader.map(|s| s.bytes().map(|x| x - b'0').collect::<Vec<u8>>()).collect::<Vec<_>>();

    let mut score: u32 = 0;
    for i in 0..map.len(){
        for j in 0..map[i].len(){
            let current_num = map[i][j];
            let mut surrounding_nums = vec![];
            if i != 0{
                surrounding_nums.push(map[i-1][j]);
            }
            if i != map.len() - 1{
                surrounding_nums.push(map[i + 1][j]);
            }
            if j != 0{
                surrounding_nums.push(map[i][j -1]);
            }
            if j != map[i].len() - 1 {
                surrounding_nums.push(map[i][j + 1]);
            }

            let lowest = surrounding_nums.iter().all(|x| x > &current_num);
            if lowest{
                score += 1 + map[i][j] as u32;
            }
        }
    }

    println!("Part 1: Score (Low points): {}", score);
}

fn part_two(){

    let file = File::open("./day9/inputday9.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());
    let map = reader.map(|s| s.bytes().map(|x| x - b'0').collect::<Vec<u8>>()).collect::<Vec<_>>();
    let mut lowest_points = HashSet::new();

    for i in 0..map.len(){
        for j in 0..map[i].len(){
            let current_num = map[i][j];
            let mut surrounding_nums = vec![];
            if i != 0{
                surrounding_nums.push(map[i-1][j]);
            }
            if i != map.len() - 1{
                surrounding_nums.push(map[i + 1][j]);
            }
            if j != 0{
                surrounding_nums.push(map[i][j -1]);
            }
            if j != map[i].len() - 1 {
                surrounding_nums.push(map[i][j + 1]);
            }

            let lowest = surrounding_nums.iter().all(|x| x > &current_num);
            if lowest{
                lowest_points.insert((i, j));
            }
        }
    }


    let mut basin_sizes: Vec<usize> = vec![];

    //BFS to find basin sizes
    for lowest in lowest_points.clone() {
        let mut neighbours: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = HashSet::new();
        neighbours.push_back(lowest);
        visited.insert(lowest);

        while !neighbours.is_empty() {
            let current_node = neighbours.pop_front().unwrap();
            let (i,j) = current_node;

            if i != 0 && !visited.contains(&(i-1, j)) && map[i-1][j] != 9{
                visited.insert((i-1, j));
                neighbours.push_back((i-1, j));
            }
            if i != map.len() - 1 && !visited.contains(&(i+1, j)) && map[i+1][j] != 9{
                visited.insert((i+1, j));
                neighbours.push_back((i+1, j));
            }
            if j != 0 && !visited.contains(&(i, j-1)) && map[i][j-1] != 9{
                visited.insert((i, j-1));
                neighbours.push_back((i, j-1));
            }
            if j != map[i].len() - 1 && !visited.contains(&(i, j + 1)) && map[i][j+1] != 9{
                visited.insert((i, j+1));
                neighbours.push_back((i, j+1));
            }
        }

        basin_sizes.push(visited.len())
    }


    basin_sizes.sort();
    let score: usize = basin_sizes.iter().rev().take(3).product();

    println!("Part 2: Basin Score (Top 3): {}", score );

}