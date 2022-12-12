use eframe::egui;
use egui::TextBox;

use crate::extract;


pub fn init_window(){
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rename",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}


struct MyApp {
    root_path: String,
    check_files_buffer: String,
    is_everything_correct: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            root_path: "G:\\AN\\Anime".to_owned(),
            check_files_buffer: "".to_owned(),
            is_everything_correct: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Path: ");
                ui.text_edit_singleline(&mut self.root_path)
                    .labelled_by(name_label.id);
            });
            let response = ui.add(egui::TextEdit::multiline(&mut self.check_files_buffer));
            if ui.button("Click Me").clicked(){ 
                extract::iter_over_all_files(&self.root_path, response);
                let mut text_box = TextBox::new("Hello, World!");
            }

        });

    }
}