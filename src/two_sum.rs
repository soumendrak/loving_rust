// Two sum problem in Leetcode

use std::collections::HashMap;

fn test() -> Vec<i32> {
    let target = 9;
    let numbers = [2, 7, 11, 15];
    let mut hash_table: HashMap<i32, i32> = HashMap::new();
    for i in 0..numbers.len() {
        println!("Processing number: {}", numbers[i]);
        match hash_table.get(&numbers[i]) {
            Some(&x) => return vec![x, i as i32],
            None => hash_table.insert(target - numbers[i], i as i32),
        };
    }
    return vec![-1, -1];
}

fn main() {
    println!("The result is: {:?}", test())
}
