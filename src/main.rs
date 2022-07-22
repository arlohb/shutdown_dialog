use eframe::egui;

struct App {}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }

    pub fn creator() -> eframe::AppCreator {
        Box::new(|cc| Box::new(App::new(cc)))
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello world!!");
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native("Shutdown dialog", native_options, App::creator());
}
