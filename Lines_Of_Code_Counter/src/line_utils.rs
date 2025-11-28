//! This module provides utility functions for analyzing source files, including determining
//! whether a line contains source code and counting the number of source lines in a file.
use std::io::{BufRead, BufReader};


/// Determines if a given line of code is considered a source line.
///
/// # Arguments
///
/// * `line` - A string slice that represents a single line of text.
///
/// # Returns
///
/// * `bool` - Returns `true` if the line is a source line, and `false` if it is considered
///   a comment line.
pub(crate) fn determine_if_line_is_source(line:&str) -> bool {

    let trimmed = line.trim();

    if trimmed.starts_with("//") {
        return false;
    }


    return true;
}

pub(crate) fn count_number_of_source_lines(filepath: &str) -> u32 {

    let mut line_counter = 0;
    if let Ok(input_file) = std::fs::File::open(filepath) {
        let mut reader = BufReader::new(input_file);

        let mut line = String::new();

        loop {
            line.clear();
            let line_read_result = reader.read_line(&mut line);

            if let Ok(count) = line_read_result{
                if count == 0{
                    //println!("Reached end of file");
                    break;
                }
                if determine_if_line_is_source(line.as_str()){
                    line_counter += 1;
                }
            }
            else{
                println!("Error while reading source line from file {} : {}", filepath, line_read_result.unwrap_err());
                break;
            }

        }// line counting loop

    }else {
        println!("Error while opening source file: {}", filepath);
    }




    line_counter
}