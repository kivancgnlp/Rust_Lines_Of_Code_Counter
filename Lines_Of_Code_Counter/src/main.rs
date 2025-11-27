use std::collections::HashMap;

mod line_utils;
mod file_traversal_utils;

fn main() {

    let path = match  std::env::args().nth(1) {
        Some(x) => x,
        None => {".".to_string()}
    };

    let verbose = false;
    let mut total_line_count : u32 = 0;
    let mut file_list = Vec::new();
    let mut count_for_types: HashMap<String,u32> = HashMap::new();

    match file_traversal_utils::list_files(&path, 0, &mut file_list, verbose) {
        Ok(_) => {
            println!("File traversal done. {} suitable source files found.", file_list.len());

            for (file_path,extension) in file_list.iter() {

                let lc = count_for_types.get(extension);

                let line_count = line_utils::count_number_of_source_lines(file_path) ;
                total_line_count += line_count;

                match lc {
                    Some(count) => {
                        count_for_types.insert(extension.to_string(), count+line_count);
                    },
                    None => {
                        count_for_types.insert(extension.to_string(),line_count);
                    }
                }

            }

        },
        Err(e) => {
            eprintln!("Some error occurred during traversal {:?}", e);
            return;
        }
    }

    println!("Total lines of code : {}",total_line_count);

    for (k,v) in count_for_types.iter() {
        println!("{:10} = {:10} ( % {} )",k,v, (*v as f32) / total_line_count as f32 * 100.0) ;
    }





}