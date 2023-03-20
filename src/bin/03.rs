use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/03input.txt").expect("Failed to read input file");
    
    let item_counts = count_items(&input);
    let badge_types = find_badge_types(&input, &item_counts);
    let sum_of_priorities: usize = badge_types.iter().map(|&item| item_counts[&item]).sum();
    
    println!("{}", sum_of_priorities); // prints 94
}

// Count the number of occurrences of each item in the input
fn count_items(input: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    
    for c in input.chars() {
        if c.is_alphabetic() {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    
    counts
}

// Find the item type that corresponds to the badges of each three-Elf group
fn find_badge_types(input: &str, item_counts: &HashMap<char, usize>) -> Vec<char> {
    let mut badge_types = Vec::new();
    let mut lines = input.lines().filter(|line| !line.is_empty());

    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next()) {
        let mut common_items = item_counts.keys().cloned().collect::<Vec<char>>();
        for item in &common_items {
            if !first.contains(*item) || !second.contains(*item) || !third.contains(*item) {
                // This item is not common to all three Elves, so remove it from the list
                common_items.retain(|&x| x != *item);
            }
        }
        if let Some(badge_type) = common_items.first() {
            badge_types.push(*badge_type);
        }
    }

    badge_types
}
