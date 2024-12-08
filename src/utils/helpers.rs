use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;


pub fn get_file_as_lines(abs_path:PathBuf) -> Result<Vec<String>, std::io::Error> {
    let file = match File::open(abs_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return Err(e);
        }
    };
    
    let reader = io::BufReader::new(file);
    
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    return Ok(lines);
}

pub fn parse_str_as_ints(line:&String) -> Vec<i32>{
   return line.split_whitespace().map(| f|  f.parse::<i32>().unwrap()).collect();
}