use eframe::egui::{self, Panel};
use std::sync::Arc;

struct Content {
    text: String,
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
            Ok(Box::new(Content::default()))
        }),
    )
}

impl Default for Content {
    fn default() -> Self {
        Self {
            text: String::new(),
            is_colsed: false,
        }
    }
}

impl eframe::App for Content {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let target_width = if self.is_colsed { 60.0 } else { 250.0 };

        // 1. 建立左側欄
        Panel::left("main_sidebar")
            .resizable(false)
            .default_size(target_width)
            .max_size(target_width)
            .min_size(target_width) //
            .show(ui, |ui| {
                // 【核心修正點】：使用官方正確的 vertical_centered 方法
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    // 展開/收合按鈕
                    let button_text = if self.is_colsed { "▶" } else { "◀ 收合" };
                    if ui.button(button_text).clicked() {
                        self.is_colsed = !self.is_colsed;
                    }

                    ui.separator();

                    // 依據開關狀態顯示選單或圖示
                    if !self.is_colsed {
                        ui.label("🏠 首頁項目");
                        ui.label("⚙️ 設定項目");
                    } else {
                        ui.label("🏠");
                        ui.label("⚙️");
                    }
                    // 1. 最外層容器：定義整個組合輸入框的邊框與整體背景
                    egui::Frame::NONE
                        .fill(egui::Color32::WHITE) // 輸入區域主要是白色
                        .stroke(egui::Stroke::new(
                            1.0,
                            egui::Color32::from_rgb(25, 118, 210),
                        )) // 圖片中的深藍色外邊框
                        .inner_margin(egui::Margin::ZERO) // 關鍵：外層邊距歸零，讓藍色方塊能貼齊邊框
                        .show(ui, |ui| {
                            let total_height = 28.0; // 調整到適合單行輸入的高度

                            // 消除水平排列時元件之間的預設間距（Gap），達成無縫拼接
                            ui.spacing_mut().item_spacing.x = 0.0;

                            ui.horizontal(|ui| {
                                // 2. 左側藍色區塊 (1)
                                egui::Frame::NONE
                                    .fill(egui::Color32::from_rgb(33, 150, 243)) // 圖片中的高飽和度藍色
                                    .inner_margin(egui::Margin::symmetric(10, 0)) // 左右內邊距使用 i8 整數
                                    .show(ui, |ui| {
                                        ui.set_height(total_height);
                                        ui.set_width(28.0);

                                        ui.centered_and_justified(|ui| {
                                            ui.label(
                                                egui::RichText::new("1")
                                                    .color(egui::Color32::WHITE)
                                                    .strong()
                                                    .size(13.0),
                                            );
                                        });
                                    });

                                // 3. 中間主要輸入框
                                // 計算剩餘寬度，並扣除右側清除按鈕預留的空間（大約 28 像素）
                                // 3. 中間主要輸入框
                                // 安全計算剩餘寬度：如果剩餘空間不夠，就給它一個合理的預設寬度（例如 200.0），避免算出負數
                                let button_width = 28.0;
                                let available_w = ui.available_width();
                                let text_edit_width = if available_w > button_width + 50.0 {
                                    available_w - button_width
                                } else {
                                    200.0 // 降級保底寬度，確保不會小於等於 0
                                };

                                // 隱藏 TextEdit 原生的背景與邊框
                                let text_style = ui.style_mut();
                                text_style.visuals.widgets.inactive.bg_fill = egui::Color32::WHITE;
                                text_style.visuals.widgets.hovered.bg_fill = egui::Color32::WHITE;
                                text_style.visuals.widgets.active.bg_fill = egui::Color32::WHITE;

                                text_style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
                                text_style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
                                text_style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

                                let font_id =
                                    egui::FontId::new(16.0, egui::FontFamily::Proportional);
                                text_style.override_font_id = Some(font_id);

                                // 建立輸入框
                                let text_edit = egui::TextEdit::singleline(&mut self.text)
                                    .desired_width(text_edit_width)
                                    .margin(egui::Margin::symmetric(6, 4));

                                ui.add_sized([text_edit_width, total_height], text_edit);

                                // 4. 右側灰色「✕」清除按鈕
                                if !self.text.is_empty() {
                                    let btn_style = ui.style_mut();
                                    btn_style.visuals.widgets.inactive.bg_fill =
                                        egui::Color32::TRANSPARENT;
                                    btn_style.visuals.widgets.hovered.bg_fill =
                                        egui::Color32::from_white_alpha(10);
                                    btn_style.visuals.widgets.active.bg_fill =
                                        egui::Color32::from_white_alpha(20);

                                    btn_style.visuals.widgets.inactive.bg_stroke =
                                        egui::Stroke::NONE;
                                    btn_style.visuals.widgets.hovered.bg_stroke =
                                        egui::Stroke::NONE;
                                    btn_style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

                                    let close_button = egui::Button::new(
                                        egui::RichText::new("X")
                                            .color(egui::Color32::from_rgb(180, 180, 180))
                                            .size(16.0),
                                    );

                                    if ui
                                        .add_sized([button_width, total_height], close_button)
                                        .clicked()
                                    {
                                        self.text.clear();
                                    }
                                } else {
                                    ui.allocate_space(egui::vec2(button_width, total_height));
                                }
                            });
                        });
                });
            });

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
