use eframe::egui;

#[derive(Debug, Clone)]
pub(crate) struct Coordinate {
    pub(crate) transform: [[f32; 2]; 2],
}

pub(crate) struct MyRenderResources {
    pub(crate) pipeline: wgpu::RenderPipeline,
}

impl egui_wgpu::CallbackTrait for Coordinate {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let _resources: &MyRenderResources = callback_resources.get().unwrap();
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        let resources: &MyRenderResources = callback_resources.get().unwrap();

        render_pass.set_pipeline(&resources.pipeline);

        render_pass.draw(0..3, 0..1);
    }
}
