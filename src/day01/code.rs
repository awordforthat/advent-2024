use std::fs::File;
use std::io::{self, BufRead};

pub fn a() -> io::Result<()> {

    let relative_path = "src/day01/data.txt";
    let absolute_path = std::env::current_dir()?.join(relative_path);

    let file = match File::open(absolute_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return Err(e);
        }
    };
    
    let reader = io::BufReader::new(file);
    
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    
    let mut first_set: Vec<i32> = Vec::new();
    let mut second_set: Vec<i32> = Vec::new();

    for line in &lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        
        // Access words by index
        let left_str:  &&str = words.get(0).unwrap();
        let right_str: &&str = words.get(1).unwrap();

        let left_int: i32 = left_str.parse().unwrap();
        let right_int:i32 = right_str.parse().unwrap();

        first_set.push(left_int);
        second_set.push(right_int);
    }

    first_set.sort();
    second_set.sort();

    println!("{:?}", first_set);
    println!("{:?}", second_set);

    let zipped = first_set.iter().zip(second_set.iter());

    let mut distances: Vec<i32> = Vec::new();

    for (first, second) in zipped {
        distances.push((first - second).abs());
    }
    let total:i32 = distances.iter().sum();
    println!("Total: {}", total);


    Ok(())
}