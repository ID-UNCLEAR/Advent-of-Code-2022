use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    // Open the file
    let file = File::open("src/bin/02input.txt").expect("Failed to open file");

    // Create a buffered reader to read the file contents
    let reader = BufReader::new(file);

    let mut my_array: Vec<i32> = Vec::new();
    let mut total_array: Vec<i32> = Vec::new();
    let mut sum = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        
        match line.as_str() {
            "A X" => my_array.push(3),
            "A Y" => my_array.push(4),
            "A Z" => my_array.push(8),
            "B X" => my_array.push(1),
            "B Y" => my_array.push(5),
            "B Z" => my_array.push(9),
            "C X" => my_array.push(2),
            "C Y" => my_array.push(6),
            "C Z" => my_array.push(7),
            _ => println!("NEEN!!"),
        }
    }
    for int in &my_array {
        sum += int;
    }
    total_array.push(sum);
    println!("Punten: {:?}", total_array);
    
}
