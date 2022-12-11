use walkdir::{DirEntry, WalkDir};

use regex::Regex;

fn check_files_extract_number_from_string(file_name: &str) -> i32{
    //create a regex instance that finds numbers
    let re = Regex::new(r"\d+").unwrap();
    //search with regex only numbers inside a &str
    let mut numbers_in_array = re.find_iter(file_name).collect::<Vec<_>>();

    if numbers_in_array.len() == 1{
        return numbers_in_array[0].as_str().parse().unwrap();}

    else if numbers_in_array.len() == 0 {
        return -1;}//did not find

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
        return numbers_in_array[0].as_str().parse().unwrap();
    }

    return -1;
}

fn get_file_extention(file_name: &str) -> &str{
    let dot_separator = file_name.split('.');
    let dot_vec: Vec<&str> = dot_separator.collect();
    return dot_vec[dot_vec.len() - 1];
}

fn get_series_name_and_season(file_path: &str, file_depth: usize) -> Vec<&str>{
    if file_depth != 3{
        return ["-1", "-1"].to_vec();//no need to find series
    }

    let slash_seperator = file_path.split('\\');
    let slash_vec: Vec<&str> = slash_seperator.collect();
    return [slash_vec[slash_vec.len() - 3], slash_vec[slash_vec.len() - 2]].to_vec();
}

fn filter_extention(file_extention: &str) -> bool{
    if file_extention == "ini" || file_extention == "nfo" || file_extention == "ico"{
        return true;
    }

    return false;
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

pub fn iter_over_all_files(root_path: &str){
    let walker = WalkDir::new(root_path).into_iter();
    if WalkDir::new(root_path).into_iter().count() > 1 {
        for entry in walker.filter_entry(|e| !is_hidden(e)){
            let current_entry = entry.unwrap();
            
            let _a = current_entry.file_type().is_dir();
            if current_entry.file_type().is_dir(){
                println!("Entering Directory: {}", current_entry.file_name().to_str().unwrap());
            }
            else {
                let current_series_name_and_season: Vec<&str>  = get_series_name_and_season(current_entry.path().to_str().unwrap(), current_entry.depth());
                let current_file_name = current_entry.file_name().to_str().unwrap();
                let current_episode_number = check_files_extract_number_from_string(current_file_name);
                let current_season_number = extract_season_number(current_series_name_and_season[1]);
                let current_file_extention = get_file_extention(current_file_name);
                //println!("{}", current_entry.path().display());
                if !filter_extention(current_file_extention){
                    println!("file_name: {0}, episode_number: {1}, season_number: {2}, file_extention: {3}", current_file_name, current_episode_number, current_season_number, current_file_extention);
                    is_file_name_valid(current_file_name, check_files_extract_number_from_string(current_file_name), current_series_name_and_season[0], current_season_number, current_episode_number, current_file_extention);
                }
            }
        }
    }
}

fn is_file_name_valid(file_name: &str, file_name_number: i32, series_name: &str, season_number: i32, episode_number: i32, file_extention: &str) -> bool{
    if episode_number == -1{
        return false;
    }
    let mut subtitle_helper = "";
    //TODO helper for episodes
    //let mut episode_helper: String = String::new();//E01 or E0001
    //let mut season_helper: String = String::new();//S01 or S0001
    if file_extention == "ass" {
        subtitle_helper = ".eng";
    }

    return true;
}

/* 
fn helper_create(number: i32) -> String{
    let mut helper: String = String::new();
    let mut local_number = number;
    while local_number / 10 != 0{
        helper.push_str("0");
        local_number /= 10;
    }
    return helper;
}
*/