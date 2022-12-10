//use std::path::Path;

mod extract;

fn main() {
    let file_name = "86 - Eighty Six - S01E02.mkv";
    println!("Hello1");
    let a = extract::extract_number_from_string(file_name);
    println!("Hello2");
    println!("{}", a);
}


