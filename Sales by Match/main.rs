use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut sock_counts = HashMap::new();

    for &sock in ar {
        *sock_counts.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;

    for &count in sock_counts.values() {
        pairs += count / 2; 
    }

    pairs 
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
