use std::fs::read_dir;

pub(crate) fn list_files(path_str:&str, depth:u32, file_list: &mut Vec<(String,String)>, verbose: bool) -> Result<(), Box<dyn std::error::Error>>{

    if verbose {
        println!("Listing files in : {path_str} , depth: {depth}");
    }



    let dir_it_result = read_dir(path_str);

    if dir_it_result.is_err() {
        println!("Failed to read directory : {}", path_str);
        return Ok(());
    }

    let dir_iterator = dir_it_result.unwrap();


    for entry in dir_iterator {

        if entry.is_err(){
            println!("Error while iterating directory: {}", entry.err().unwrap().kind());
            continue;
        }
        let entry = entry?;
        if verbose{
            println!("{:?}",entry.path());
        }


        if entry.path().is_file() {
            if let Some(filename) = entry.path().file_name() {
                if let Some(filename_str) = filename.to_str() {
                    if verbose{
                        println!("{filename_str}");
                    }

                    let eval_positive = should_file_be_processed(filename_str);
                    if eval_positive.0{
                        file_list.push((entry.path().display().to_string(),eval_positive.1));
                    }
                }
            }

        }
        if entry.path().is_dir() {
            list_files(&entry.path().display().to_string(), depth + 1, file_list, verbose)?;
        }

    }
    Ok(())

}

static KNOWN_EXTENSIONS:&[&str] = &["c","cpp","h","hpp","rs","py","v","vhd"]; // should be in lower case


pub(crate) fn should_file_be_processed(file_name : &str) -> (bool, String){


    if let Some((file_base, file_extension)) = file_name.split_once('.'){

        if KNOWN_EXTENSIONS.contains(&file_extension.to_lowercase().as_str()){
            return (true, file_extension.to_string());
        }

    } else {
        //println!("file extension not splittable {}", file_name);
        return (false,"".to_string());
    }

    (false, "".to_string())

}