use eframe::egui;

pub fn ui_main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "I Am The Backend",
        opts,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    host: String,
    port: u16,
    output: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            host: "localhost".to_owned(),
            port: 8080,
            output: "Hello, frontend!".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // disabled multiline text edit for output display
                ui.add_enabled( false,
                    egui::TextEdit::multiline(&mut self.output)
                        .code_editor()
                        .lock_focus(false)
                        .desired_width(400.0)
                );

                ui.vertical(|ui| {
                    ui.heading("Server Configuration");

                    ui.horizontal(|ui| {
                        ui.label("Host:");
                        ui.text_edit_singleline(&mut self.host);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Port:");
                        ui.add(egui::DragValue::new(&mut self.port).range(1..=65535));
                    });

                    if ui.button("Connect").clicked() {
                        println!("Connecting to {}:{}", self.host, self.port);
                    }
                })
            })
        });
    }
}
