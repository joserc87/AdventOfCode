use std::fmt;
use std::fs::read_to_string;

struct Coordinate {
    x: i32,
    y: i32,
}

struct Part {
    number: i32,
    pos: Coordinate,
}

#[derive(Debug, PartialEq)]
enum ElementType {
    Empty,
    Symbol,
    Part,
}

impl ElementType {
    fn parse_char(c: char) -> Self {
        match c {
            '.' => ElementType::Empty,
            '0'..='9' => ElementType::Part,
            _ => ElementType::Symbol,
        }
    }
}

struct Element {
    content: ElementType,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self.content {
            ElementType::Empty => ".",
            ElementType::Symbol => "*",
            ElementType::Part => "0",
        };
        write!(f, "{}", t)?;
        Ok(())
    }
}

struct SchematicLine {
    elements: Vec<Element>,
}

impl SchematicLine {
    fn parse_line(line: String) -> Result<Self, String> {
        let elements = Vec::new();
        for (i, c) in line.chars().enumerate() {
            let et = ElementType::parse_char(c);
            if et

        }
        let line = SchematicLine { elements };
        return Ok(line);
    }
}

impl fmt::Display for SchematicLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.elements {
            write!(f, "SchematicLine {}", e)?;
        }
        Ok(())
    }
}

struct Schematic {
    elements: Vec<SchematicLine>,
}

impl Schematic {
    fn new(elements: Vec<SchematicLine>) -> Self {
        Self { elements: elements }
    }

    fn from_file(filename: String) -> Result<Self, String> {
        let mut schematic_lines = Vec::new();

        let content = read_to_string(filename);
        for line in content.unwrap().lines() {
            schematic_lines.push(SchematicLine::parse_line(line.to_string())?);
        }
        let schematic = Schematic {
            elements: schematic_lines,
        };

        Ok(schematic)
    }
}

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.elements {
            write!(f, "Schematic {}", e)?;
        }
        Ok(())
    }
}

/*
fn parse_line(line: str) -> Result<SchematicLine, String> {

}

fn read_input(filename: str) -> Result<Schematic, String> {
    let mut schematic_lines = Vec::new();

    let content = read_to_string(filename);
    for line in content.unwrap().lines() {
        schematic_line = parse_line(line);
        schematic_lines.push(schematic_line);
    }
    schematic = Schematic{schematic_lines};

    Ok(schematic)
}
    */

fn main() {
    //let schematic = read_input("filename").unwrap();
    //let schematic = Schematic::new(v);
    let schematic = Schematic::from_file(String::from("input")).unwrap();
    println!("{}", schematic);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        let line = SchematicLine::parse_line(String::from("*..")).unwrap();
        assert_eq!(line.elements.len(), 3);
        assert_eq!(line.elements[0].content, ElementType::Empty);
    }

    #[test]
    fn parse_element_type() {
        assert_eq!(ElementType::parse_char('.'), ElementType::Empty);
        assert_eq!(ElementType::parse_char('*'), ElementType::Symbol);
        assert_eq!(ElementType::parse_char('0'), ElementType::Part);
        assert_eq!(ElementType::parse_char('&'), ElementType::Symbol);
        assert_eq!(ElementType::parse_char('1'), ElementType::Part);
    }

}
