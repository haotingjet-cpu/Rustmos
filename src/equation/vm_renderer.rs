pub struct ComputeRenderer {
    pipeline: wgpu::ComputePipeline,
}

impl ComputeRenderer {
    pub fn new(
        device: &wgpu::Device,
        render_data: &crate::coordinate::render_data::RenderData,
    ) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute pipeline"),
            source: wgpu::ShaderSource::Wgsl(include_str!("draw_equation/vm_compute.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute_pipeline_layout"),
            bind_group_layouts: &[Some(
                render_data.bytecode_uniform.get_ref_bindgroup_layout(),
            )],
            immediate_size: 0,
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: Some("vm_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        Self { pipeline }
    }
}
