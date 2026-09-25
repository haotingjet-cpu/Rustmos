use wgpu::util::DeviceExt;

pub struct BufferData<T> {
    pub data: T,
    pub bindgroup_layout: wgpu::BindGroupLayout,
    pub bindgroup: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
}

pub struct RenderData {
    pub coordinate_uniform: BufferData<super::CoordinateUniform>,
    pub bytecode_uniform: BufferData<(
        super::CoordinateUniformForCompute,
        crate::equation::InputInstruction,
    )>,
    pub equation_vertex_buffer: BufferData<[super::vertex::Vertex; crate::pub_const::VERTEX_NUM]>,
}

impl<T> BufferData<T> {
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
                center: [0.0; 2],
                size: [5.0, 5.0],
                _pad: [0; 2],
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

            BufferData {
                data: coordinate_uniform_init,
                bindgroup_layout,
                bindgroup,
                buffer: uniform_buffer,
            }
        };
        let bytecode_uniform = {
            let input_instruction_uniform_init = crate::equation::InputInstruction::init();

            let coordinate_uniform_for_compute_init = super::CoordinateUniformForCompute::init();

            let input_instruction_uniform_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("bytecode uniform"),
                    contents: bytemuck::cast_slice(&[input_instruction_uniform_init]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });

            let coordinate_uniform_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("bytecode uniform"),
                    contents: bytemuck::cast_slice(&[input_instruction_uniform_init]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });

            let bindgroup_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("bind group layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

            let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("time_bind_group"),
                layout: &bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: input_instruction_uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: coordinate_uniform_buffer.as_entire_binding(),
                    },
                ],
            });

            BufferData {
                data: (
                    coordinate_uniform_for_compute_init,
                    input_instruction_uniform_init,
                ),
                bindgroup_layout,
                bindgroup,
                buffer: input_instruction_uniform_buffer,
            }
        };

        let equation_vertex_buffer = {
            let vertex_buffer_init = super::vertex::Vertex::init();
            let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("equation uniform"),
                contents: bytemuck::cast_slice(&[vertex_buffer_init]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
            });

            let bindgroup_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("bind group layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,

                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
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
                    resource: storage_buffer.as_entire_binding(),
                }],
            });

            BufferData {
                data: vertex_buffer_init,
                bindgroup,
                bindgroup_layout,
                buffer: storage_buffer,
            }
        };

        Self {
            coordinate_uniform,
            bytecode_uniform,
            equation_vertex_buffer,
        }
    }

    pub fn write_data_to_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.coordinate_uniform.buffer,
            0,
            bytemuck::cast_slice(&[self.coordinate_uniform.data]),
        );
    }
}
