use eframe::egui;

pub fn init_window(){
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
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
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            root_path: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rename");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.root_path)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.root_path, self.age));
        });
    }
}