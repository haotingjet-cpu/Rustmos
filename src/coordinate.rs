use eframe::egui;

/// transform: width / height
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CoordinateUniform {
    pub(crate) transform: f32,
    pub(crate) s: f32,
    pub(crate) center: [f32; 2],
}

pub(crate) struct Coordinate {
    pub(crate) transform: f32,
    pub(crate) s: f32,
    pub(crate) center: [f32; 2],
}

pub(crate) struct MyRenderResources {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) uniform_buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl egui_wgpu::CallbackTrait for Coordinate {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &MyRenderResources = callback_resources.get().unwrap();

        let coordinate_uniform = CoordinateUniform {
            transform: self.transform,
            s: self.s,
            center: self.center,
        };

        queue.write_buffer(
            &resources.uniform_buffer,
            0,
            bytemuck::cast_slice(&[coordinate_uniform]),
        );

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        let resources: &MyRenderResources = callback_resources.get().unwrap();

        render_pass.set_bind_group(0, &resources.bind_group, &[]);

        render_pass.set_pipeline(&resources.pipeline);

        render_pass.draw(0..12, 0..1);
    }
}
