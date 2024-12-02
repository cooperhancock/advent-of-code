#[allow(dead_code)]

mod day1{
    pub fn part1(s: String) -> i32 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1_example_test() {
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
}
