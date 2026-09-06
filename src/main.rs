use eframe::egui;
use std::sync::Arc;
mod ui;

pub(crate) struct Context<'a> {
    functions: Vec<ui::InputBox<'a>>,
    pub(crate) deleted_func: Vec<usize>,
    is_colsed: bool,
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            //.with_inner_size([960.0, 640.0])
            .with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "my app",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(Context::default()))
        }),
    )
}

impl<'a> Default for Context<'a> {
    fn default() -> Self {
        Self {
            functions: vec![ui::InputBox::new("abc54")],
            deleted_func: vec![],
            is_colsed: false,
        }
    }
}

impl<'a> eframe::App for Context<'a> {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui::create_left_bar(self, ui, frame);
        // 2. 主畫面
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("這是主畫面區域");
            ui.label("側邊欄收合時，主畫面會自動往左展開填滿！");
        });
    }
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_chinese_font".to_owned(),
        Arc::new(egui::FontData::from_owned(
            include_bytes!("../fonts/SourceHanSerif-Regular.ttc").to_vec(),
        )),
    );

    if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        vec.insert(0, "my_chinese_font".to_owned());
    }

    if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        vec.insert(0, "my_chinese_font".to_owned());
    }
    ctx.set_fonts(fonts);
}
