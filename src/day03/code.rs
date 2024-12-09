use std::io::{self};
use regex::Regex;

use crate::utils::helpers::{get_file_as_lines};

const DATA_SOURCE: &str = "./src/day03/data.txt";

fn load_file() -> Vec<String> {
    return get_file_as_lines(std::env::current_dir().expect("not a string").join(DATA_SOURCE)).unwrap();
}

pub fn a() -> io::Result<()> {
    let instructions: Vec<String> = load_file();
    println!("{:?}", instructions);
    let mut total:u128 = 0;
    for line in instructions {
        let mult_indices: Vec<usize> = line.match_indices("mul").map(|(index, _)| index).collect();
        let expression_re: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap(); // "mul(1,3)"
        let digits_re: Regex = Regex::new(r"(\d{1,3})").unwrap(); // 123
        for instance in mult_indices {
            let max_index: usize = std::cmp::min(instance + 12, line.len());
            let substr: &str = &line[instance..max_index];
            let expression_match: Option<regex::Match<'_>> = expression_re.find(substr);
            if expression_match.is_some() {
                let mult_exp: &str = &substr[expression_match.unwrap().range()];
                // println!("{:?}", mult_exp);
                let captures:Vec<regex::Captures<>> = digits_re.captures_iter(mult_exp).collect();
                if captures.len() >= 2 {
                    let first_capture_result:(&str, [&str; 1]) = captures.get(0).unwrap().extract();
                    let first_capture:u128 = first_capture_result.0.parse().unwrap();
                    let second_capture_result:(&str, [&str; 1]) = captures.get(1).unwrap().extract();
                    let second_capture:u128 = second_capture_result.0.parse().unwrap();
                    // println!("{} * {} = {}", first_capture, second_capture, first_capture * second_capture)
                    total += first_capture * second_capture;
                }
    
               
            }
        }
    }
    
   
    println!("Total: {}", total);
    // 31862452 too low

    Ok(())
}

pub fn b() -> io::Result<()> {
   
    Ok(())
}