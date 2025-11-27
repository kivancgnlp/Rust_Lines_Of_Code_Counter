
use std::io::{BufRead, BufReader};

pub(crate) fn determine_if_line_is_source(line:&str) -> bool {

    if line.contains("//"){
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
                    println!("Reached end of file");
                    break;
                }
                if determine_if_line_is_source(line.as_str()){
                    line_counter += 1;
                }
            }
            else{
                println!("Error while reading source line: {}", line_read_result.unwrap_err());
                break;
            }

        }




    }else {
        println!("Error while opening source file: {}", filepath);
    }




    line_counter
}