fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Vynamp", options, Box::new(|_cc| Ok(Box::new(VynampApp))));
}

struct VynampApp;

impl eframe::App for VynampApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        vynamp_player::ui::classic_stack(ui);
    }
}
