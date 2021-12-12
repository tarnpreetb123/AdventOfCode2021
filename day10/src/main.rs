use std::collections::{HashMap, HashSet};
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 10!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day10/inputday10.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());
    let mut score_table = HashMap::new();
    score_table.insert(')', 3);
    score_table.insert(']', 57);
    score_table.insert('}', 1197);
    score_table.insert('>', 25137);

    let mut mirror_bracket = HashMap::new();
    let mut opening_brackets = HashSet::new();
    mirror_bracket.insert('(', ')');
    mirror_bracket.insert('[', ']');
    mirror_bracket.insert('{', '}');
    mirror_bracket.insert('<', '>');
    opening_brackets.insert('(');
    opening_brackets.insert('[');
    opening_brackets.insert('{');
    opening_brackets.insert('<');


    let mut score: u32 = 0;

    for line in reader{
        let brackets: Vec<_> = line.chars().collect();
        let mut stack = vec![];
        'currentLine: for char in brackets{
            if opening_brackets.contains(&char){
                stack.push(char);
            }else {
                let last_opening_bracket = stack.pop().unwrap();
                let mirror_bracket = mirror_bracket.get(&last_opening_bracket).unwrap();
                if mirror_bracket != &char{
                    score += score_table.get(&char).unwrap();
                    break 'currentLine;
                }
            }
        }

    }

    println!("Part 1: Score: {}", score);
}

fn part_two(){
    let file = File::open("./day10/inputday10.txt").unwrap();
    let reader = BufReader::new(file).lines().map(|x| x.unwrap());
    let mut score_table = HashMap::new();
    score_table.insert(')', 1);
    score_table.insert(']', 2);
    score_table.insert('}', 3);
    score_table.insert('>', 4);

    let mut mirror_bracket = HashMap::new();
    let mut opening_brackets = HashSet::new();
    mirror_bracket.insert('(', ')');
    mirror_bracket.insert('[', ']');
    mirror_bracket.insert('{', '}');
    mirror_bracket.insert('<', '>');
    opening_brackets.insert('(');
    opening_brackets.insert('[');
    opening_brackets.insert('{');
    opening_brackets.insert('<');


    let mut score: Vec<u64> = vec![];

    for line in reader{
        let brackets: Vec<_> = line.chars().collect();
        let mut stack = vec![];
        let mut corrupted = false;
        'currentLine: for char in brackets{
            if opening_brackets.contains(&char){
                stack.push(char);
            }else {
                let last_opening_bracket = stack.pop().unwrap();
                let mirror_bracket = mirror_bracket.get(&last_opening_bracket).unwrap();
                if mirror_bracket != &char{
                    corrupted = true;
                    break 'currentLine;
                }
            }
        }

        if !corrupted && stack.len() != 0{
            let mut local_score: u64 = 0;
            for _ in 0..stack.len(){
                let opening = stack.pop().unwrap();
                let closing = mirror_bracket.get(&opening).unwrap();
                local_score *= 5;
                local_score += score_table.get(&closing).unwrap();
            }
            score.push(local_score);
        }

    }

    score.sort();
    let middle_score = score.get(score.len()/2).unwrap();

    println!("Part 2: Score: {}",middle_score);
}