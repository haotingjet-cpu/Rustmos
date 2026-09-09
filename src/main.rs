use eframe::egui;
use std::sync::Arc;

use crate::coordinate::Coordinate;
mod coordinate;
mod ui;

pub(crate) struct MyApp<'a> {
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
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

impl<'a> MyApp<'a> {
    fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_state = cc.wgpu_render_state.as_ref().expect("");
        let device = &wgpu_state.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("axes_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("coordinate/shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("axes_pipeline_layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu_state.target_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        wgpu_state
            .renderer
            .write()
            .callback_resources
            .insert(coordinate::MyRenderResources { pipeline });

        Self {
            functions: Vec::new(),
            deleted_func: Vec::new(),
            is_colsed: false,
        }
    }
}

impl<'a> eframe::App for MyApp<'a> {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui::create_left_bar(self, ui, frame);
        // 2. 主畫面
        let main_surface = egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("這是主畫面區域");
            ui.label("之後這邊繪畫出座標");

            let (rect, _response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

            let callback = coordinate::Coordinate {
                transform: [[0.0, 0.0], [0.0, 0.0]],
            };

            ui.painter()
                .add(egui_wgpu::Callback::new_paint_callback(rect, callback));
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
