use std::fs::read_to_string;
struct ProblemInput {
    reports: Vec<Vec<i32>>
}

fn read_input_file(filename: String) -> ProblemInput {
    let mut reports = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let mut report: Vec<i32> = Vec::new();
        for part in parts {
            let part_num = part.parse::<i32>().unwrap();
            report.push(part_num);
        }
        reports.push(report);
    }
    ProblemInput { reports }
}

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn get_first_and_second(dampening: usize) -> (usize, usize) {
    if dampening == 0 {
        (1, 2)
    } else if dampening == 1 {
        (0, 2)
    } else {
        (0, 1)
    }
}

fn is_report_safe(report: &Vec<i32>, dampening: usize, verbose: bool) -> bool {
    let (first, second) = get_first_and_second(dampening);
    let direction = if report[first] < report[second] { Direction::Increasing } else { Direction::Decreasing };
    let mut last_value = report[first];

    for i in second..report.len() {
        if i == dampening {
            continue;
        }
        let current_value = report[i];
        let diff = match direction {
            Direction::Increasing => current_value - last_value,
            Direction::Decreasing => last_value - current_value,
        };
        if diff <= 0 || diff > 3 {
            // This is the retry -> return false
            if dampening != usize::MAX {
                return false;
            }
            // Retry with dampening
            if i < 2 {
                return is_report_safe(report, i - 1, verbose) || is_report_safe(report, i, verbose);
            } else {
                return is_report_safe(report, i - 1, verbose) || is_report_safe(report, i, verbose) || is_report_safe(report, i - 2, verbose);
            }
        }
        last_value = current_value;
    }
    if verbose && dampening != usize::MAX{
        print!("(*{:?})", dampening);
    }
    true
}

fn count_safe_reports(problem_input: &ProblemInput, verbose: bool) -> i32 {
    let mut count = 0;
    for report in &problem_input.reports {
        if verbose {
            for i in 0..report.len() {
                print!("{} ", report[i]);
            }
        }
        let safe = is_report_safe(&report, usize::MAX, verbose);
        if safe {
            count += 1;
        }
        if verbose {
            println!("{}", if safe { "Safe" } else { "Unsafe" });
        }
    }
    count
}

fn main() {
    let filename = "input";
    let problem_input = read_input_file(filename.to_string());
    let count = count_safe_reports(&problem_input, true);
    println!("Count of safe reports: {}", count);
}
