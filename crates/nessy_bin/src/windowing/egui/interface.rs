use std::sync::Arc;

use egui::{Context, FullOutput, RawInput, TexturesDelta};
use egui_wgpu::{Renderer, RendererOptions, ScreenDescriptor};
use egui_winit::State;
use wgpu::{LoadOp, Operations, RenderPassColorAttachment, StoreOp};
use winit::window::Theme;

use crate::{
    rendering::triangle_renderer::TriangleRenderer,
    windowing::{
        egui::windows::{EGuiWindow, root_window::RootWindow},
        state::GpuState,
    },
};

pub struct EGuiInterface {
    context: Context,
    pub winit_state: State,
    renderer: Renderer,
    // triangle_renderer: TriangleRenderer,
}

impl EGuiInterface {
    pub fn new(
        window: &Arc<winit::window::Window>,
        gpu_state: &GpuState,
        target_format: wgpu::TextureFormat,
    ) -> Self {
        let context = Context::default();

        let renderer_options = RendererOptions::default();
        let mut renderer = Renderer::new(&gpu_state.device, target_format, renderer_options);

        let winit_state = egui_winit::State::new(
            context.clone(),
            context.viewport_id(),
            &window,
            Some(window.scale_factor() as _),
            Some(Theme::Dark),
            None,
        );

        TriangleRenderer::new(gpu_state, target_format, &mut renderer);

        Self {
            context,
            renderer,
            winit_state,
            // triangle_renderer,
        }
    }

    /// Generates the output description for the renderer to use to output.
    /// Here's where you'll build the UI, basically, but *not* render it. See below.
    pub fn prepare(&self, raw_input: RawInput) -> FullOutput {
        self.context.run_ui(raw_input, |ctx| {
            RootWindow::render(ctx);
        })
    }

    pub fn render(
        &mut self,
        gpu: &GpuState,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        output: FullOutput,
        width: u32,
        height: u32,
        delta_time: std::time::Duration,
    ) {
        // convert the egui shapes to gpu primitives
        let paint_jobs = self
            .context
            .tessellate(output.shapes, output.pixels_per_point);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: output.pixels_per_point,
        };

        self.render_frame(
            &gpu.device,
            &gpu.queue,
            target,
            encoder,
            screen_descriptor,
            paint_jobs,
            output.textures_delta,
            delta_time,
        );
    }

    fn render_frame(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        screen_descriptor: ScreenDescriptor,
        paint_jobs: Vec<egui::epaint::ClippedPrimitive>,
        mut textures_delta: TexturesDelta,
        _delta_time: std::time::Duration,
    ) {
        // let delta_time = delta_time.as_secs_f32();

        // upload the egui texture changes
        for (id, image_delta) in textures_delta.set.drain(..) {
            // for image in images {} (used in 0.36)
            self.renderer
                .update_texture(device, queue, id, &image_delta);
        }

        // free up texture resources no longer required
        for id in textures_delta.free.drain(..) {
            self.renderer.free_texture(&id);
        }

        // setup the vertex buffers
        self.renderer.update_buffers(
            device,
            queue,
            encoder,
            paint_jobs.as_slice(),
            &screen_descriptor,
        );

        // the bloody render pass here
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: target,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        self.renderer.render(
            &mut render_pass.forget_lifetime(),
            paint_jobs.as_slice(),
            &screen_descriptor,
        );
    }
}
