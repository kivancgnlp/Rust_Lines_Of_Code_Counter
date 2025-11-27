
mod line_utils;

fn main() {

    let line_count = line_utils::count_number_of_source_lines("Deneme.txt");

    println!("{} lines", line_count);
}