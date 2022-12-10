use regex::Regex;


//noraml regex
/*
let re = Regex::new(r"\d+").unwrap();
    
let answer = re.find_iter(file_name);
for i in answer{
    println!("{}", i.as_str());
}
*/

pub fn extract_number_from_string(file_name: &str) -> i32{
    let mut numbers_in_array: Vec<&str> = Vec::new();
    for cap in Regex::new(r"\d+").unwrap().find_iter(file_name) {
        numbers_in_array.push(cap.as_str());
    }

    if !(numbers_in_array.len() > 1){
        return numbers_in_array[0].parse().unwrap();
    }

    numbers_in_array = Vec::new();
    let dash_separator = file_name.split('-');
    let dash_vec: Vec<&str> = dash_separator.collect();
    for cap in Regex::new(r"\d+").unwrap().find_iter(dash_vec[dash_vec.len() - 1]) {
        numbers_in_array.push(cap.as_str());
    }

    if numbers_in_array.len() > 1{
        return numbers_in_array[1].parse().unwrap();
    }
    else{
        return numbers_in_array[0].parse().unwrap();
    }
}