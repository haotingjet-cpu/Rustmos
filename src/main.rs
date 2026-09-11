use eframe::egui;
use egui::Key;
use parking_lot::Mutex;
use std::sync::Arc;

mod coordinate;
mod ui;

pub(crate) struct MyApp<'a> {
    functions: Vec<ui::InputBox<'a>>,
    pub(crate) deleted_func: Vec<usize>,
    is_colsed: bool,
    center: [f32; 2],
    s: f32,
    offscreen_renderer: Arc<Mutex<coordinate::offscreen_renderer::OffscreenRenderer>>,
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "my app",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

impl<'a> MyApp<'a> {
    fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_state = cc.wgpu_render_state.as_ref().expect("");
        let device = &wgpu_state.device;

        let offscreen_renderer = coordinate::offscreen_renderer::OffscreenRenderer::new(
            device,
            &mut *wgpu_state.renderer.write(),
        );

        // wgpu_state.renderer.write().callback_resources.insert(
        //     coordinate::render_sources::MyRenderResources {
        //         target_width: 400,
        //         target_height: 400,
        //         s: 1.0,
        //         center: [0.0; 2],
        //     },
        // );

        Self {
            functions: Vec::new(),
            deleted_func: Vec::new(),
            is_colsed: false,
            center: [0.0; 2],
            s: 1.0,
            offscreen_renderer: Arc::new(Mutex::new(offscreen_renderer)),
        }
    }
}

impl<'a> eframe::App for MyApp<'a> {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui::create_left_bar(self, ui, frame);
        // 2. 主畫面
        egui::CentralPanel::default().show(ui, |ui| {
            let available_size = ui.available_size();
            let (rect, _response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

            let pixels_per_point = ui.ctx().pixels_per_point();

            let w = (rect.width() * pixels_per_point).round() as u32;
            let h = (rect.height() * pixels_per_point).round() as u32;

            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                rect,
                coordinate::call_back::MyCallback {
                    renderer: self.offscreen_renderer.clone(),
                    target_width: w,
                    target_height: h,
                    s: 1.0,
                    center: self.center,
                },
            ));

            let texture_id = self.offscreen_renderer.lock().egui_texture_id;

            ui.put(
                rect,
                egui::Image::new(egui::load::SizedTexture::new(texture_id, available_size)),
            );
        });
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_down(Key::ArrowUp)) {
            println!("up");
            self.center[1] += 0.01;
        }
        if ctx.input(|i| i.key_down(Key::ArrowDown)) {
            self.center[1] -= 0.01;
        }
        if ctx.input(|i| i.key_down(Key::ArrowLeft)) {
            self.center[0] -= 0.01;
        }
        if ctx.input(|i| i.key_down(Key::ArrowRight)) {
            self.center[0] += 0.01;
        }
        if ctx.input(|i| i.key_down(Key::W)) {
            self.s *= 1.005;
        }
        if ctx.input(|i| i.key_down(Key::S)) {
            self.s /= 1.005;
        }
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
