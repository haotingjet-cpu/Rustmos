use wgpu::util::DeviceExt;

pub(crate) struct TransformBuffer {
    pub(crate) transform_buffer: wgpu::Buffer,
    pub(crate) bindgroup_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl TransformBuffer {
    pub(crate) fn new_buffer(device: &wgpu::Device) -> Self {
        let transform_uniform_init = super::Transform {
            transform: 1.0,
            _pad: [0; 3],
        };

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("transform"),
            contents: bytemuck::cast_slice(&[transform_uniform_init]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("transform lauout"),
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
            label: Some("tranform bind group"),
            layout: &bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        Self {
            transform_buffer,
            bindgroup_layout,
            bind_group,
        }
    }
}
