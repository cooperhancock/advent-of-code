#[allow(dead_code)]

mod day3 {
    use std::collections::HashMap;
    
    // start at 0,0; arrows from input describe moving across grid
    // return number of unique grids visited
    
    pub fn part1(s: String) -> usize {
        // get counter for input string
        // return number of coords (keys) in counter
        let counter = visit_counter(s);
        counter.len()
    }
    
    pub fn visit_counter(s: String) -> HashMap<(i32, i32), u32> {
        // create counter for (x, y) coord pairs 
        // (count will probably be important later)
        let mut counter: HashMap<(i32, i32), u32> = HashMap::new();
        // start with (0, 0) at 1
        let mut curr_loc = (0, 0);
        counter.insert(curr_loc, 1);
        // iterate over string and count coord visits
        for c in s.chars() {
            // parse input and update current location
            match c {
                '^' => curr_loc.1 += 1,
                '>' => curr_loc.0 += 1,
                'v' => curr_loc.1 -= 1,
                '<' => curr_loc.0 -= 1,
                _ => panic!("error parsing input string '{:?}'", c),
            };
            // update counter
            counter.entry(curr_loc).and_modify(|count| *count += 1).or_insert(1);
        }
        // return counter
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn counter_test() {
        let result = day3::visit_counter(String::from("^>v<"));
        let counter = HashMap::from([
            ((0, 0), 2),
            ((1, 0), 1),
            ((1, 1), 1),
            ((0, 1), 1),
        ]);
        println!("{:?}", result);
        println!("{:?}", counter);
        assert_eq!(result, counter);
    }
    
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

