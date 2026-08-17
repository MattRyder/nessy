use crate::rendering::render_resource::RenderResource;

pub struct TriangleCallback {
    pub angle: f32,
}

impl egui_wgpu::CallbackTrait for TriangleCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some::<&RenderResource>(resources) = callback_resources.get() {
            resources.prepare(device, queue);
        }

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        if let Some::<&RenderResource>(resources) = callback_resources.get() {
            resources.paint(render_pass);
        }
    }
}
