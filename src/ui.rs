pub(crate) mod input_box;
use eframe::egui::{self, Panel};

pub(crate) fn create_left_bar(
    obj: &mut crate::Content,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
) {
    let target_width = if obj.is_colsed { 60.0 } else { 250.0 };

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
                let button_text = if obj.is_colsed { "▶" } else { "◀ 收合" };
                if ui.button(button_text).clicked() {
                    obj.is_colsed = !obj.is_colsed;
                }

                ui.separator();

                // 依據開關狀態顯示選單或圖示
                if !obj.is_colsed {
                    ui.label("🏠 首頁項目");
                    ui.label("⚙️ 設定項目");
                } else {
                    ui.label("🏠");
                    ui.label("⚙️");
                }
                self::input_box::create_box(obj, ui, frame);
            });
        });
}
