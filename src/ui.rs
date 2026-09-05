pub(crate) mod input_box;
use eframe::egui::{self, Panel};

pub(crate) struct InputBox<'a> {
    pub(crate) content: String,
    pub(crate) color: egui::Color32,
    pub(crate) hash: &'a str,
}

pub(crate) fn create_left_bar(
    obj: &mut crate::Context,
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
) {
    let target_width = if obj.is_colsed { 60.0 } else { 400.0 };

    // 1. 建立左側欄
    Panel::left("main_sidebar")
        .resizable(false)
        .default_size(target_width)
        .max_size(target_width)
        .min_size(target_width - 100.0) //
        .show(ui, |ui| {
            // 【核心修正點】：使用官方正確的 vertical_centered 方法
            ui.vertical_centered(|ui| {
                // 展開/收合按鈕
                ui.horizontal(|ui| {
                    // 減 10 是因為右側會超出
                    ui.set_max_width(target_width - 10.0);
                    if !obj.is_colsed {
                        ui.label("⚙️ 設定項目");
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let button_text = if obj.is_colsed { "▶" } else { "◀ 收合" };
                        if ui.button(button_text).clicked() {
                            obj.is_colsed = !obj.is_colsed;
                        }
                    });
                });

                ui.separator();

                // 依據開關狀態顯示選單或圖示
                for i in obj.deleted_func.iter().rev() {
                    obj.functions.remove(*i);
                }
                obj.deleted_func.clear();
                for (order, input_box) in obj.functions.iter_mut().enumerate() {
                    self::input_box::create_box(input_box, ui, frame, order, &mut obj.deleted_func);
                }
            });
        });
}
