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
            is_number_first: false,
            is_number_second: false,
            is_number_last: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Path: ");
                ui.text_edit_singleline(&mut self.user_path)
                    .labelled_by(name_label.id);
            });
            if ui.button("Rename").clicked(){ 
                //"G:\\AN\\Anime\\86 - Eighty Six\\Season 1\\86 - Eighty Six - S01E01.mkv"
                extract::iter_rename_files(&self.user_path);
            }
            ui.add(egui::Checkbox::new(&mut self.is_number_first, "is number first?"));
            ui.add(egui::Checkbox::new(&mut self.is_number_second, "is number second?"));
            ui.add(egui::Checkbox::new(&mut self.is_number_last, "is number last?"));
            if ui.button("Check Files").clicked(){ 
                extract::iter_over_all_files_check_files(&self.root_path_anime);
                extract::iter_over_all_files_check_files(&self.root_path_anime_not);
            }

        });

    }
}