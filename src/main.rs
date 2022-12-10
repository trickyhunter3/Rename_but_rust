//use std::path::Path;

mod extract;

fn main() {
    let file_name = "Gintama - S01E03.mkv";
    println!("Hello1");
    let a = extract::extract_number_from_string(file_name);
    println!("Hello2");
    println!("{}", a);
}


