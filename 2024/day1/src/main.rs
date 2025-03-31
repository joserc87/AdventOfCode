use std::fs::read_to_string;
use std::collections::HashMap;

struct ProblemInput {
    first_list: Vec<i32>,
    second_list: Vec<i32>,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        lines.push(line.to_string());
    }

    lines
}

fn read_problem_input_from_lines(lines: Vec<String>) -> ProblemInput {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in lines {
        // split line by space
        let mut parts = line.split_whitespace();
        let left = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();
        let left_num = left.parse::<i32>().unwrap();
        let right_num = right.parse::<i32>().unwrap();

        first_list.push(left_num);
        second_list.push(right_num);
    }

    ProblemInput {
        first_list,
        second_list,
    }
}

fn find_total_distance_between_lists(problem_input: &ProblemInput) -> i32 {
    let mut result = 0;
    let mut sorted = ProblemInput {
        first_list: problem_input.first_list.clone(),
        second_list: problem_input.second_list.clone(),
    };
    sorted.first_list.sort();
    sorted.second_list.sort();

    if sorted.first_list.len() != sorted.second_list.len() {
        panic!("Lists must be the same length");
    }

    for i in 0..sorted.first_list.len() {
        let first = sorted.first_list[i];
        let second = sorted.second_list[i];
        let diff = (second - first).abs();
        result += diff;
    }

    result
}

fn find_score(problem_input: &ProblemInput) -> i32 {
    let mut score = 0;
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for item in &problem_input.second_list {
        let mut current_count = 0;
        if counts.contains_key(&item) {
            current_count = counts[&item];
        }
        current_count += 1;
        counts.insert(*item, current_count);
    }

    for item in &problem_input.first_list {
        let count = if counts.contains_key(&item) {  counts[&item] } else { 0 };
        score += *item * count;
    }

    score
}

fn main() {
    let file = "input.txt";
    let lines = read_lines(file);
    let problem_input = read_problem_input_from_lines(lines);
    let distance = find_total_distance_between_lists(&problem_input);
    println!("Distance: {:?}", distance);

    let score = find_score(&problem_input);
    println!("Score: {:?}", score);
}
