use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file
    let file = File::open("src/bin/01input.txt").expect("Failed to open file");

    // Create a buffered reader to read the file contents
    let reader = BufReader::new(file);
    
    let mut my_array: Vec<i32> = Vec::new();
    let mut my_total: Vec<i32> = Vec::new();
    let mut sum = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if line == "" || numbers.is_empty() {
            for int in &my_array {
                sum += int;
            }
            println!("");
            println!("{}. Totaal calorieën: {:?}", index + 1, sum);
            println!("");
            my_total.push(sum);
            my_array.clear();
            sum = 0;
        } else {
            for number in numbers {
                my_array.push(number.parse().expect("Failed to parse number"));
            }
            println!("{}. {}", index + 1, line);
        }
    }
    my_total.sort_by(|a, b| b.cmp(a));
    println!("Totaal van totaal calorieën: {:?}", my_total);
    // println!("übertotaal: {};", my_total.iter().max().unwrap());
}