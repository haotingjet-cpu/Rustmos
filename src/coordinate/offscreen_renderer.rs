use eframe::egui;
use wgpu::util::DeviceExt;

#[allow(dead_code)]
pub(crate) struct OffscreenRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) uniform_buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,

    pub(crate) msaa_texture_view: wgpu::TextureView,
    pub(crate) resolve_texture: wgpu::Texture,
    pub(crate) resolve_texture_view: wgpu::TextureView,
    pub(crate) egui_texture_id: egui::TextureId,

    pub(crate) last_render_size: (u32, u32),
}

impl OffscreenRenderer {
    pub(crate) fn new(device: &wgpu::Device, renderer: &mut egui_wgpu::Renderer) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("axes_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let coordinate_uniform_init = crate::coordinate::CoordinateUniform {
            transform: 1.0,
            s: 1.0,
            center: [1.0; 2],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Coordinate"),
            contents: bytemuck::cast_slice(&[coordinate_uniform_init]),
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::UNIFORM,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("time_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(), // 綁定剛剛的緩衝區
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("axes_pipeline_layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        // 先預設 400
        let (width, height) = (800, 800);

        let msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Offscreen MSAA Buffer"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4, // 🚀 開啟 4x MSAA
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let msaa_texture_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let resolve_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Offscreen Resolve Target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let resolve_texture_view =
            resolve_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let egui_texture_id = renderer.register_native_texture(
            &device,
            &resolve_texture_view,
            wgpu::FilterMode::Linear, // 讓縮放時保持平滑
        );

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
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, //Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self {
            pipeline,
            uniform_buffer,
            bind_group,
            msaa_texture_view,
            resolve_texture,
            resolve_texture_view,
            egui_texture_id,
            last_render_size: (800, 800),
        }
    }

    pub fn check_and_resize(
        &mut self,
        device: &wgpu::Device,
        egui_renderer: &mut egui_wgpu::Renderer, // 接收傳進來的 egui 渲染器
        new_width: u32,
        new_height: u32,
    ) {
        // 1. 防禦性程式碼：防止寬高為 0（例如視窗被最小化時）導致 GPU 崩潰
        let new_width = new_width.max(1);
        let new_height = new_height.max(1);

        let (last_width, last_height) = self.last_render_size;

        // 2. 🚀 關鍵檢查：如果尺寸與上一次渲染一模一樣，代表用戶沒有拉動視窗，直接跳過不浪費效能
        if last_width == new_width && last_height == new_height {
            return;
        }

        // 3. 更新當前記錄的尺寸
        self.last_render_size = (new_width, new_height);

        // ==========================================
        // 4. 重新建立 4x MSAA 紋理（繪製目標）
        // ==========================================
        let msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Dynamic Offscreen MSAA Buffer"),
            size: wgpu::Extent3d {
                width: new_width,
                height: new_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4, // 保持 4x MSAA
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb, // 必須與 Pipeline 一致
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        // 覆蓋舊的 View，舊的會因為 Rust 的所有權機制自動被釋放（Drop）
        self.msaa_texture_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // ==========================================
        // 5. 重新建立 1x Resolve 紋理（解析目標）
        // ==========================================
        let resolve_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Dynamic Offscreen Resolve Target"),
            size: wgpu::Extent3d {
                width: new_width,
                height: new_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // 💡 記得加上 TEXTURE_BINDING，egui 才能讀取它當作貼圖
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        self.resolve_texture_view =
            resolve_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // ==========================================
        // 6. 🚀 終極魔法：更新 egui 內部的貼圖綁定
        // ==========================================
        // 這個函式會把我們原有的 self.egui_texture_id 重新指向剛做好的 1x 紋理視圖
        egui_renderer.update_egui_texture_from_wgpu_texture(
            device,                     // 1. wgpu 設備
            &self.resolve_texture_view, // 2. 全新尺寸的 1x 紋理視圖
            wgpu::FilterMode::Linear,   // 3. 縮放過濾模式
            self.egui_texture_id,       // 4. 原本長存的 ID
        );
    }
}
