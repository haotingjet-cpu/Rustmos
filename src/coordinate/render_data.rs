use wgpu::util::DeviceExt;

pub struct UniformData<T> {
    pub data: T,
    pub bindgroup_layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

pub struct RenderData {
    pub coordinate_uniform: UniformData<super::CoordinateUniform>,
}

impl<T> UniformData<T> {
    pub fn get_ref_bindgroup_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bindgroup_layout
    }
}

impl RenderData {
    pub fn new(device: &wgpu::Device) -> Self {
        let coordinate_uniform = {
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

            let bindgroup_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

            let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("time_bind_group"),
                layout: &bindgroup_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

            UniformData {
                data: coordinate_uniform_init,
                bindgroup_layout,
                bindgroup,
                uniform_buffer,
            }
        };

        Self { coordinate_uniform }
    }

    pub fn write_data_to_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.coordinate_uniform.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.coordinate_uniform.data]),
        );
    }
}
