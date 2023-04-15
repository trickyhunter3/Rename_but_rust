use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use serde_json::Value;
use std::collections::HashMap;
use std::fs; //read json

use super::*; //extract library inside

pub fn init_window() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rename",
        options,
        Box::new(|_cc| Box::new(MyApp::default()))
    );
}

struct MyApp {
    json_paths: Vec<String>,
    user_path: String,
    name_enc: String,
    is_number_first: bool,
    is_number_second: bool,
    is_number_last: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            json_paths: get_json(),
            user_path: "".to_owned(),
            name_enc: "[denisplay] , (1080p) [AV1]".to_owned(),
            is_number_first: false,
            is_number_second: false,
            is_number_last: false,
        }
    }
}

fn get_json() -> Vec<String> {
    let mut final_vec: Vec<String> = Vec::new();
    let json_file_string = match fs::read_to_string("paths.json") {
        Ok(string) => string,
        Err(_err) => {
            println!("Cannot Open json file \"paths.json\"");
            return final_vec;
        }
    };
    let json_values: Value = match serde_json::from_str(&json_file_string) {
        Ok(value) => value,
        Err(_err) => {
            println!("\"paths.json\" fromatted incorectly");
            return final_vec;
        }
    };
    let json_names_value = match json_values["Value_Names"].as_str() {
        Some(str) => str,
        None => {
            println!("\"Value_Names\" was not found inside \"paths.json\"");
            return final_vec;
        }
    };
    let comma_seperator: Vec<&str> = json_names_value.split(',').collect();
    for i in comma_seperator {
        let current_value = json_values[i].as_str();
        if current_value.is_some() {
            final_vec.push(current_value.unwrap().to_string());
        } else {
            println!("\"{}\" was not found inside \"paths.json\"", i);
        }
    }
    return final_vec;
}

fn get_file_path_no_name(full_name: String) -> String {
    let mut final_string: String = "".to_string();
    let slash_seperator: Vec<&str> = full_name.split('\\').collect();

    for i in 0..slash_seperator.len() - 1 {
        final_string = final_string + &slash_seperator[i].to_string() + &"\\".to_string();
    }
    return final_string;
}

fn extract_the_directories(wrong_names: Vec<Vec<String>>) {
    let mut directories: HashMap<String, i32> = HashMap::new();
    for i in wrong_names {
        for j in i {
            let path = get_file_path_no_name(j);
            directories.insert(path, 1);
        }
    }
    for (i, _j) in directories {
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
            (Button, FontId::new(20.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ].into();

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_style(style);
            ui.heading("Renamer");
            ui.horizontal(|ui| {
                let name_label = ui.label("Path: ");
                ui.text_edit_multiline(&mut self.user_path).labelled_by(name_label.id);
            });

            if ui.button("Rename").clicked() {
                println!("---------------------------------------------------");
                let slash_seperator = self.user_path.split('\n');
                let slash_vec: Vec<&str> = slash_seperator.collect();
                for i in slash_vec {
                    extract::iter_rename_files(
                        i,
                        self.is_number_first,
                        self.is_number_second,
                        self.is_number_last
                    );
                    println!("---------------------------------------------------");
                }
            }

            ui.add(egui::Checkbox::new(&mut self.is_number_first, "is number first?"));
            ui.add(egui::Checkbox::new(&mut self.is_number_second, "is number second?"));
            ui.add(egui::Checkbox::new(&mut self.is_number_last, "is number last?"));

            ui.add_space(15.0);

            if ui.button("Rename To Numbers").clicked() {
                let slash_seperator = self.user_path.split('\n');
                let slash_vec: Vec<&str> = slash_seperator.collect();
                for i in slash_vec {
                    extract::iter_rename_into_number(
                        i,
                        self.is_number_first,
                        self.is_number_second,
                        self.is_number_last
                    );
                    println!("---------------------------------------------------");
                }
            }

            if ui.button("Subtract numbers").clicked() {
                println!()
            }

            ui.add_space(30.0);

            ui.horizontal(|ui| {
                if ui.button("Check Files").clicked() {
                    btn_check_files(&self.json_paths)
                }
                if ui.button("Show Folder Files").clicked() {
                    btn_show_folder_files(&self.user_path);
                }
            });

            ui.add_space(30.0);

            ui.horizontal(|ui| {
                let name_label = ui.label("Encoder_name, Res");
                ui.text_edit_singleline(&mut self.name_enc).labelled_by(name_label.id);
                if ui.button("Rename Encodes").clicked() {
                    btn_rename_encoding(&self.user_path, &self.name_enc, self.is_number_first, self.is_number_second, self.is_number_last);
                }
            });
        });
    }
}

fn btn_check_files(json_paths: &Vec<String>) {
    if json_paths.len() == 0 {
        println!("There are no folders to check\ncheck if the json file formated correctly");
        return;
    }
    let mut wrong_names: Vec<Vec<String>> = Vec::new();
    let mut was_there_error = false;
    for i in json_paths {
        println!("---------------------------------------------------");
        println!("{}", i);
        wrong_names.push(extract::iter_over_all_files_check_files(i));
    }
    println!("---------------------------------------------------");
    println!("Errors:");
    for i in &wrong_names {
        for j in i {
            println!("{}", j);
            was_there_error = true;
        }
    }
    if !was_there_error {
        println!("everything is correct!");
    } else {
        println!("---------------------------------------------------");
        println!("Directories:");
        extract_the_directories(wrong_names.clone());
    }
}

fn btn_show_folder_files(folder_path: &str){
    extract::iter_print_all_files(folder_path);
}

fn btn_rename_encoding(folder_path: &str, name_enc: &str, is_number_first: bool, is_number_second: bool, is_number_last: bool){
    let name_enc_vec: Vec<&str> = name_enc.split(",").collect();
    extract::iter_rename_encodes(folder_path, &name_enc_vec, is_number_first, is_number_second, is_number_last);
}
