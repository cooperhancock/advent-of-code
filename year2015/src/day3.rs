#[allow(dead_code)]

mod day3 {
    // start at 0,0; arrows from input describe moving across grid
    // return number of unique grids visited
    pub fn part1(s: String) -> u32 {
        // get counter for input string
        // return number of coords (keys) in counter
    }
    
    fn visit_counter(s: String) -> Vec<(i32, i32)> {
        // create counter for (x, y) coord pairs 
        // (count will probably be important later)
        // start with (0, 0) at 1
        // iterate over string and count coord visits
        // return counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        let result = day3::part1(String::from(">"));
        assert_eq!(result, 2);
    }
    #[test]
    fn test2() {
        let result = day3::part1(String::from("^>v<"));
        assert_eq!(result, 4);
    }
    #[test]
    fn test3() {
        let result = day3::part1(String::from("^v^v^v^v^v"));
        assert_eq!(result, 2);
    }
}

