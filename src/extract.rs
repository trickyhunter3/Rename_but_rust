use walkdir::{DirEntry, WalkDir};
use regex::Regex;
use std::fs;

pub fn iter_over_all_files_check_files(root_path: &str) -> bool{
    let mut is_everything_correct: bool = true;
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
                        is_everything_correct = false;
                        println!("{}", current_file_name);
                    }
                }
            }
        }
    }
    if !is_there_a_folder
    {
        println!("Folder is empty/non existent");
    }
    return is_everything_correct;
}
//full_file_name: &str, series_name: &str, season_helper: String, season_number: i32, episode_helper: String, episode_number: i32

pub fn iter_rename_files(folder_path: &str){
    let walker = WalkDir::new(folder_path).into_iter();
    if WalkDir::new(folder_path).into_iter().count() > 1 {
        for entry in walker.filter_entry(|e| !is_hidden(e)){
            let current_entry = entry.unwrap();

            if !current_entry.file_type().is_dir(){
                let current_series_name_and_season: Vec<&str>  = get_series_name_and_season(current_entry.path().to_str().unwrap(), current_entry.depth());
                let current_file_name = current_entry.file_name().to_str().unwrap();
                //rewrite extract number
                let current_episode_number = check_files_extract_number_from_string(current_file_name);
                let current_season_number = extract_season_number(current_series_name_and_season[1]);
                let current_file_extention = get_file_extention(current_file_name);

                if !filter_extention(current_file_extention){

                }
            }
        }
    }
    else{
        println!("Folder is empty/non existent");
    }
}


fn rename_file(full_file_name: &str, file_name: &str, series_name: &str, season_helper: String, season_number: i32, episode_helper: String, episode_number: i32, subtitle_helper: &str, file_extention: &str){
    let file_path = get_file_path_no_name(full_file_name);
    let mut final_name: String = file_path + &series_name.to_string() + &" - ".to_string() + &season_helper + &season_number.to_string() + &episode_helper + &episode_number.to_string() + subtitle_helper + &".".to_string() + file_extention;
    println!("{} --> {}", file_name, final_name);
}

fn get_file_path_no_name(full_file_name: &str) -> String{
    let mut final_string: String = "".to_string();
    let slash_seperator = full_file_name.split('\\');
    let slash_vec: Vec<&str> = slash_seperator.collect();

    let mut i = 0;
    while i < (slash_vec.len() - 1) {
        final_string = final_string +  &slash_vec[i].to_string() + &"\\".to_string();
        i += 1;
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

fn episode_helper_create(number: i32) -> String{
    if number / 10 == 0{
        return "E0".to_string();
    }

    return "E".to_string();
}

fn season_helper_create(number: i32) -> String{
    if number / 10 == 0{
        return "S0".to_string();
    }

    return "S".to_string();
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

    let episode_helper = episode_helper_create(episode_number);//E or E0
    let season_helper = season_helper_create(season_number);//S or S0

    return is_name_format_correct(file_name, series_name, season_number, episode_number, file_extention, season_helper, episode_helper, subtitle_helper);
}




#[cfg(test)]
mod tests {
    use super::*;
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
