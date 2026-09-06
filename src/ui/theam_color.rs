use eframe::egui;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Color {
    Blue,
    LightBlue,
    Gray,
    DarkGray,
    Purple,
    LightPurple,
}

impl From<Color> for egui::Color32 {
    fn from(color: Color) -> Self {
        match color {
            Color::Blue => egui::Color32::from_rgb(33, 150, 243),
            Color::Gray => egui::Color32::from_rgb(180, 180, 180),
            Color::DarkGray => egui::Color32::from_rgb(120, 120, 120),
            Color::Purple => egui::Color32::from_rgb(156, 39, 176),
            Color::LightBlue => egui::Color32::from_rgb(25, 118, 210),
            Color::LightPurple => egui::Color32::from_rgb(225, 190, 231),
        }
    }
}
