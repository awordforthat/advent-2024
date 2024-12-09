use std::io::{self};

use crate::utils::helpers::{get_file_as_lines, parse_str_as_ints};

fn is_line_safe(line:&Vec<i32>) -> bool {
    let mut last:&i32 = &line[0];
    let mut last_delta_sign:i32 = if last - &line[1] > 0 {-1} else {1};
    for next in &line[1..] {

        let delta = last - next;
        let delta_sign: i32 = if last - next > 0 {-1} else {1};
        if (delta.abs() > 3 || delta.abs() == 0) ||(delta_sign != last_delta_sign) {
            return false;
        }
        last_delta_sign = delta_sign;
        last = next;
    }
    return true;
    
}

pub fn a() -> io::Result<()> {

    let absolute_path = std::env::current_dir()?.join("src/day02/data.txt");
    let lines:Vec<String> = get_file_as_lines(absolute_path)?;
    let mut total_safe_lines = 0;
    for line in &lines {
        let ints: Vec<i32> = parse_str_as_ints(line);

        let line_is_safe = is_line_safe(&ints);
        total_safe_lines += if line_is_safe {1} else {0};
        
    }
    println!("Total safe lines: {}", total_safe_lines);

    Ok(())
}

pub fn b() -> io::Result<()> {
    let absolute_path = std::env::current_dir()?.join("src/day02/data.txt");
    let lines:Vec<String> = get_file_as_lines(absolute_path)?;
    let mut total_safe_lines = 0;
    let mut failed_lines: Vec<Vec<i32>> = Vec::new();
    for line in &lines {
        let ints: Vec<i32> = parse_str_as_ints(line);
        let line_is_safe = is_line_safe(&ints);
        
        total_safe_lines += if line_is_safe {1} else {0};
        if !line_is_safe {
            failed_lines.push(ints);
        }
        
    }

    for failed_line in failed_lines {
       
        for i in 0..failed_line.len() {
            let mut copy = failed_line.clone();
            copy.remove(i);
            let trunc_is_safe = is_line_safe(&copy);
            if trunc_is_safe {
                total_safe_lines += 1;
                break;
            }
        }
    }
    println!("Total safe lines: {}", total_safe_lines);

    Ok(())
}