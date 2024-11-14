#![allow(unused)]
mod day1 {
    fn main() {
        println!("Part 1: {}", part1(input()));
        println!("Part 2: {}", part2(input()));
    }

    fn part1(s: String) -> i32 {
        s.chars().fold(0, |acc, x| {
            if x == '(' {
                acc + 1
            } else if x == ')' {
                acc - 1
            } else {
                acc
            }
        })
    }
        
    fn part2(s: String) -> i32 {
        let mut floor: i32 = 0;
        for (i, x) in s.chars().enumerate() {
            if x == '(' {
                floor += 1;
            } else if x == ')' {
                floor -= 1;
            }
            if floor < 0 {
                return i as i32 + 1;
            }
        }
        return -1;
    }
        
    fn input() -> String {
        String::from("()") // your input goes here
    }
}

