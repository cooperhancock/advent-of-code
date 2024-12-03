#[allow(dead_code)]

mod day2{
    pub fn part1(s: String) -> i32 {
        s.as_str()
         .split('\n') // split lines
         .map(|r| r // convert records into Vec<i32>
            .split_whitespace() // split record into iter sep at spaces
            .map(|x| x.parse::<i32>().unwrap()) // convert to ints
            .collect()) // collect into vector
         .fold(0, |acc, r| acc + if is_safe(r) {1} else {0}) // count up safe reports
    }
    pub fn part2(s: String) -> i32 {
        0
    }
    pub fn is_safe(report: Vec<i32>) -> bool {
        // check is sorted or reverse is sorted
        // if so, iterate through adj pairs to check if 1 <= diff <= 3
        
        // ascending check
        report
            .windows(2)
            .all(|x| x[1]-x[0] >= 1 && x[1]-x[0] <= 3)
        // descending check
        || report
            .windows(2)
            .all(|x| x[0]-x[1] >= 1 && x[0]-x[1] <= 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2_test_reports_1() {
        assert_eq!(true, day2::is_safe(vec![7, 6, 4, 2, 1]));
    }
    #[test]
    fn d2_test_reports_2() {
        assert_eq!(false, day2::is_safe(vec![1, 2, 7, 8, 9]));
    }
    #[test]
    fn d2_test_reports_3() {
        assert_eq!(false, day2::is_safe(vec![9, 7, 6, 2, 1]));
    }
    #[test]
    fn d2_test_reports_4() {
        assert_eq!(false, day2::is_safe(vec![1, 3, 2, 4, 5]));
    }
    #[test]
    fn d2_test_reports_5() {
        assert_eq!(false, day2::is_safe(vec![8, 6, 4, 4, 1]));
    }
    #[test]
    fn d2_test_reports_6() {
        assert_eq!(true, day2::is_safe(vec![1, 3, 6, 7, 9]));
    }
    
    #[test]
    fn d2_example_p1() {
        let result = day2::part1(String::from(
            "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"
        ));
        assert_eq!(2, result);
    }

}
