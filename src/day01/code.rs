use std::io::{self};
use crate::utils;
use std::collections::HashMap;

/**
 * Returns all left ints and right ints in a tuple of two sorted arrays.
 */
fn extract_lists() -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let absolute_path = std::env::current_dir()?.join("src/day01/data.txt");
    let lines:Vec<String> = utils::helpers::get_file_as_lines(absolute_path)?;
    
    let mut first_set: Vec<i32> = Vec::new();
    let mut second_set: Vec<i32> = Vec::new();

    for line in &lines {
        let words: Vec<&str> = line.split_whitespace().collect();

        let left_str:  &&str = words.get(0).unwrap();
        let right_str: &&str = words.get(1).unwrap();

        let left_int: i32 = left_str.parse().unwrap();
        let right_int:i32 = right_str.parse().unwrap();

        first_set.push(left_int);
        second_set.push(right_int);
    }

    first_set.sort();
    second_set.sort();

    return Ok((first_set, second_set));
}

pub fn a() -> io::Result<()> {

    let (first_set, second_set) = extract_lists().unwrap();
   
    let zipped = first_set.iter().zip(second_set.iter());

    let mut distances: Vec<i32> = Vec::new();

    for (first, second) in zipped {
        distances.push((first - second).abs());
    }
    let total:i32 = distances.iter().sum();
    println!("Total: {}", total);

    Ok(())
}

pub fn b() -> io::Result<()> {
    let (first_set, second_set) = extract_lists().unwrap();

    let mut sum :i32 = 0; 
    let mut occurrences:HashMap<i32, i32> = HashMap::new();
    for num in &first_set {
        occurrences.insert(*num, second_set.iter().filter(|&n| *n == *num ).count() as i32);
    }

    for num in &first_set {
        sum = sum + num * occurrences.get(&num).ok_or(0).unwrap();
    }

    println!("Total: {}", sum);

    Ok(())
}