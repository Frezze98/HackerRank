use std::io::{self, BufRead};

fn miniMaxSum(arr: &[i64]) {
    
    let total_sum: i64 = arr.iter().sum();

    
    let min_value = arr.iter().min().unwrap();
    let max_value = arr.iter().max().unwrap();

    
    let min_sum = total_sum - max_value;

    
    let max_sum = total_sum - min_value;

    
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    
    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();


    miniMaxSum(&arr);
}
