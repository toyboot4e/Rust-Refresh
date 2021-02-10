//! Simple test app

use std::{
    env,
    ffi::CString,
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use refresh::{img::Image, Resource};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}

impl From<([f32; 3], [f32; 2])> for Vertex {
    fn from(xs: ([f32; 3], [f32; 2])) -> Vertex {
        Vertex {
            pos: xs.0,
            uv: xs.1,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RaymarchUniforms {
    time: f32,
    pad: f32,
    res: [f32; 2],
}

impl RaymarchUniforms {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self as *const _ as *const _, std::mem::size_of::<Self>())
        }
    }
}

unsafe extern "C" fn logger(msg: *const std::os::raw::c_char) {
    let m = CString::from_raw(msg as *mut _);
    let m_str = m.to_str().unwrap();
    println!("{}", m_str);
    std::mem::forget(m);
}

pub fn main() -> Result<(), String> {
    refresh::hook_log_functions(Some(logger), Some(logger), Some(logger));

    let scx = sdl2::init()?;
    let video = scx.video()?;

    let win_size = [1280.0 as u32, 720.0 as u32];

    let window = video
        .window("Simple app", win_size[0], win_size[1])
        .position_centered()
        .vulkan()
        .build()
        .map_err(|e| e.to_string())?;

    let device = {
        let params = refresh::PresentationParameters {
            deviceWindowHandle: window.raw() as *mut _,
            presentMode: refresh::PresentMode::Immediate as u32,
            ..Default::default()
        };
        let is_debug = true;
        refresh::Device::new(&params, is_debug)
    };

    let render_area = refresh::Rect {
        x: 0,
        y: 0,
        w: win_size[0] as i32,
        h: win_size[1] as i32,
    };

    // compile shaders
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = root.join("examples/a-res");

    let passthrough_shader = {
        let bytes = fs::read(dir.join("passthrough_vert.spv")).unwrap();
        refresh::ShaderModule::create(
            &device,
            &refresh::ShaderModuleCreateInfo {
                byteCode: bytes.as_ptr() as *const _,
                codeSize: bytes.len() as _,
            },
        )
    };

    let raymarch_shader = {
        let bytes = fs::read(dir.join("hexagon_grid.spv")).unwrap();
        refresh::ShaderModule::create(
            &device,
            &refresh::ShaderModuleCreateInfo {
                byteCode: bytes.as_ptr() as *const _,
                codeSize: bytes.len() as _,
            },
        )
    };

    // load textures
    let (wood_tex, wood_tex_slice) = {
        let wood_name = CString::new(format!("{}", root.join("woodgrain.png").display())).unwrap();
        let wood_img = Image::load_name(&wood_name);

        let wood_tex = refresh::Texture::create(
            &device,
            &refresh::TextureCreateInfo {
                width: wood_img.w,
                height: wood_img.h,
                depth: 0,
                isCube: false as _,
                sampleCount: refresh::SampleCount::N1 as _,
                levelCount: 1,
                format: refresh::TextureFormat::R8G8B8A8 as _,
                usageFlags: refresh::TextureUsageFlags::SAMPLER.bits(),
            },
        );

        let tex_slice = refresh::TextureSlice {
            texture: wood_tex,
            rectangle: refresh::Rect {
                x: 0,
                y: 0,
                w: wood_img.w as _,
                h: wood_img.h as _,
            },
            depth: 0,
            layer: 0,
            level: 0,
        };
        device.set_texture_data(&tex_slice, &wood_img.pixels);
        (wood_tex, tex_slice)
    };

    let noise_tex = {
        let noise_name = CString::new(format!("{}", root.join("noise.png").display())).unwrap();
        let noise_img = Image::load_name(&noise_name);
        refresh::Texture::create(
            &device,
            &refresh::TextureCreateInfo {
                width: noise_img.w,
                height: noise_img.h,
                depth: 0,
                isCube: false as _,
                sampleCount: refresh::SampleCount::N1 as _,
                levelCount: 1,
                format: refresh::TextureFormat::R8G8B8A8 as _,
                usageFlags: refresh::TextureUsageFlags::SAMPLER.bits(),
            },
        )
    };

    // define vertex buffer
    let vertices = vec![
        Vertex {
            pos: [-1.0, -1.0, 0.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [3.0, -1.0, 0.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [-1.0, 3.0, 0.0],
            uv: [0.0, 0.0],
        },
    ];

    let mut vbuf = refresh::Buffer::create(
        &device,
        &refresh::BufferCreateInfo {
            usage_flags: refresh::BufferUsageFlags::INDEX,
            size_in_bytes: (std::mem::size_of::<Vertex>() * vertices.len()) as _,
        },
    );

    // RenderPass
    let render_pass = {
        let color = refresh::ColorTargetDescription {
            format: refresh::TextureFormat::R8G8B8A8 as _,
            multisampleCount: refresh::SampleCount::N1 as _,
            loadOp: refresh::LoadOp::Clear as _,
            storeOp: refresh::StoreOp::Store as _,
        };
        let depth = refresh::DepthStencilTargetDescription {
            depthStencilFormat: refresh::TextureFormat::D32SfloatS8Uint as _,
            loadOp: refresh::LoadOp::Clear as _,
            storeOp: refresh::StoreOp::DontCare as _,
            stencilLoadOp: refresh::LoadOp::DontCare as _,
            stencilStoreOp: refresh::StoreOp::DontCare as _,
        };

        refresh::RenderPass::create(
            &device,
            &refresh::RenderPassCreateInfo {
                colorTargetCount: 1,
                colorTargetDescriptions: &color as *const _ as *mut _,
                depthTargetDescription: &depth as *const _ as *mut _,
            },
        )
    };

    let color_target_tex = refresh::Texture::create(
        &device,
        &refresh::TextureCreateInfo {
            width: win_size[0],
            height: win_size[1],
            depth: 0,
            isCube: false as _,
            sampleCount: refresh::SampleCount::N1 as _,
            levelCount: 1,
            format: refresh::TextureFormat::R8G8B8A8 as _,
            usageFlags: refresh::TextureUsageFlags::SAMPLER.bits(),
        },
    );

    let color_target_tex_slice = refresh::TextureSlice {
        texture: color_target_tex,
        rectangle: refresh::Rect {
            x: 0,
            y: 0,
            w: win_size[0] as _,
            h: win_size[1] as _,
        },
        depth: 0,
        layer: 0,
        level: 0,
    };

    // TODO: distinc target for color and depth/stencil?
    let render_target = refresh::RenderTarget::create(
        &device,
        &refresh::RenderTargetCreateInfo {
            texture_slice: &color_target_tex_slice as *const _ as *mut _,
            multi_sample_count: refresh::SampleCount::N1 as _,
        },
    );

    // frame buffer
    let fbuf = refresh::Framebuffer::create(
        &device,
        &refresh::FramebufferCreateInfo {
            renderPass: render_pass,
            // slice
            pColorTargets: &render_target as *const _ as *mut _,
            colorTargetCount: 1,
            pDepthStencilTarget: render_target,
            width: win_size[0],
            height: win_size[1],
        },
    );

    let render_target_blend_state = refresh::ColorTargetBlendState {
        colorWriteMask: refresh::ColorComponent::RGBA.bits(),
        ..Default::default()
    };

    // pipeline
    let vertex_binding = refresh::VertexBinding {
        binding: 0,
        stride: std::mem::size_of::<Vertex>() as _,
        inputRate: refresh::VertexInputRate::Vertex as _,
    };
    let vertex_attributes = vec![
        refresh::VertexAttribute {
            location: 0,
            binding: 0,
            format: refresh::VertexElementFormat::Vector3 as _,
            offset: 0,
        },
        refresh::VertexAttribute {
            location: 1,
            binding: 0,
            format: refresh::VertexElementFormat::Vector2 as _,
            offset: std::mem::size_of::<f32>() as u32 * 3,
        },
    ];

    let viewport = refresh::Viewport {
        x: 0.0,
        y: 0.0,
        w: win_size[0] as _,
        h: win_size[1] as _,
        minDepth: 0.0,
        maxDepth: 1.0,
    };

    let raymarch_pip = refresh::GraphicsPipeline::create(
        &device,
        &refresh::GraphicsPipelineCreateInfo {
            vertexShaderState: refresh::ShaderStageState {
                shaderModule: passthrough_shader,
                entryPointName: "main\0" as *const _ as _,
                uniformBufferSize: 0,
            },
            fragmentShaderState: refresh::ShaderStageState {
                shaderModule: raymarch_shader,
                entryPointName: "main\0" as *const _ as _,
                uniformBufferSize: 0,
            },
            vertexInputState: refresh::VertexInputState {
                vertexBindings: &vertex_binding as _,
                vertexBindingCount: 1,
                vertexAttributes: vertex_attributes.as_ptr(),
                vertexAttributeCount: vertex_attributes.len() as _,
            },
            primitiveType: refresh::PrimitiveType::TriangleList as _,
            viewportState: refresh::ViewportState {
                viewports: &viewport as *const _,
                viewportCount: 1,
                scissors: &render_area,
                scissorCount: 1,
            },
            rasterizerState: refresh::RasterizerState {
                cullMode: refresh::CullMode::Back as _,
                depthBiasClamp: 0.0,
                depthBiasConstantFactor: 0.0,
                depthBiasEnable: 0,
                depthBiasSlopeFactor: 0.0,
                depthClampEnable: 0,
                fillMode: refresh::FillMode::Fill as _,
                frontFace: refresh::FrontFace::Clockwise as _,
                lineWidth: 1.0,
            },
            multisampleState: refresh::MultisampleState {
                multisampleCount: refresh::SampleCount::N1 as _,
                sampleMask: 0, // FIXME: -1??
            },
            depthStencilState: refresh::DepthStencilState {
                depthTestEnable: false as _,
                depthWriteEnable: false as _,
                compareOp: refresh::CompareOp::Never as _,
                depthBoundsTestEnable: false as _,
                stencilTestEnable: false as _,
                frontStencilState: refresh::StencilOpState {
                    failOp: refresh::StencilOp::Zero as _,
                    passOp: refresh::StencilOp::Zero as _,
                    depthFailOp: refresh::StencilOp::Zero as _,
                    compareOp: refresh::CompareOp::Never as _,
                    compareMask: 0,
                    writeMask: 0,
                    reference: 0,
                },
                backStencilState: refresh::StencilOpState {
                    failOp: refresh::StencilOp::Zero as _,
                    passOp: refresh::StencilOp::Zero as _,
                    depthFailOp: refresh::StencilOp::Zero as _,
                    compareOp: refresh::CompareOp::Never as _,
                    compareMask: 0,
                    writeMask: 0,
                    reference: 0,
                },
                minDepthBounds: 0.0,
                maxDepthBounds: 1.0,
            },
            colorBlendState: refresh::ColorBlendState {
                logicOp: refresh::LogicOp::NoOp as _,
                blendStateCount: 1,
                blendStates: &render_target_blend_state,
                ..Default::default()
            },
            pipelineLayoutCreateInfo: refresh::GraphicsPipelineLayoutCreateInfo {
                vertexSamplerBindingCount: 0,
                fragmentSamplerBindingCount: 2,
            },
            renderPass: render_pass,
        },
    );

    let clear_color = refresh::Vec4 {
        x: 100.0 / 255.0,
        y: 149.0 / 255.0,
        z: 237.0 / 255.0,
        w: 255.0 / 255.0,
    };
    let depth_stencil_clear = refresh::DepthStencilValue {
        depth: 1.0,
        stencil: 0,
    };

    // sampling
    let sampler = refresh::Sampler::create(
        &device,
        &refresh::SamplerStateCreateInfo {
            addressModeU: refresh::SamplerAddressMode::Repeat as _,
            addressModeV: refresh::SamplerAddressMode::Repeat as _,
            addressModeW: refresh::SamplerAddressMode::Repeat as _,
            anisotropyEnable: false as _,
            borderColor: refresh::BorderColor::FloatOpaqueBlack as _,
            compareEnable: false as _,
            compareOp: refresh::CompareOp::Never as _,
            magFilter: refresh::Filter::Linear as _,
            maxAnisotropy: 0.0,
            maxLod: 1.0,
            minFilter: refresh::Filter::Linear as _,
            minLod: 1.0,
            mipLodBias: 1.0,
            mipmapMode: refresh::SamplerMipmapMode::Linear as _,
        },
    );

    let sample_textures = [wood_tex, noise_tex];
    let samplers = [sampler, sampler];

    let flip_rect = refresh::Rect {
        x: 0,
        y: win_size[1] as _,
        w: win_size[0] as _,
        h: -(win_size[1] as i32),
    };

    // run application
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = scx.event_pump()?;

    let mut instant = Instant::now();
    let mut tot_accum = Duration::default();
    let mut accum = Duration::default();
    let dt = Duration::from_secs_f64(1.0 / 60.0);

    let mut raymarch_uniforms = RaymarchUniforms {
        time: 0.0,
        pad: 0.0,
        res: [win_size[0] as _, win_size[1] as _],
    };

    'running: loop {
        // poll events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // tick
        let new_time = Instant::now();
        tot_accum += new_time - instant;
        accum += new_time - instant;
        instant = new_time;

        if accum < dt {
            continue;
        }

        accum -= dt;

        let is_fixed = false;
        let cbuf = device.acquire_command_buffer(is_fixed);

        device.begin_render_pass(
            cbuf,
            render_pass,
            fbuf,
            &render_area,
            Some(&[clear_color]),
            // TODO: is this right
            &depth_stencil_clear,
        );
        device.bind_graphics_pipeline(cbuf, raymarch_pip);
        // TODO: right?
        raymarch_uniforms.time = tot_accum.as_secs_f32();

        let frag_param_offset =
            device.push_fragment_shader_params(raymarch_pip, raymarch_uniforms.as_bytes());

        device.clear(
            cbuf,
            &render_area,
            refresh::ClearOptions::DEPTH | refresh::ClearOptions::STENCIL,
            &[],
            // TODO: is this right
            depth_stencil_clear,
        );
        device.end_render_pass(cbuf);

        // canvas.clear();
        // canvas.present();
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
