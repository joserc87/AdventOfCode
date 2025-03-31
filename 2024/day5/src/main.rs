use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

struct Order {
    lower_num: i32,
    higher_num: i32,
}

struct Report {
    nums: Vec<i32>,
    nums_by_index: HashMap<i32, i32>,
}

struct ProblemInput {
    orders: Vec<Order>,
    reports: Vec<Report>,
}

impl Report {
    fn get_middle_num(&self) -> i32 {
        self.nums[self.nums.len() / 2]
    }
}

impl ProblemInput {
    fn new() -> ProblemInput {
        let orders = Vec::new();
        let reports = Vec::new();
        ProblemInput { orders, reports }
    }

    fn read_input(&mut self, filename: &str) {
        let file = File::open(filename).expect("File not found");
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line")).collect();
        let mut reading_orders = true;
        for line in lines {
            if reading_orders {
                if line == "" {
                    reading_orders = false;
                    continue
                }
                let order: Vec<i32> = line.split("|").map(|n| n.parse().expect("Not a number")).collect();
                if order.len() != 2 {
                    panic!("Invalid order");
                }
                self.orders.push(Order { lower_num: order[0], higher_num: order[1] });
            } else {
                let numbers: Vec<i32> = line.split(",").map(|n| n.parse().expect("Not a number")).collect();
                let mut numbers_by_index = HashMap::new();
                for (i, num) in numbers.iter().enumerate() {
                    numbers_by_index.insert(*num, i as i32);
                }
                let report = Report { nums: numbers.clone(), nums_by_index: numbers_by_index };
                self.reports.push(report);
            }
        }
    }
}

fn main() {
    let mut problem_input = ProblemInput::new();
    problem_input.read_input("input");
    let mut total = 0;
    for report in &problem_input.reports {
        let mut valid = true;
        for order in &problem_input.orders {
            let ilow = report.nums_by_index.get(&order.lower_num);
            let ihigh = report.nums_by_index.get(&order.higher_num);
            if ilow != None && ihigh != None && ilow > ihigh {
                valid = false;
                break;
            }
        }
        if valid {
            total += report.get_middle_num();
        }
    }
    println!("Total: {}", total);
}
