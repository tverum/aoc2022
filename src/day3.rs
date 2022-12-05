use std::fs;
use std::env;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = content.split("\n").collect();
    println!("Processing items");

    let shared_items = process_lines_1(lines.clone());
    let mut prios: Vec<usize> = Vec::new();
    println!("Shared items:\n---");
    for item in shared_items.iter() {
        let mut prio = ALPHABET.chars().position(|c| c == *item).unwrap();
        prio += 1;
        prios.push(prio);
    }
    println!("Total prio 1 = {}", prios.iter().sum::<usize>());

    let shared_items_2 = process_lines_2(lines.clone());
    let mut prios_2: Vec<usize> = Vec::new();
    println!("Shared items:\n---");
    for item in shared_items_2.iter() {
        let mut prio = ALPHABET.chars().position(|c| c == *item).unwrap();
        prio += 1;
        prios_2.push(prio);
    }
    println!("Total prio 2 = {}", prios_2.iter().sum::<usize>());
}

fn process_lines_1(lines: Vec<&str>) -> Vec<char> {
    let mut shared_items = Vec::new();
    for line in lines {
        let (first, second) = line.split_at(line.len()/2);
        for a in first.chars() {
            if second.contains(a) {
                shared_items.push(a);
                break;
            }
        }
    }
    return shared_items;
}

fn process_lines_2(lines: Vec<&str>) -> Vec<char> {
    let mut shared_items = Vec::new();
    for chunks in lines.chunks(3) {
        for a in chunks[0].chars() {
            if chunks[1].contains(a) && chunks[2].contains(a) {
                shared_items.push(a);
                break;
            }
        }
    }
    return shared_items;
}
