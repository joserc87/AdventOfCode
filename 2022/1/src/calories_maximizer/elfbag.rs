use std::cmp::Ordering;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct ElfBag {
    num_calories: i32,
    index: i32,
}

impl PartialOrd for ElfBag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.num_calories.cmp(&other.num_calories))
    }
}

impl ElfBag {
    pub fn new(index: i32) -> ElfBag {
        ElfBag {num_calories: 0, index: index}
    }

    pub fn add_calories(&mut self, calories: i32) {
        self.num_calories += calories;
    }

    pub fn get_calories(&self) -> i32 {
        self.num_calories
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_items() {
        let mut bag: ElfBag = ElfBag::new(100);
        bag.add_calories(12);
        bag.add_calories(3);
        assert_eq!(bag.num_calories, 15);
        assert_eq!(bag.index, 100);
    }

    #[test]
    fn test_elf_items_comparison() {
        let mut bag1: ElfBag = ElfBag::new(100);
        let mut bag2: ElfBag = ElfBag::new(100);
        bag1.add_calories(12);
        bag2.add_calories(3);
        assert!(bag1 > bag2);
        assert!(bag1 >= bag2);
        assert!(bag2 >= bag2);
        assert!(bag2 < bag1);
        assert!(bag2 <= bag1);
        assert!(bag1 <= bag1);
    }
}
