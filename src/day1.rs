use std::fs;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split("\n").collect();
    let bags: Vec<Vec<i32>> = parse_bags(lines);
    let mut weights: Vec<i32> = calculate_weights(bags);
    weights.sort();
    weights.reverse();

    let max_weight: i32 = *(weights.iter().max().unwrap());
    println!("The heighest weight carried by an elf is {max_weight}");

    let total_weight = weights[0] + weights[1] + weights[2];
    println!("The total weight of the three heaviest bags is {total_weight}");
}

fn parse_bags(lines: Vec<&str>)-> Vec<Vec<i32>>{
    let mut bags = Vec::new();
    let mut bag = Vec::new();
    for line in lines {
        if line.is_empty() {
            bags.push(bag);
            bag = Vec::new();
        }
        else {
            bag.push(line.parse::<i32>().unwrap());
        }
    }
    bags.push(bag);
    bags
}

fn calculate_weights(bags: Vec<Vec<i32>>) -> Vec<i32> {
    bags.iter().map(|x| x.iter().sum()).collect()
}