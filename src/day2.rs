use std::fs;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Working with file {filename}");

    let content = fs::read_to_string(filename)
        .expect("Should have been able to read the file in {filename}");

    let lines: Vec<&str> = content.split("\n").collect();
    let score_1: i32 = play_rounds_1(lines.clone());
    println!("The score for this round {score_1}");

    let score_2: i32 = play_rounds_2(lines.clone());
    println!("The score for the second part is {score_2}");
}

fn play_rounds_1(lines: Vec<&str>) -> i32 {
    let results: Vec<i32> = lines.iter().map(|x| play_round_1(*x)).collect();
    let score: i32 = results.iter().sum::<i32>();
    score
}

fn play_rounds_2(lines: Vec<&str>) -> i32 {
    let results: Vec<i32> = lines.iter().map(|x| play_round_2(*x)).collect();
    let score: i32 = results.iter().sum::<i32>();
    score
}

// A = rock, B = paper, C = scissors
// X = rock, Y = paper, Z = scissors
fn play_round_1(line: &str) -> i32 {
    let mut score: i32 = 0;
    if line.contains("X") {
        score += 1;
        if line.contains("A") {
            score += 3;
            return score;
        } else if line.contains("B") {
            return score;
        } else {
            score += 6;
            return score;
        }
    } else if line.contains("Y") {
        score += 2;
        if line.contains("A") {
            score += 6;
            return score;
        } else if line.contains("B") {
            score += 3;
            return score;
        } else {
            return score;
        }
    } else {
        score += 3;
        if line.contains("A") {
            return score;
        } else if line.contains("B") {
            score += 6;
            return score;
        } else {
            score += 3;
            return score;
        }
    }
}

// A = rock, B = paper, C = scissors
// X = rock, Y = paper, Z = scissors
fn play_round_2(line: &str) -> i32 {
    let mut score: i32 = 0;
    // What the opponent plays
    if line.contains("A") {
        score = 1;
    } else if line.contains("B") {
        score = 2;
    } else if line.contains("C") {
        score = 3;
    }

    if line.contains("Y") {
        score += 3;
    } else if line.contains("X") {
        score -= 1;
        score = if score == 0 {3} else {score};
    } else if line.contains("Z") {
        score += 1;
        score = if score == 4 {1} else {score};
        score += 6;
    }

    score
}