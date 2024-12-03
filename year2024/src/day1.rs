#[allow(dead_code)]

mod day1{
    pub fn part1(s: String) -> i32 {
        // parse input into 2 lists, one for L, one for R
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        (left, right) = parse_input(s);
        // sort lists
        left.sort();
        right.sort();
        // zip lists on abs of difference
        // sum up difference list
        left.iter()
            .zip(right)
            .map(|(x, y)| (x - y).abs())
            .sum()
    }
    pub fn part2(s: String) -> i32 {
        use std::collections::HashMap;
        // parse input into 2 lists, one for L, one for R
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        (left, right) = parse_input(s);
        // create counter for right list
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for n in right {
            counter.entry(n).and_modify(|e| *e += 1).or_insert(1);
        }
        // iterate through left list
        // add to sum that number times count of that number in counter
        left.iter().map(|x| counter.get(x).unwrap_or(&0) * x).sum()
    }
    fn parse_input(s: String) -> (Vec<i32>, Vec<i32>) {
        // parse input into 2 lists, one for L, one for R
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        s.split_whitespace()
         .map(str::parse::<i32>)
         .enumerate()
         .for_each(|(i, x)| 
            match x {
                x if i % 2 == 0 => left.push(x.unwrap()),
                x => right.push(x.unwrap()),
            }
         );
        (left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1_example_test_p1() {
        let result = day1::part1(String::from(
            "3   4
            4   3
            2   5
            1   3
            3   9
            3   3"
        ));
        assert_eq!(result, 11);
    }
    
    #[test]
    fn d1_example_test_p2() {
        let result = day1::part2(String::from(
            "3   4
            4   3
            2   5
            1   3
            3   9
            3   3"
        ));
        assert_eq!(result, 31);
    } 
}
