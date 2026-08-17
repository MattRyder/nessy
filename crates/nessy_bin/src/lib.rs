use std::{env, fs};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::UnwrapThrowExt;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use winit::event_loop::EventLoop;
use winit::window::WindowAttributes;

use crate::settings::Settings;

pub mod rendering;
pub mod settings;
pub mod windowing;

#[cfg(not(target_arch = "wasm32"))]
fn load_settings() -> anyhow::Result<Settings, String> {
    let args: Vec<String> = env::args().collect();
    let settings_path = if true {
        "./assets/settings.toml"
    } else {
        &args[1]
    };

    let settings_file_content = match fs::read_to_string(settings_path) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };

    match toml::from_str(&settings_file_content) {
        Ok(settings) => Ok(settings),
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(target_arch = "wasm32")]
fn load_settings() -> anyhow::Result<Settings, String> {
    let settings_path = "/assets/settings.toml";

    // let window = web_sys::window().ok_or_else(|| String::from("window is unavailable"))?;
    //
    // let response = JsFuture::from(window.fetch_with_str(&settings_path)).await;
    //
    // if !response.is_ok() {
    //     return Err(format!("failed to load '{}'", settings_path));
    // }
    //
    // let buffer_array = Uint8Array::new(&response.unwrap());
    // let str = match String::from_utf8(buffer_array.to_vec()) {
    //     Ok(s) => s,
    //     Err(reason) => return Err(reason.to_string()),
    // };

    let demo = r#"
   # Some settings for the nessy_bin
title = "NES Emulator"

[display]
width = 600
height = 600
clear_colour = [1, 2, 3] 
    "#;

    let settings: Settings = match toml::from_str(demo) {
        Ok(settings) => Ok(settings),
        Err(error) => Err(error.to_string()),
    }?;

    Ok(settings)
}

pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;

    let settings = match load_settings() {
        Ok(it) => it,
        Err(err_str) => {
            // come on, bro
            panic!("Settings not loaded: {}", err_str);
        }
    };

    let window_attributes = WindowAttributes::default()
        .with_title(&settings.title)
        .with_inner_size(winit::dpi::LogicalSize::new(
            settings.display.width,
            settings.display.height,
        ));

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app = crate::windowing::app::App::new(Some(window_attributes));
        event_loop.run_app(&mut app)?;
    }

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::EventLoopExtWebSys;

        let app = crate::windowing::app::App::new(Some(window_attributes), &event_loop);
        event_loop.spawn_app(app);
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    run().unwrap_throw();

    Ok(())
}
