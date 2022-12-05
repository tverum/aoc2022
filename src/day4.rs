use std::fs;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Working with file {filename}");

    let content = fs::read_to_string(filename)
        .expect("Should have been able to read the file in {filename}");

    let lines: Vec<&str> = content.split("\n").collect();

    let pairs = process_lines_1(lines.clone());
    println!("Amount of qualified assigment pairs {}", pairs);

    let pairs_2 = process_lines_2(lines.clone());
    println!("Amount of qualified assigment pairs {}", pairs_2);
}

fn process_lines_1(lines: Vec<&str>) -> i32 {
    let mut amount = 0;
    for assigment in lines {
        let mut line_split = assigment.split(",");
        let range_a = line_split.next().unwrap();
        let range_b = line_split.next().unwrap();

        let mut a_split = range_a.split("-");
        let mut b_split = range_b.split("-");

        let (min_a_s, max_a_s) = (a_split.next().unwrap(), a_split.next().unwrap());
        let (min_b_s, max_b_s) = (b_split.next().unwrap(), b_split.next().unwrap());

        let min_a = min_a_s.parse::<i32>().unwrap();
        let min_b = min_b_s.parse::<i32>().unwrap();
        let max_a = max_a_s.parse::<i32>().unwrap();
        let max_b = max_b_s.parse::<i32>().unwrap();

        if min_a <= min_b && max_a >= max_b {
            amount += 1;
        } else if min_a >= min_b && max_a <= max_b {
            amount += 1;
        }
    }
    return amount;
}

fn process_lines_2(lines: Vec<&str>) -> i32 {
    let mut amount = 0;
    for assigment in lines {
        let mut line_split = assigment.split(",");
        let range_a = line_split.next().unwrap();
        let range_b = line_split.next().unwrap();

        let mut a_split = range_a.split("-");
        let mut b_split = range_b.split("-");

        let (min_a_s, max_a_s) = (a_split.next().unwrap(), a_split.next().unwrap());
        let (min_b_s, max_b_s) = (b_split.next().unwrap(), b_split.next().unwrap());

        let min_a = min_a_s.parse::<i32>().unwrap();
        let min_b = min_b_s.parse::<i32>().unwrap();
        let max_a = max_a_s.parse::<i32>().unwrap();
        let max_b = max_b_s.parse::<i32>().unwrap();

        if min_a <= min_b && max_a >= min_b {
            amount += 1;
        } else if min_a >= min_b && min_a <= max_b {
            amount += 1;
        }
    }
    return amount;
}