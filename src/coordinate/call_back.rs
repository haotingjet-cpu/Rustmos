use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct MyCallback {
    pub(crate) renderer: Arc<Mutex<super::offscreen_renderer::OffscreenRenderer>>,
    pub(crate) transform_buffer: Arc<Mutex<super::transform::TransformBuffer>>,
    pub target_width: u32,
    pub target_height: u32,
    pub(crate) coordinate_discripter: super::CoordinateDiscripter,
}

impl egui_wgpu::CallbackTrait for MyCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        // if let Some(resources) =
        //     callback_resources.get::<super::render_sources::MyRenderResources>()
        // {
        let mut renderer = self.renderer.lock();
        let mut transform_buffer_storage = self.transform_buffer.lock();
        if let Some(wgpu_state) = callback_resources.get::<egui_wgpu::RenderState>() {
            let mut egui_renderer = wgpu_state.renderer.write();

            // 執行動態縮放與更新 TextureId
            renderer.check_and_resize(
                device,
                &mut egui_renderer,
                self.target_width,
                self.target_height,
            );
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Eframe Offscreen MSAA Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &renderer.msaa_texture_view,
                    resolve_target: Some(&renderer.resolve_texture_view), // 🚀 自動縮小
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None, // 如果有深度緩衝區再補
                occlusion_query_set: None,
                timestamp_writes: None,
                ..Default::default()
            });

            // println!("{:?}", resources.center);

            queue.write_buffer(
                &renderer.uniform_buffer,
                0,
                bytemuck::cast_slice(&[self.coordinate_discripter]),
            );

            let transform = super::Transform {
                transform: self.target_width as f32 / self.target_height as f32,
                _pad: [0; 3],
            };

            queue.write_buffer(
                &transform_buffer_storage.transform_buffer,
                0,
                bytemuck::cast_slice(&[transform]),
            );

            render_pass.set_bind_group(1, &renderer.bind_group, &[]);
            render_pass.set_bind_group(0, &transform_buffer_storage.bind_group, &[]);

            // 畫出你的 3D 場景（Pipeline 的 count 為 4）
            render_pass.set_pipeline(&renderer.pipeline);
            render_pass.draw(0..12, 0..1);
            // }
        }

        Vec::new()
    }

    fn paint(
        &self,
        _info: eframe::egui::PaintCallbackInfo,
        _render_pass: &mut wgpu::RenderPass<'static>,
        _callback_resources: &egui_wgpu::CallbackResources,
    ) {
    }
}
