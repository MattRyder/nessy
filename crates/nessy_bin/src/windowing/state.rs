use std::sync::Arc;
use web_time::Instant;
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::{Window, WindowId},
};

use crate::windowing::egui::interface::EGuiInterface;

pub struct GpuState {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

// This will store the state of our window
pub struct AppState {
    pub window: Arc<Window>,
    pub gpu: GpuState,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    pub last_render_time: Option<Instant>,

    pub egui_interface: Option<EGuiInterface>,
}

impl AppState {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                // apply_limit_buckets: true,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            // color_space: wgpu::SurfaceColorSpace::Auto,
        };

        let gpu_state: GpuState = GpuState {
            surface,
            device,
            queue,
        };

        let egui_interface = Some(EGuiInterface::new(&window, &gpu_state, surface_format));

        Ok(AppState {
            window,
            gpu: gpu_state,
            config,
            is_surface_configured: false,
            last_render_time: Some(Instant::now()),
            egui_interface,
            // clear_colour: wgpu::Color {
            //     r: 0.1,
            //     g: 0.2,
            //     b: 0.3,
            //     a: 1.0,
            // },
        })
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        }
    }

    pub fn update(&mut self, _window_id: WindowId, _event: WindowEvent) -> anyhow::Result<()> {
        // self.imgui_state
        //     .handle_event(&self.window, window_id, event)?;
        //
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            #[cfg(target_arch = "wasm32")]
            {
                const WEBGL_MAX_DIMENSION_PX: u32 = 2048;
                self.config.width = width.min(WEBGL_MAX_DIMENSION_PX);
                self.config.height = height.min(WEBGL_MAX_DIMENSION_PX);
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                self.config.width = width;
                self.config.height = height;
            }

            self.gpu.surface.configure(&self.gpu.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&mut self, delta_time: std::time::Duration) -> anyhow::Result<()> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.gpu.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // This frame can be skipped.
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.gpu.surface.configure(&self.gpu.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // The surface has completely disppeared. Would need rebuilding...
                anyhow::bail!("The device has been lost.");
            }
        };

        let texture_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        if let Some(egui_interface) = &mut self.egui_interface {
            // Draw the egui state:
            let egui_raw_input = egui_interface.winit_state.take_egui_input(&self.window);

            let egui_draw_data = egui_interface.prepare(egui_raw_input);

            egui_interface
                .winit_state
                .handle_platform_output(&self.window, egui_draw_data.platform_output.clone());

            egui_interface.render(
                &self.gpu,
                &mut encoder,
                &texture_view,
                egui_draw_data,
                self.config.width,
                self.config.height,
                delta_time,
            );
        }

        // submit will accept anything that implements IntoIter
        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        // self.queue.present(output);

        Ok(())
    }
}
