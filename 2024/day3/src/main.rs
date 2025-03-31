use std::fs::read_to_string;



fn read_file(filename: &str) -> String {
    read_to_string(filename).expect("Failed to read file")
}

fn find_muls(input: &str) -> i32 {
    // find all the matches for "mul\(\d+,\d+\)"
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut total = 0;
    let mut evaluate = true;
    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do()" => {
                evaluate = true;
            }
            "don't()" => {
                evaluate = false;
            }
            _ => {
                if evaluate {
                    let a: i32 = cap[1].parse().unwrap();
                    let b: i32 = cap[2].parse().unwrap();
                    total += a * b;
                }
            }
        }
    }
    total
}

fn main() {
    let filename = "input";
    let contents = read_file(filename);
    let total = find_muls(&contents);
    println!("Total: {}", total);
}
