use eframe::egui;

use super::theam_color::Color;

impl<'a> super::InputBox<'a> {
    pub(crate) fn new(_hash: &'a str) -> Self {
        super::InputBox {
            content: String::new(),
            color: Color::Blue,
            _hash,
        }
    }
}

pub(super) fn create_box(
    obj: &mut super::InputBox,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    order: usize,
    delete_fuc: &mut Vec<usize>,
) {
    egui::Frame::NONE
        .fill(egui::Color32::WHITE) // 輸入區域主要是白色
        .stroke(egui::Stroke::new(1.0, Color::LightBlue)) // 圖片中的深藍色外邊框
        .inner_margin(egui::Margin::ZERO) // 關鍵：外層邊距歸零，讓藍色方塊能貼齊邊框
        .show(ui, |ui| {
            let total_height = 40.0; // 調整到適合單行輸入的高度

            // 消除水平排列時元件之間的預設間距（Gap），達成無縫拼接
            ui.spacing_mut().item_spacing.x = 0.0;

            ui.horizontal(|ui| {
                // 2. 左側藍色區塊 (1)
                egui::Frame::NONE
                    .fill(obj.color.into()) // 圖片中的高飽和度藍色
                    .inner_margin(egui::Margin::symmetric(10, 0)) // 左右內邊距使用 i8 整數
                    .show(ui, |ui| {
                        ui.set_height(total_height);
                        ui.set_width(20.0);

                        ui.centered_and_justified(|ui| {
                            ui.label(
                                egui::RichText::new(format!("{}", order + 1))
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

                let font_id = egui::FontId::new(14.0, egui::FontFamily::Proportional);
                text_style.override_font_id = Some(font_id);

                // 建立輸入框
                let text_edit = egui::TextEdit::singleline(&mut obj.content)
                    .desired_width(text_edit_width)
                    .margin(egui::Margin::symmetric(6, 4));

                ui.add_sized([text_edit_width, total_height], text_edit);

                // 4. 右側灰色「✕」清除按鈕
                let btn_style = ui.style_mut();
                btn_style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                btn_style.visuals.widgets.hovered.bg_fill = egui::Color32::from_white_alpha(10);
                btn_style.visuals.widgets.active.bg_fill = egui::Color32::from_white_alpha(20);

                btn_style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
                btn_style.visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
                btn_style.visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

                let close_button =
                    egui::Button::new(egui::RichText::new("X").color(Color::Gray).size(16.0));

                if ui
                    .add_sized([button_width, total_height], close_button)
                    .clicked()
                {
                    delete_fuc.push(order);
                }
            });
        });
}
