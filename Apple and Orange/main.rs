use std::io::{self, BufRead};

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    // Count apples that fall on the house
    let apples_on_house = apples.iter().filter(|&&d| {
        let landing_position = a + d;
        landing_position >= s && landing_position <= t
    }).count();

    // Count oranges that fall on the house
    let oranges_on_house = oranges.iter().filter(|&&d| {
        let landing_position = b + d;
        landing_position >= s && landing_position <= t
    }).count();

    // Output the results
    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();
    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = third_multiple_input[0].trim().parse::<i32>().unwrap();
    let n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}
