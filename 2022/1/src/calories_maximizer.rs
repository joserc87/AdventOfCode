mod elfbag;
pub use elfbag::ElfBag;

pub struct CaloriesMaximizer {
    max: ElfBag,
    current: ElfBag,
    next: i32,
}

impl CaloriesMaximizer {
    pub fn new() -> CaloriesMaximizer {
        CaloriesMaximizer {max: ElfBag::new(-1), current: ElfBag::new(0), next: 0}
    }

    pub fn add_calories(&mut self, calories: i32) {
        self.current.add_calories(calories);
        if self.current > self.max {
            self.max = self.current;
        }
    }

    pub fn go_to_next(&mut self) {
        self.next += 1;
        self.current = ElfBag::new(self.next);
    }

    pub fn get_max(&self) -> ElfBag {
        self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calories_maximizer_new() {
        let maximizer: CaloriesMaximizer = CaloriesMaximizer::new();
        assert_eq!(maximizer.get_max().get_calories(), 0);
        assert_eq!(maximizer.get_max().get_index(), -1);
    }

    #[test]
    fn test_calories_maximizer_append() {
        let mut maximizer: CaloriesMaximizer = CaloriesMaximizer::new();
        maximizer.add_calories(1);
        maximizer.add_calories(2);
        maximizer.add_calories(3);
        assert_eq!(maximizer.get_max().get_calories(), 6);
        assert_eq!(maximizer.get_max().get_index(), 0);
    }

    #[test]
    fn test_calories_maximizer_incremental() {
        let mut maximizer: CaloriesMaximizer = CaloriesMaximizer::new();
        maximizer.add_calories(1);
        maximizer.go_to_next();
        maximizer.add_calories(1);
        maximizer.add_calories(2);
        maximizer.add_calories(3);
        maximizer.go_to_next();
        maximizer.add_calories(1);
        maximizer.add_calories(2);
        maximizer.add_calories(3);
        maximizer.add_calories(4);
        assert_eq!(maximizer.get_max().get_calories(), 10);
        assert_eq!(maximizer.get_max().get_index(), 2);
    }

    #[test]
    fn test_calories_maximizer_decremental() {
        let mut maximizer: CaloriesMaximizer = CaloriesMaximizer::new();
        maximizer.add_calories(1);
        maximizer.add_calories(2);
        maximizer.add_calories(3);
        maximizer.add_calories(4);
        maximizer.go_to_next();
        maximizer.add_calories(1);
        maximizer.add_calories(2);
        maximizer.add_calories(3);
        maximizer.go_to_next();
        maximizer.add_calories(3);
        assert_eq!(maximizer.get_max().get_calories(), 10);
        assert_eq!(maximizer.get_max().get_index(), 0);
        assert_eq!(maximizer.current.get_calories(), 3);
        assert_eq!(maximizer.current.get_index(), 2);
    }

}
