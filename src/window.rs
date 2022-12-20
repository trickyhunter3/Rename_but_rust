use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;

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
    root_path_anime: String,
    root_path_anime_not: String,
    user_path: String,
    json_path: String,
    is_number_first: bool,
    is_number_second: bool,
    is_number_last: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            root_path_anime: "G:\\AN\\Anime".to_owned(),
            root_path_anime_not: "G:\\AN\\Anime not".to_owned(),
            user_path: "".to_owned(),
            json_path: "".to_owned(),
            is_number_first: false,
            is_number_second: false,
            is_number_last: false,
        }
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
                ui.text_edit_singleline(&mut self.user_path)
                    .labelled_by(name_label.id);
            });
            if ui.button("Rename").clicked(){ 
                println!("-----------------");
                extract::iter_rename_files(&self.user_path, self.is_number_first, self.is_number_second, self.is_number_last);
                println!("-----------------");
            }
            ui.add(egui::Checkbox::new(&mut self.is_number_first, "is number first?"));
            ui.add(egui::Checkbox::new(&mut self.is_number_second, "is number second?"));
            ui.add(egui::Checkbox::new(&mut self.is_number_last, "is number last?"));
            if ui.button("Check Files").clicked(){ 
                let mut wrong_names:Vec<Vec<String>> = Vec::new();
                let mut was_there_error = false;
                println!("-----------------");
                wrong_names.push(extract::iter_over_all_files_check_files(&self.root_path_anime));
                wrong_names.push(extract::iter_over_all_files_check_files(&self.root_path_anime_not));
                println!("-----------------");
                println!("Errors:");
                for i in wrong_names{
                    for j in i{
                        println!("{}", j);
                        was_there_error = true;
                    }
                }
                if !was_there_error{
                    println!("everything is correct!");
                }
            }

        });

    }
}