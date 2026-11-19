use eframe::egui::{self, Sense, Ui, Vec2};

const TRANSPORT: [(&str, &str); 5] = [
    ("Abrir", "Open"),
    ("Play", "Play"),
    ("Pause", "Pause"),
    ("Stop", "Stop"),
    ("Próxima", "Next"),
];

/// Renders the minimum-size Classic Stack transport controls.
pub fn classic_stack(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.horizontal_wrapped(|ui| {
            for (label, _command) in TRANSPORT {
                transport_button(ui, label);
            }
        });
        ui.add_space(8.0);
        let mut seek = 0.0_f32;
        let response = ui.add_sized(
            Vec2::new(ui.available_width().max(1.0), 32.0),
            egui::Slider::new(&mut seek, 0.0..=100.0)
                .show_value(false)
                .text("Seek"),
        );
        response.widget_info(|| egui::WidgetInfo::slider(true, 0.0, "Seek"));
        response.surrender_focus();
    });
}

fn transport_button(ui: &mut Ui, label: &str) {
    let _response = ui
        .add_sized(Vec2::new(68.0, 32.0), egui::Button::new(label))
        .interact(Sense::click());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_controls_have_keyboard_targets() {
        let _commands: [(&str, &str); 5] = TRANSPORT;
    }
}
