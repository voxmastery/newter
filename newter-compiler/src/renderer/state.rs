//! wgpu state: device, queue, pipeline for drawing rectangles (and text when glyphon supports wgpu 22+).

use crate::layout::{LayoutNode, LayoutKind};
use bytemuck::{Pod, Zeroable};
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
    rect_pixels: [f32; 4],
    rect_radius: f32,
    stroke_color: [f32; 4],
    stroke_width: f32,
    viewport: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Uniforms {
    viewport: [f32; 2], // width, height
    _pad: [f32; 2],
}

pub struct DrawRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub r: f32, // corner radius
    pub fill: [f32; 4],
    pub stroke: Option<[f32; 4]>,
}

pub struct RendererState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    #[allow(dead_code)] // must stay alive — shared Arc used by text_atlas/viewport
    glyphon_cache: glyphon::Cache,
    glyphon_viewport: glyphon::Viewport,
    text_atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
}

impl RendererState {
    pub async fn new(window: &Window) -> Result<Self, String> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window).map_err(|e| e.to_string())?;
        let surface: wgpu::Surface<'static> = unsafe { std::mem::transmute(surface) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("No GPU adapter")?;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("newter device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .map_err(|e| e.to_string())?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("rect shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("rect.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("rect pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rect pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as u64,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        wgpu::VertexAttribute {
                            offset: (std::mem::size_of::<[f32; 2]>() + std::mem::size_of::<[f32; 4]>()) as u64,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        wgpu::VertexAttribute {
                            offset: (std::mem::size_of::<[f32; 2]>() + std::mem::size_of::<[f32; 4]>() * 2) as u64,
                            shader_location: 3,
                            format: wgpu::VertexFormat::Float32,
                        },
                        wgpu::VertexAttribute {
                            offset: (std::mem::size_of::<[f32; 2]>() + std::mem::size_of::<[f32; 4]>() * 2 + 4) as u64,
                            shader_location: 4,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                        wgpu::VertexAttribute {
                            offset: (std::mem::size_of::<[f32; 2]>() + std::mem::size_of::<[f32; 4]>() * 3 + 4) as u64,
                            shader_location: 5,
                            format: wgpu::VertexFormat::Float32,
                        },
                        wgpu::VertexAttribute {
                            offset: (std::mem::size_of::<[f32; 2]>() + std::mem::size_of::<[f32; 4]>() * 3 + 4 + 4) as u64,
                            shader_location: 6,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rect vertex buffer"),
            size: (100_000 * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();
        let glyphon_cache = glyphon::Cache::new(&device);
        let glyphon_viewport = glyphon::Viewport::new(&device, &glyphon_cache);
        let mut text_atlas = glyphon::TextAtlas::new(&device, &queue, &glyphon_cache, config.format);
        let text_renderer = glyphon::TextRenderer::new(
            &mut text_atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );

        Ok(Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            vertex_count: 0,
            font_system,
            swash_cache,
            glyphon_cache,
            glyphon_viewport,
            text_atlas,
            text_renderer,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn rect_to_vertices(rect: &DrawRect, viewport_w: f32, viewport_h: f32) -> [Vertex; 6] {
        let x0 = (rect.x / viewport_w) * 2.0 - 1.0;
        let y0 = 1.0 - (rect.y / viewport_h) * 2.0;
        let x1 = ((rect.x + rect.w) / viewport_w) * 2.0 - 1.0;
        let y1 = 1.0 - ((rect.y + rect.h) / viewport_h) * 2.0;
        let c = rect.fill;
        let rp = [rect.x, rect.y, rect.w, rect.h];
        let rr = rect.r;
        let (stroke_color, stroke_width) = rect
            .stroke
            .map(|s| (s, 1.0))
            .unwrap_or(([0.0, 0.0, 0.0, 0.0], 0.0));
        let viewport = [viewport_w, viewport_h];
        let v = |pos: [f32; 2]| Vertex {
            position: pos,
            color: c,
            rect_pixels: rp,
            rect_radius: rr,
            stroke_color,
            stroke_width,
            viewport,
        };
        [
            v([x0, y0]),
            v([x1, y0]),
            v([x0, y1]),
            v([x0, y1]),
            v([x1, y0]),
            v([x1, y1]),
        ]
    }

    pub fn draw_layout(&mut self, root: &LayoutNode) {
        let viewport_w = self.config.width as f32;
        let viewport_h = self.config.height as f32;

        // 1. Collect and upload rect vertices
        let mut rects = Vec::new();
        collect_rects(root, viewport_w, viewport_h, &mut rects);
        let mut vertices: Vec<Vertex> = Vec::new();
        for r in &rects {
            let v = Self::rect_to_vertices(r, viewport_w, viewport_h);
            vertices.extend_from_slice(&v);
        }
        self.vertex_count = vertices.len() as u32;
        if self.vertex_count > 0 {
            self.queue.write_buffer(
                &self.vertex_buffer,
                0,
                bytemuck::cast_slice(&vertices),
            );
        }

        // 2. Collect text nodes
        let mut text_nodes = Vec::new();
        collect_text_nodes(root, &mut text_nodes);

        // 3. Update glyphon viewport
        self.glyphon_viewport.update(&self.queue, glyphon::Resolution {
            width: viewport_w as u32,
            height: viewport_h as u32,
        });

        // 4. Create buffers for each text node
        let mut buffers: Vec<glyphon::Buffer> = Vec::new();
        for tn in &text_nodes {
            let metrics = glyphon::Metrics::new(tn.font_size, tn.font_size * 1.2);
            let mut buffer = glyphon::Buffer::new(&mut self.font_system, metrics);
            buffer.set_size(&mut self.font_system, Some(tn.w), Some(tn.h));
            buffer.set_text(
                &mut self.font_system,
                &tn.text,
                glyphon::Attrs::new().family(glyphon::Family::SansSerif),
                glyphon::Shaping::Advanced,
            );
            buffer.shape_until_scroll(&mut self.font_system, false);
            buffers.push(buffer);
        }

        // 5. Build TextAreas and prepare renderer
        let text_areas: Vec<glyphon::TextArea> = text_nodes
            .iter()
            .zip(buffers.iter())
            .map(|(tn, buf)| {
                let default_color = if tn.has_dark_bg {
                    glyphon::Color::rgb(226, 226, 232)
                } else {
                    glyphon::Color::rgb(26, 26, 32)
                };
                glyphon::TextArea {
                    buffer: buf,
                    left: tn.x,
                    top: tn.y,
                    scale: 1.0,
                    bounds: glyphon::TextBounds {
                        left: tn.x as i32,
                        top: tn.y as i32,
                        right: (tn.x + tn.w) as i32,
                        bottom: (tn.y + tn.h) as i32,
                    },
                    default_color,
                    custom_glyphs: &[],
                }
            })
            .collect();

        let _ = self.text_renderer.prepare(
            &self.device,
            &self.queue,
            &mut self.font_system,
            &mut self.text_atlas,
            &self.glyphon_viewport,
            text_areas,
            &mut self.swash_cache,
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render encoder"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.95,
                            g: 0.95,
                            b: 0.97,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..self.vertex_count, 0..1);

            let _ = self.text_renderer.render(
                &self.text_atlas,
                &self.glyphon_viewport,
                &mut pass,
            );
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

struct TextNode {
    text: String,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    font_size: f32,
    has_dark_bg: bool,
}

fn collect_text_nodes(node: &LayoutNode, out: &mut Vec<TextNode>) {
    if let Some(ref text) = node.text {
        let has_dark_bg = node.fill
            .map(|(r, g, b, _)| (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) < 128.0)
            .unwrap_or(false);
        out.push(TextNode {
            text: text.clone(),
            x: node.rect.x,
            y: node.rect.y,
            w: node.rect.w,
            h: node.rect.h,
            font_size: node.font_size.max(8.0),
            has_dark_bg,
        });
    }
    for child in &node.children {
        collect_text_nodes(child, out);
    }
}

fn collect_rects(node: &LayoutNode, _vw: f32, _vh: f32, out: &mut Vec<DrawRect>) {
    let r = &node.rect;
    let fill = node
        .fill
        .map(|(r, g, b, a)| [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0])
        .unwrap_or([0.9, 0.9, 0.9, 1.0]);
    let stroke = node.stroke.map(|(r, g, b, a)| {
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0]
    });
    match node.kind {
        LayoutKind::Box | LayoutKind::Row | LayoutKind::Column | LayoutKind::Grid | LayoutKind::Stack | LayoutKind::Center
        | LayoutKind::Button | LayoutKind::Input | LayoutKind::Modal => {
            out.push(DrawRect {
                x: r.x,
                y: r.y,
                w: r.w,
                h: r.h,
                r: node.radius,
                fill,
                stroke,
            });
        }
        LayoutKind::Text | LayoutKind::Spacer | LayoutKind::Image => {
            if node.kind == LayoutKind::Text && node.fill.is_some() {
                out.push(DrawRect {
                    x: r.x,
                    y: r.y,
                    w: r.w,
                    h: r.h,
                    r: node.radius,
                    fill,
                    stroke,
                });
            }
        }
    }
    for child in &node.children {
        collect_rects(child, _vw, _vh, out);
    }
}
