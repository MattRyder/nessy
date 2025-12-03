use sdl2::{
    EventPump,
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

pub struct GraphicsSystem {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
}

pub struct GraphicsSystemOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub draw_colour: [u8; 3],
}

impl GraphicsSystem {
    pub fn new(options: &GraphicsSystemOptions) -> Result<GraphicsSystem, String> {
        let sdl_context = sdl2::init()?;

        let video_subsystem = sdl_context.video()?;

        let window = match video_subsystem
            .window(&options.title, options.width, options.height)
            .position_centered()
            .build()
        {
            Ok(wnd) => wnd,
            Err(window_build_error) => {
                return Err(window_build_error.to_string());
            }
        };

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(
            options.draw_colour[0],
            options.draw_colour[1],
            options.draw_colour[2],
        ));

        canvas.set_scale(10.0, 10.0)?;

        let event_pump = sdl_context.event_pump()?;

        let texture_creator = canvas.texture_creator();

        Ok(GraphicsSystem {
            canvas,
            event_pump,
            texture_creator,
        })
    }

    pub fn create_texture(
        &mut self,
        format: PixelFormatEnum,
        width: u32,
        height: u32,
    ) -> Result<Texture<'_>, String> {
        self.texture_creator
            // .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
            .create_texture_target(format, width, height)
            .map_err(|e| e.to_string())
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn swap(&mut self) {
        self.canvas.present();
    }
}
