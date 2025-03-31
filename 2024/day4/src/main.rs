use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
struct Soup {
    grid: Vec<Vec<char>>,
    direction: Option<Direction>,
    current_row: usize,
}

impl Soup {
    fn get_next_row(&mut self) -> Option<String> {
        let current_row = self.get_current_row();
        self.current_row += 1;
        if !self.is_current_row_valid() {
            self.current_row = 0;
            self.direction = self.get_next_direction();
        }
        current_row
    }

    fn get_next_direction(&self) -> Option<Direction> {
        match self.direction {
            Some(Direction::Right) => Some(Direction::Left),
            Some(Direction::Left) => Some(Direction::Down),
            Some(Direction::Down) => Some(Direction::Up),
            Some(Direction::Up) => Some(Direction::DownRight),
            Some(Direction::DownRight) => Some(Direction::DownLeft),
            Some(Direction::DownLeft) => Some(Direction::UpRight),
            Some(Direction::UpRight) => Some(Direction::UpLeft),
            Some(Direction::UpLeft) => None,
            None => None,
        }
    }

    fn get_current_row(&self) -> Option<String> {
        let mut row = String::new();
        let (mut i, mut j, di, dj);
        match self.direction {
            Some(Direction::Up) => {
                i = self.grid.len() - 1;
                j = self.current_row;
                di = -1;
                dj = 0;
            }
            Some(Direction::Down) => {
                i = 0;
                j = self.current_row;
                di = 1;
                dj = 0;
            }
            Some(Direction::Left) => {
                i = self.current_row;
                j = self.grid.len() - 1;
                di = 0;
                dj = -1;
            }
            Some(Direction::Right) => {
                i = self.current_row;
                j = 0;
                di = 0;
                dj = 1;
            }
            Some(Direction::UpLeft) => {
                di = -1;
                dj = -1;
                if self.current_row <= self.grid.len() - 1 {
                    i = self.current_row;
                    j = self.grid.len() - 1;
                } else {
                    i = self.grid.len() - 1;
                    j = self.current_row - (self.grid.len() - 1) - 1;
                }
            }
            Some(Direction::UpRight) => {
                di = -1;
                dj = 1;
                if self.current_row <= self.grid.len() - 1 {
                    i = self.current_row;
                    j = 0;
                } else {
                    i = self.grid.len() - 1;
                    j = self.current_row - (self.grid.len() - 1);
                }
            }
            Some(Direction::DownLeft) => {
                di = 1;
                dj = -1;
                if self.current_row <= self.grid.len() - 1 {
                    i = 0;
                    j = self.current_row;
                } else {
                    i = self.current_row - (self.grid.len() - 1);
                    j = self.grid.len() - 1;
                }
            }
            Some(Direction::DownRight) => {
                di = 1;
                dj = 1;
                if self.current_row <= self.grid.len() - 1 {
                    i = 0;
                    j = self.current_row;
                } else {
                    i = self.current_row - (self.grid.len() - 1);
                    j = 0;
                }
            }
            None => return None,
        }

        while i < self.grid.len() && j < self.grid[i].len() {
            row.push(self.grid[i][j]);
            i = (i as i32 + di) as usize;
            j = (j as i32 + dj) as usize;
        }


        Some(row)
    }

    fn is_current_row_valid(&self) -> bool {
        let horizontal_size = self.grid.len();
        let vertical_size = self.grid[0].len();
        let diagonal_size = horizontal_size + vertical_size - 1;

        match self.direction {
            Some(Direction::Up) | Some(Direction::Down) => self.current_row < horizontal_size,
            Some(Direction::Left) | Some(Direction::Right) => self.current_row < vertical_size,
            Some(Direction::UpLeft) | Some(Direction::UpRight) | Some(Direction::DownLeft) | Some(Direction::DownRight) => self.current_row < diagonal_size,
            None => false,
        }
    }
}

fn read_lines(filename: &str) -> Soup {
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line")).collect();

    let mut grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }

    Soup {
        grid,
        direction: Some(Direction::Right),
        current_row: 0,
    }
}

fn count_occurrences(text: &str, word: &str) -> i32 {
    let mut count = 0;
    let mut start = 0;
    while let Some(pos) = text[start..].find(word) {
        count += 1;
        start += pos + 1;
    }
    count
}

#[allow(dead_code)]
fn print_soup_lines(soup: &mut Soup) {
    let mut row;
    let mut direction = soup.direction.clone();
    loop {
        row = soup.get_next_row();
        if row.is_none() {
            break;
        }
        println!("{}", row.unwrap());
        if soup.direction != direction {
            println!("\n{:?}", &soup.direction);
            direction = soup.direction.clone();
        }
    }
}

fn first_part() {
    let filename = "input";
    let mut soup = read_lines(filename);
    println!("Puzzle read");
    let mut occurrences = 0;
    loop {
        let row = soup.get_next_row();
        if row.is_none() {
            break;
        }
        let row = row.unwrap();
        occurrences += count_occurrences(&row, "XMAS");
    }
    println!("Occurrences: {}", occurrences);
}

fn find_xmas_occurrences(soup: &Soup) -> i32 {
    let mut occurrences = 0;
    for i in 1..(soup.grid.len() - 1) {
        for j in 1..(soup.grid[i].len() - 1) {
            if soup.grid[i][j] != 'A' {
                continue;
            }
            // corners
            let (ul, ur, dl, dr) = (soup.grid[i - 1][j - 1], soup.grid[i - 1][j + 1], soup.grid[i + 1][j - 1], soup.grid[i + 1][j + 1]);
            let mut valid_letters = true;
            for x in [ul, ur, dl, dr].iter() {
                if *x != 'M' && *x != 'S' {
                    valid_letters = false;
                    break;
                }
            }
            if !valid_letters {
                continue;
            }
            if ul == dr || ur == dl {
                continue
            }
            occurrences += 1;
        }
    }
    occurrences
}

fn second_part() {
    let filename = "input";
    let soup = read_lines(filename);
    let occurrences = find_xmas_occurrences(&soup);
    println!("Occurrences: {}", occurrences);
}

fn main() {
    first_part();
    second_part();
}
