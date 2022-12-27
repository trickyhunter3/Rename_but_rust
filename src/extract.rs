use walkdir::{DirEntry, WalkDir};
use regex::Regex;
use std::{collections::HashMap, fs};

pub fn iter_over_all_files_check_files(root_path: &str) -> Vec<String>{
    let mut wrong_names: Vec<String> = Vec::new();
    let mut is_there_a_folder = false;
    let walker = WalkDir::new(root_path).into_iter();
    if WalkDir::new(root_path).into_iter().count() > 1 {
        is_there_a_folder = true;
        for entry in walker.filter_entry(|e| !is_hidden(e)){
            let current_entry = entry.unwrap();
            
            if current_entry.file_type().is_dir(){
                println!("Entering Directory: {}", current_entry.file_name().to_str().unwrap());
            }
            else {
                let current_series_name_and_season: Vec<&str>  = get_series_name_and_season(current_entry.path().to_str().unwrap(), current_entry.depth());
                let current_file_name = current_entry.file_name().to_str().unwrap();
                let current_episode_number = check_files_extract_number_from_string(current_file_name);
                let current_season_number = extract_season_number(current_series_name_and_season[1]);
                let current_file_extention = get_file_extention(current_file_name);
                if !filter_extention(current_file_extention){
                    if !is_file_name_valid(current_file_name, current_series_name_and_season[0], current_season_number, current_episode_number, current_file_extention){
                        wrong_names.push(current_entry.path().to_string_lossy().to_string());
                        println!("Wrong name: \"{}\"", current_entry.path().to_str().unwrap());
                    }
                }
            }
        }
    }
    if !is_there_a_folder
    {
        println!("Folder \"{}\" is empty/non existent", root_path);
    }
    return wrong_names;
}

pub fn iter_rename_files(folder_path: &str, is_number_first: bool, is_number_second: bool, is_number_last: bool){
    let walker = WalkDir::new(folder_path).into_iter();
    if WalkDir::new(folder_path).into_iter().count() > 1 {
        let numbers_hashmap = create_hashmap_from_names_in_folder(folder_path);
        for entry in walker.filter_entry(|e| !is_hidden(e)){
            let current_entry = entry.unwrap();

            if !current_entry.file_type().is_dir(){
                let full_file_name = current_entry.path().to_str().unwrap();
                let file_name = current_entry.file_name().to_str().unwrap();
                let series_name_and_season: Vec<&str>  = get_series_name_and_season(current_entry.path().to_str().unwrap(), 3);
                let episode_number = extract_number_from_string_v2(numbers_hashmap.clone(), file_name, is_number_first, is_number_second, is_number_last);
                let season_number = extract_season_number(series_name_and_season[1]);
                let file_extention = get_file_extention(file_name);
                let season_helper = helper_create(season_number, "S".to_string());
                let episode_helper = helper_create(episode_number, "E".to_string());
                let subtitle_helper;
                if file_extention == "ass" {
                    subtitle_helper = ".eng";
                }
                else{
                    subtitle_helper = "";
                }

                if !filter_extention(file_extention){
                    if file_is_safe_to_change(current_entry.depth()){
                        rename_file(full_file_name, file_name, series_name_and_season[0], season_helper, season_number, episode_helper, episode_number, subtitle_helper, file_extention)
                    }
                }
            }
        }
    }
    else{
        println!("Folder is empty/non existent");
    }
}

fn file_is_safe_to_change(file_depth: usize) -> bool{
    if file_depth == 1{
        return true;
    }

    return  false;
}

fn extract_number_from_string_v2(numbers_hashmap: HashMap<String, i32>, file_name: &str, is_number_first: bool, is_number_second: bool, is_number_last: bool) -> i32{
    /*
        how works:
        takes all the files in the folder
        gets all the numbers in the file names
        gets all the numbers in the file names and count them
        use the the most NOT used number
        that probably the number i need
    */
    let re = Regex::new(r"\d+").unwrap();
    //search with regex only numbers inside a &str
    let numbers_in_array_from_file = re.find_iter(file_name).collect::<Vec<_>>();
    //if no numbers found
    if numbers_in_array_from_file.len() == 0{
        return -1;
    }
    else if numbers_in_array_from_file.len() == 1{//if there is only one number found
        return numbers_in_array_from_file[0].as_str().parse().unwrap();
    }
    //choosen by user
    if is_number_first{
        return numbers_in_array_from_file[0].as_str().parse().unwrap();
    }
    if is_number_second{
        return numbers_in_array_from_file[1].as_str().parse().unwrap();
    }
    if is_number_last{
        return numbers_in_array_from_file[numbers_in_array_from_file.len() - 1].as_str().parse().unwrap();
    }
    //find in hash the number with lowest apearence

    //for all the numbers in the file i will check how much it apeared
    //and take the lowest number that apread
    let mut min_num_str = numbers_in_array_from_file[0].as_str();
    let mut min_num_apeared = numbers_hashmap.get(numbers_in_array_from_file[0].as_str()).unwrap();

    for i in 1..numbers_in_array_from_file.len(){
        let current_num_apeared = numbers_hashmap.get(numbers_in_array_from_file[i].as_str()).unwrap();

        if current_num_apeared < min_num_apeared{
            min_num_apeared = current_num_apeared;
            min_num_str = numbers_in_array_from_file[i].as_str();
        }
    }

    return min_num_str.parse().unwrap();
}

fn create_hashmap_from_names_in_folder(folder_path: &str) -> HashMap<String, i32>{
    let mut numbers_hashmap: HashMap<String, i32> = HashMap::new();
    //regex to find numbers
    let re = Regex::new(r"\d+").unwrap();
    let walker = WalkDir::new(folder_path).into_iter();

    //iter over the files inside the folder
    for entry in walker.filter_entry(|e| !is_hidden(e)){
        let current_entry = entry.unwrap();

        //dont enter subfolders
        if current_entry.depth() == 1{
            let file_name = current_entry.file_name().to_string_lossy().to_string();
            let numbers_in_array = re.find_iter(file_name.as_str()).collect::<Vec<_>>();

            for j in 0..numbers_in_array.len(){
                if !numbers_hashmap.contains_key(numbers_in_array[j].as_str()){
                    numbers_hashmap.insert(numbers_in_array[j].as_str().to_string(), 1);
                }
                else {
                    //found this number already so increase the times that i saw it
                    let mut number_from_key_value = *(numbers_hashmap.get(numbers_in_array[j].as_str()).unwrap());
                    number_from_key_value = number_from_key_value + 1;//still learning rust sorry
                    numbers_hashmap.insert(numbers_in_array[j].as_str().to_string(), number_from_key_value);
                }
            }
        }
    }

    return numbers_hashmap;
}


fn rename_file(full_file_name: &str, file_name: &str, series_name: &str, season_helper: String, season_number: i32, episode_helper: String, episode_number: i32, subtitle_helper: &str, file_extention: &str){
    let file_path = get_file_path_no_name(full_file_name);
    let final_name: String = file_path + &series_name.to_string() + &" - ".to_string() + &season_helper + &season_number.to_string() + &episode_helper + &episode_number.to_string() + subtitle_helper + &".".to_string() + file_extention;
    let final_name_no_path: String = series_name.to_string() + &" - ".to_string() + &season_helper + &season_number.to_string() + &episode_helper + &episode_number.to_string() + subtitle_helper + &".".to_string() + file_extention;
    if full_file_name != final_name{
        println!("\"{}\" -> \"{}\"", file_name, final_name_no_path);
        fs::rename(full_file_name, final_name).unwrap();
    }
    else{
        println!("\"{}\" is already a correct name", file_name);
    }
}

fn get_file_path_no_name(full_file_name: &str) -> String{
    let mut final_string: String = "".to_string();
    let slash_seperator = full_file_name.split('\\');
    let slash_vec: Vec<&str> = slash_seperator.collect();

    for i in 0..(slash_vec.len() - 1){
        final_string = final_string +  &slash_vec[i].to_string() + &"\\".to_string();
    }
    return final_string;
}

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

fn helper_create(number: i32, e_or_s: String) -> String{
    if number / 10 == 0{
        return e_or_s + &"0".to_string();
    }

    return e_or_s.to_string();
}

fn is_name_format_correct(file_name: &str, series_name: &str, season_number: i32, episode_number: i32, file_extention: &str, season_helper: String, episode_helper: String, subtitle_helper: &str) -> bool{
    //normal name with 00 example: "Fullmetal Alchemist Brotherhood - S01E01.mp4"
    if file_name == series_name.to_string() + &" - ".to_string() + &season_helper + &season_number.to_string() + &episode_helper + &episode_number.to_string() + subtitle_helper + &".".to_string() + file_extention{
        return true;
    }
    //two episodes in one video: "Fullmetal Alchemist Brotherhood - S01E01-E02.mp4" TODO:will not work on 9-10?
    if file_name == series_name.to_string() + &" - ".to_string() + &season_helper + &season_number.to_string() + &episode_helper + &(episode_number-1).to_string() + &"-".to_string() + &episode_helper + &(episode_number).to_string() + subtitle_helper + &".".to_string() + file_extention{
        return true;
    }
    return false;
}

fn is_file_name_valid(file_name: &str, series_name: &str, season_number: i32, episode_number: i32, file_extention: &str) -> bool{
    if episode_number == -1 || season_number == -1{
        return false;
    }
    let subtitle_helper;
    if file_extention == "ass" {
        subtitle_helper = ".eng";
    }
    else{
        subtitle_helper = "";
    }

    let episode_helper = helper_create(episode_number, "E".to_string());//E or E0
    let season_helper = helper_create(season_number, "S".to_string());//S or S0

    return is_name_format_correct(file_name, series_name, season_number, episode_number, file_extention, season_helper, episode_helper, subtitle_helper);
}




#[cfg(test)]
mod tests {
    use crate::extract::*;
    #[test]
    fn helper_create_test(){
        let season_number_1 = 1;
        let season_number_2 = 13;
        let season_number_3 = 108;

        let episode_number_1 = 1;
        let episode_number_2 = 15;
        let episode_number_3 = 105;

        let episode_helper_1 = helper_create(episode_number_1, "E".to_string());
        let episode_helper_2 = helper_create(episode_number_2, "E".to_string());
        let episode_helper_3 = helper_create(episode_number_3, "E".to_string());

        assert_eq!(episode_helper_1, "E0");
        assert_eq!(episode_helper_2, "E");
        assert_eq!(episode_helper_3, "E");

        let season_helper_1 = helper_create(season_number_1, "S".to_string());
        let season_helper_2 = helper_create(season_number_2, "S".to_string());
        let season_helper_3 = helper_create(season_number_3, "S".to_string());

        assert_eq!(season_helper_1, "S0");
        assert_eq!(season_helper_2, "S");
        assert_eq!(season_helper_3, "S");
    }
    #[test]
    fn get_file_path_no_name_test(){
        let path = "C:\\Code\\hello\\86 - Eighty Six\\Season 1\\86 - Eighty Six - S01E02.txt";
        let path_no_name = get_file_path_no_name(path);
        assert_eq!(path_no_name, "C:\\Code\\hello\\86 - Eighty Six\\Season 1\\");
    }
    #[test]
    fn get_series_name_and_season_check(){
        let path = "C:\\Code\\hello\\86 - Eighty Six\\Season 1\\86 - Eighty Six - S01E02.txt";
        let series_and_name = get_series_name_and_season(path, 3);
        assert_eq!(series_and_name[0], "86 - Eighty Six");
        assert_eq!(series_and_name[1], "Season 1");

        let path2 = "C:\\Code\\hello\\Do It Yourself!!\\Season 3\\Do It Yourself!! - S03E06.txt";
        let series_and_name2 = get_series_name_and_season(path2, 3);
        assert_eq!(series_and_name2[0], "Do It Yourself!!");
        assert_eq!(series_and_name2[1], "Season 3");
    }
    #[test]
    fn check_files_extract_number_from_string_check(){
        let name = "86 - Eighty Six - S01E02.txt";
        let name2 = "Do It Yourself!! - S03E06.txt";

        assert_eq!(check_files_extract_number_from_string(name), 2);
        assert_eq!(check_files_extract_number_from_string(name2), 6);

    }
    #[test]
    fn extract_season_number_check(){
        let season = "Season 1";
        let season2 = "Season 3";
        let season3 = "Season";

        assert_eq!(extract_season_number(season), 1);
        assert_eq!(extract_season_number(season2), 3);
        assert_eq!(extract_season_number(season3), -1);
    }
    #[test]
    fn get_file_extention_check(){
        let name = "86 - Eighty Six - S01E02.txt";
        let name2 = "Do It Yourself!! - S03E06.mkv";
        assert_eq!(get_file_extention(name), "txt");
        assert_eq!(get_file_extention(name2), "mkv");
    }
}

