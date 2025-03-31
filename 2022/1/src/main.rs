mod calories_maximizer;
use std::io::BufRead;

fn find_elf_with_max_calories<R>(mut reader: R) -> Result<calories_maximizer::ElfBag, String>
where
    R: BufRead
{
    let mut maximizer: calories_maximizer::CaloriesMaximizer = calories_maximizer::CaloriesMaximizer::new();
    loop {
        let mut input_text: String = String::new();
        let end: Result<usize, std::io::Error> = reader.read_line(&mut input_text);
        if !end.is_ok() || input_text.len() <= 0 {
            break;
        }
        let trimmed = input_text.trim();

        if trimmed.len() > 0 {
            let parse_result: Result<i32, core::num::ParseIntError> = trimmed.parse::<i32>();
            let calories = match parse_result {
                Ok(i) => i,
                Err(err) => return Err(err.to_string())
            };
            maximizer.add_calories(calories);
        } else {
            maximizer.go_to_next();
        }
    }
    Ok(maximizer.get_max())
}

fn main() {
    use std::io::stdin;
    let result = find_elf_with_max_calories(stdin().lock());
    match result {
        Ok(elf) => println!("Max Elf {} with calories: {}", elf.get_index(), elf.get_calories()),
        Err(e) => println!("ERROR! {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptance() {
        let test_data = std::fs::read_to_string("input.txt").unwrap();
        let result = find_elf_with_max_calories(test_data.as_bytes());
        println!("Result {}", result.as_ref().unwrap().get_calories());
        let mut expected_calories = calories_maximizer::ElfBag::new(237);
        expected_calories.add_calories(68442);
        assert_eq!(result, Ok(expected_calories));
    }
}
