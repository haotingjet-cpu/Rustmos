use eframe::egui;

#[allow(dead_code)]
pub(crate) enum Color {
    LightBlue,
    Gray,
    DarkGray,
}
impl Color {
    pub(crate) fn get(&self) -> egui::Color32 {
        match self {
            Self::LightBlue => egui::Color32::from_rgb(33, 150, 243),
            Self::Gray => egui::Color32::from_rgb(180, 180, 180),
            Self::DarkGray => egui::Color32::from_rgb(120, 120, 120),
        }
    }
}
