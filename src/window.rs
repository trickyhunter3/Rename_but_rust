use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::collections::HashMap;
use std::fs;//read json
use serde_json::Value;

use super::*;//extract library inside

pub fn init_window(){
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rename",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}


struct MyApp {
    json_path_anime: String,
    json_path_anime_not: String,
    user_path: String,
    is_number_first: bool,
    is_number_second: bool,
    is_number_last: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            json_path_anime: get_json("Anime".to_string()).to_owned(),
            json_path_anime_not: get_json("Anime not".to_string()).to_owned(),
            user_path: "".to_owned(),
            is_number_first: false,
            is_number_second: false,
            is_number_last: false,
        }
    }
}

fn get_json(folder: String) -> String{
    let contents = match fs::read_to_string("paths.json"){
        Ok(_string) => _string,
        Err(_err) => {
            println!("File \"paths.json\" doesnt exist/locked");
            return "Json Read Error".to_string();
        },
    };
    let value: Value = serde_json::from_str(&contents).unwrap();
    let value_inside_json = match value[&folder].as_str(){
        Some(_str) => _str,
        None => {
            println!("value \"{}\" was not found inside paths.json", &folder);
            return "Json Value Error".to_string();
        },
    };
    return value_inside_json.to_string();//to remove ""
}

fn get_file_path_no_name(full_name: String) -> String{
    let mut final_string: String = "".to_string();
    let slash_seperator = full_name.split('\\');
    let slash_vec: Vec<&str> = slash_seperator.collect();

    for i in 0..(slash_vec.len() - 1){
        final_string = final_string +  &slash_vec[i].to_string() + &"\\".to_string();
    }
    return final_string;
}

fn extract_the_directories(wrong_names:Vec<Vec<String>>){
    let mut directories: HashMap<String, i32> = HashMap::new();
    for i in wrong_names{
        for j in i{
            let path = get_file_path_no_name(j);
            directories.insert(path, 1);
        }
    }
    for (i,_j) in directories{
        println!("{}", i);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();

        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(20.0, Proportional)),
            (Button, FontId::new(35.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
            ].into();

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_style(style);
            ui.heading("Renamer");
            ui.horizontal(|ui| {
                let name_label = ui.label("Path: ");
                ui.text_edit_multiline(&mut self.user_path)
                    .labelled_by(name_label.id);
            });
            if ui.button("Rename").clicked(){ 
                println!("---------------------------------------------------");
                let slash_seperator = self.user_path.split('\n');
                let slash_vec: Vec<&str> = slash_seperator.collect();
                for i in slash_vec{
                    extract::iter_rename_files(i, self.is_number_first, self.is_number_second, self.is_number_last);
                    println!("---------------------------------------------------");
                }
            }
            ui.add(egui::Checkbox::new(&mut self.is_number_first, "is number first?"));
            ui.add(egui::Checkbox::new(&mut self.is_number_second, "is number second?"));
            ui.add(egui::Checkbox::new(&mut self.is_number_last, "is number last?"));
            if ui.button("Check Files").clicked(){ 
                let mut wrong_names:Vec<Vec<String>> = Vec::new();
                let mut was_there_error = false;
                println!("---------------------------------------------------");
                println!("Anime: {}", &self.json_path_anime);
                println!("Anime not : {}", &self.json_path_anime_not);
                wrong_names.push(extract::iter_over_all_files_check_files(&self.json_path_anime));
                wrong_names.push(extract::iter_over_all_files_check_files(&self.json_path_anime_not));
                println!("---------------------------------------------------");
                println!("Errors:");
                for i in &wrong_names{
                    for j in i{
                        println!("{}", j);
                        was_there_error = true;
                    }
                }
                if !was_there_error{
                    println!("everything is correct!");
                }
                else{
                    println!("---------------------------------------------------");
                    println!("Directories:");
                    extract_the_directories(wrong_names);
                }
            }

        });

    }
}