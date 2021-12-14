use std::collections::{HashMap};
use std::fs::{File};
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Hello, world! Day 14!");
    get_polymer_score(10, "Part 1");
    get_polymer_score(40, "Part 2");
}

fn get_polymer_score(steps:usize, part: &str) {
    let file = File::open("./day14/inputday14.txt").unwrap();
    let mut reader = BufReader::new(file).lines().map(|x| x.unwrap());

    let polymer_template: Vec<_> = reader.next().unwrap().chars().collect();
    reader.next();

    let mut polymer_pairs = HashMap::new();
    let mut polymer_template_map = HashMap::new();
    let mut polymer_counter = HashMap::new();

    for line in reader {
        let input = line.clone();
        let pair = input.split(" -> ").collect::<Vec<&str>>();
        let key = pair.get(0).unwrap().to_string();
        let value = pair.get(1).unwrap().to_string().chars().next().unwrap();
        polymer_pairs.insert(key, value);
    }

    for i in 0..polymer_template.len() - 1 {
        let char_one = polymer_template[i];
        let char_two = polymer_template[i + 1];
        let mut pair = String::from(char_one);
        pair.push(char_two);
        let counter = polymer_template_map.entry(pair).or_insert(0);
        *counter += 1;
    }

    for char in polymer_template.clone() {
        let counter = polymer_counter.entry(char).or_insert(0);
        *counter += 1;
    }

    for _ in 0..steps {
        for (key, value) in polymer_template_map.clone(){
            let insert_element = polymer_pairs.get(&*key).unwrap();
            let mut element_one = String::from(key.chars().nth(0).unwrap());
            element_one.push(insert_element.clone());
            let mut element_two = String::from(insert_element.clone());
            element_two.push(key.chars().nth(1).unwrap());

            let counter = polymer_counter.entry(insert_element.clone()).or_insert(0);
            *counter += value;

            let counter_key = polymer_template_map.entry(key).or_insert(0);
            *counter_key -= value;

            let counter_element_one = polymer_template_map.entry(element_one).or_insert(0);
            *counter_element_one += value;

            let counter_element_two = polymer_template_map.entry(element_two).or_insert(0);
            *counter_element_two += value;
        }
    }

    let mut counter_values = polymer_counter.values().cloned().collect::<Vec<u64>>();
    counter_values.sort();
    let score:u64 = counter_values[counter_values.len()-1] - counter_values[0];

    println!("{}, {} ", part, score);

}