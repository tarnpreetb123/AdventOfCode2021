use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 4!");
    part_one();
    part_two();
}

fn part_one() {
    let file = File::open("./day4/inputday4.txt").unwrap();
    let mut reader = BufReader::new(file).lines().peekable();

    let next_line = reader.next();
    let inputs = next_line.unwrap().unwrap();
    let drawn_numbers: Vec<u32> = inputs.split(',').map(|x| x.parse().unwrap()).collect();

    //Read in puzzles
    let mut puzzles: Vec<Vec<Vec<u32>>> = vec![];

    reader.next();
    while reader.peek().is_some(){
        let mut puzzle: Vec<Vec<u32>> = vec![];
        for _ in 0..5{
            let line = reader.next().unwrap().unwrap();
            let puzzle_row: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            puzzle.push(puzzle_row);
        }
        puzzles.push(puzzle.clone());
        reader.next();
    }

    let mut marker_puzzle: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; 5]; 5]; puzzles.len()];
    'outer: for num_drawn in drawn_numbers {
        for (index, puzzle) in puzzles.iter().enumerate(){
            //mark value in puzzle
            for i in 0..5{
                if puzzle.get(i).unwrap().contains(&num_drawn){
                    let j = puzzle.get(i).unwrap().iter().position(|x| x == &num_drawn).unwrap();
                    marker_puzzle[index][i][j] = 1;
                }
            }
            //check if puzzle wins
            let mut win = false;
            let mut sum;
            let check_puzzle = marker_puzzle.get(index).unwrap();
            //check rows
            for i in 0..5{
                sum = 0;
                for j in check_puzzle.get(i).unwrap().iter(){
                    sum += j;
                    if sum == 5 {win = true}
                }
            }

            //check cols
            for col in 0..5{
                sum = 0;
                for row in 0..5{
                    sum += check_puzzle.get(row).unwrap().get(col).unwrap();
                }
                if sum == 5{
                    win = true;
                }
            }

            //if win check score and return
            let mut score = 0;
            if win {
                for i in 0..5{
                    for j in 0..5{
                        let puzzle_value = puzzle.get(i).unwrap().get(j).unwrap();
                        let check_value = check_puzzle.get(i).unwrap().get(j).unwrap();
                        if *check_value == 0{
                            score += puzzle_value;
                        }
                    }
                }
                score*= num_drawn;
                println!("Part 1: winning puzzle score: {}", score);
                break 'outer;
            }
        }
    }
}

fn part_two() {
    let file = File::open("./day4/inputday4.txt").unwrap();
    let mut reader = BufReader::new(file).lines().peekable();

    let next_line = reader.next();
    let inputs = next_line.unwrap().unwrap();
    let drawn_numbers: Vec<u32> = inputs.split(',').map(|x| x.parse().unwrap()).collect();

    //Read in puzzles
    let mut puzzles: Vec<Vec<Vec<u32>>> = vec![];

    reader.next();
    while reader.peek().is_some(){
        let mut puzzle: Vec<Vec<u32>> = vec![];
        for _ in 0..5{
            let line = reader.next().unwrap().unwrap();
            let puzzle_row: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            puzzle.push(puzzle_row);
        }
        puzzles.push(puzzle.clone());
        reader.next();
    }

    let mut marker_puzzle: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; 5]; 5]; puzzles.len()];
    let mut winning_scores = vec![];
    let mut remove_puzzle: Vec<Vec<Vec<u32>>> = vec![];
    let mut remove_marker_puzzle: Vec<Vec<Vec<u32>>> = vec![];

    for num_drawn in drawn_numbers {
        for (index, puzzle) in puzzles.iter().enumerate(){
            //mark value in puzzle
            for i in 0..5{
                if puzzle.get(i).unwrap().contains(&num_drawn){
                    let j = puzzle.get(i).unwrap().iter().position(|x| x == &num_drawn).unwrap();
                    marker_puzzle[index][i][j] = 1;
                }
            }
            //check if puzzle wins
            let mut win = false;
            let mut sum;
            let check_puzzle = marker_puzzle.get(index).unwrap();
            //check rows
            for i in 0..5{
                sum = 0;
                for j in check_puzzle.get(i).unwrap().iter(){
                    sum += j;
                    if sum == 5 {win = true}
                }
            }

            //check cols
            for col in 0..5{
                sum = 0;
                for row in 0..5{
                    sum += check_puzzle.get(row).unwrap().get(col).unwrap();
                }
                if sum == 5{
                    win = true;
                }
            }

            //if win check score and return
            let mut score = 0;
            if win {
                for i in 0..5{
                    for j in 0..5{
                        let puzzle_value = puzzle.get(i).unwrap().get(j).unwrap();
                        let check_value = check_puzzle.get(i).unwrap().get(j).unwrap();
                        if *check_value == 0{
                            score += puzzle_value;
                        }
                    }
                }
                score*= num_drawn;
                winning_scores.push(score);

                remove_puzzle.push(puzzle.clone());
                remove_marker_puzzle.push(check_puzzle.clone());

            }
        }

        puzzles.retain(|x| {
            let mut result = true;
            for i in remove_puzzle.clone(){
                if *x == i{
                    result = false;
                }
            }
            result
        });

        marker_puzzle.retain(|x| {
            let mut result = true;
            for i in remove_marker_puzzle.clone(){
                if *x == i{
                    result = false;
                }
            }
            result
        });

        remove_puzzle = vec![];
        remove_marker_puzzle = vec![];

    }

    println!("Part 2: winning puzzle score: {}", winning_scores.last().unwrap());

}