use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use regex::Regex;

fn check_files_extract_number_from_string(file_name: &str) -> i32{
    //create a regex instance that finds numbers
    let re = Regex::new(r"\d+").unwrap();
    //search with regex only numbers inside a &str
    let mut numbers_in_array = re.find_iter(file_name).collect::<Vec<_>>();

    if !(numbers_in_array.len() > 1){
        return numbers_in_array[0].as_str().parse().unwrap();
    }
    //search using the last '-'
    let dash_separator = file_name.split('-');
    let dash_vec: Vec<&str> = dash_separator.collect();
    numbers_in_array = re.find_iter(dash_vec[dash_vec.len() - 1]).collect::<Vec<_>>();
    
    if numbers_in_array.len() > 1{
        return numbers_in_array[1].as_str().parse().unwrap();
    }
    else if numbers_in_array.len() > 0 {
        return numbers_in_array[0].as_str().parse().unwrap();
    }{
        return -1;//did not find
    }
}

fn extract_season_number(directory_name: &str) -> i32{
    //create a regex instance that finds numbers
    let re = Regex::new(r"\d+").unwrap();

    let numbers_in_array = re.find_iter(directory_name).collect::<Vec<_>>();
    if numbers_in_array.len() > 0{
        return numbers_in_array[0].as_str().parse().unwrap();}

    return -1;
}

fn get_file_extention(file_name: &str) -> &str{
    let dot_separator = file_name.split('.');
    let dot_vec: Vec<&str> = dot_separator.collect();
    return dot_vec[dot_vec.len() - 1];
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

pub fn iter_over_all_files(root_path: &str){
    let mut current_directory_name: &str = "";
    let mut current_file_name: &str = "";
    let mut current_entry;

    let walker = WalkDir::new(root_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) 
    {
        current_entry = entry.unwrap();
        
        if current_entry.file_type().is_dir(){
            current_directory_name = current_entry.file_name().to_str().unwrap();
            println!("Entering Directory: {}", current_directory_name);
            check_files_extract_number_from_string(current_directory_name);
        }
        else{
            current_file_name = current_entry.file_name().to_str().unwrap();
            let current_episode_number = check_files_extract_number_from_string(current_file_name);
            let current_season_number = extract_season_number(current_directory_name);
            let current_file_extention = get_file_extention(current_file_name);
            is_file_name_valid(current_file_name, current_directory_name, current_season_number, current_episode_number, current_file_extention);
        }
    }
}


fn is_file_name_valid(file_name: &str, directory_name: &str, season_number: i32, episode_number: i32, file_extention: &str) -> bool{
    if episode_number == -1{
        return false;}
    //TODO check how much zero helpers needed

    return true;
}