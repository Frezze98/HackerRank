fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    // Initialize the variables for the minimum and maximum records
    let mut min_record = scores[0];
    let mut max_record = scores[0];
    
    // Variables to count the number of times records are broken
    let mut min_breaks = 0;
    let mut max_breaks = 0;

    // Iterate through the scores starting from the second game
    for &score in scores.iter().skip(1) {
        if score > max_record {
            max_record = score;
            max_breaks += 1;
        } else if score < min_record {
            min_record = score;
            min_breaks += 1;
        }
    }

    // Return the results in a vector: [max_breaks, min_breaks]
    vec![max_breaks, min_breaks]
}

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breakingRecords(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
