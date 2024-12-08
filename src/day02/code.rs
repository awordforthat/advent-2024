use std::io::{self};

use crate::utils::helpers::{get_file_as_lines, parse_str_as_ints};

pub fn a() -> io::Result<()> {

    let absolute_path = std::env::current_dir()?.join("src/day02/data.txt");
    let lines:Vec<String> = get_file_as_lines(absolute_path)?;
    let mut total_safe_lines = 0;
    for line in &lines {
        let ints: Vec<i32> = parse_str_as_ints(line);

        let mut last:&i32 = &ints[0];
        let mut last_delta_sign:i32 = if last - &ints[1] > 0 {-1} else {1};
        let mut line_is_safe:bool = true;
        for next in &ints[1..] {

            let delta = last - next;
            let delta_sign: i32 = if last - next > 0 {-1} else {1};
            if (delta.abs() > 3 || delta.abs() == 0) ||(delta_sign != last_delta_sign) {
                line_is_safe = false;
                break
            }
            last_delta_sign = delta_sign;
            last = next;
        }
        total_safe_lines += if line_is_safe {1} else {0};
        
    }
    println!("Total safe lines: {}", total_safe_lines);

    Ok(())
}
