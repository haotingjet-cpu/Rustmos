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
    Red,
    LightRed,
}

impl From<Color> for egui::Color32 {
    fn from(color: Color) -> Self {
        match color {
            Color::Red => egui::Color32::from_rgb(244, 67, 54),
            Color::LightRed => egui::Color32::from_rgb(255, 205, 210),
            Color::Blue => egui::Color32::from_rgb(33, 150, 243),
            Color::Gray => egui::Color32::from_rgb(180, 180, 180),
            Color::DarkGray => egui::Color32::from_rgb(120, 120, 120),
            Color::Purple => egui::Color32::from_rgb(156, 39, 176),
            Color::LightBlue => egui::Color32::from_rgb(25, 118, 210),
            Color::LightPurple => egui::Color32::from_rgb(225, 190, 231),
        }
    }
}

impl Color {
    pub fn get_raw(self) -> [f32; 4] {
        let mut c = match self {
            Color::Red => [244.0, 67.0, 54.0, 0.0],
            Color::LightRed => [255.0, 205.0, 210.0, 0.0],
            Color::Blue => [33.0, 150.0, 243.0, 0.0],
            Color::Gray => [180.0, 180.0, 180.0, 0.0],
            Color::DarkGray => [120.0, 120.0, 120.0, 0.0],
            Color::Purple => [156.0, 39.0, 176.0, 0.0],
            Color::LightBlue => [25.0, 118.0, 210.0, 0.0],
            Color::LightPurple => [225.0, 190.0, 231.0, 0.0],
        };
        c[0] /= 255.0;
        c[1] /= 255.0;
        c[2] /= 255.0;
        c[3] /= 255.0;
        c
    }
}
