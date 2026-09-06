use eframe::egui;
pub(crate) enum Color {
    LightBlue,
    Gray,
}
impl Color {
    pub(crate) fn get(&self) -> egui::Color32 {
        match self {
            Self::LightBlue => egui::Color32::from_rgb(33, 150, 243),
            Self::Gray => egui::Color32::from_rgb(180, 180, 180),
        }
    }
}
