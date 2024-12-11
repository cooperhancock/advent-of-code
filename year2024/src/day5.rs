#[allow(dead_code)]
use std::collections::HashMap;

fn day1(s: String) -> i32 {
    let mut (rules_table, split_index) = create_rules(s); 
    // 
}

fn create_rules(s: String) -> (HashMap<i32, Vec<i32>>, usize) {
    let mut rules_table: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut split_index = 0; // track split between rules and num lists
    
    for (i, line) in s.lines().enumerate() {
        if line.trim() == "" { 
            split_index = i;
            break; 
        }
        // parse rule x|y from line
        let rule: Vec<i32> = line
            .split('|')
            .map(|x| x.trim().parse::<i32>().unwrap_or(0))
            .collect();
        // for each x|y, push x to table at key y
        // rules_table[y] contains all xs that must come before y
        rules_table
            .entry(rule[1])
            .and_modify(|x| { x.push(rule[0]) })
            .or_insert(vec![rule[0]]);
    }
    
    (rules_table, split_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5_rules_test() {
        let (table, _) = create_rules(String::from(
            "1|2
            2|3
            
            "
        ));
        assert_eq!(table.get(&2), Some(vec![1]).as_ref());
        assert_eq!(table.get(&3), Some(vec![2]).as_ref());
        println!("{:?}", table);
    }
    
}
