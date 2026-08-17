use nessy_bin::run;

// #[async_std::main]
fn main() {
    println!("Booting Nessy");

    let _ = run();

    // let rom_path = if true {
    //     "./assets/nestest.nes"
    // } else {
    //     &args[2]
    // };

    // let graphics_system_options = GraphicsSystemOptions {
    //     height: settings.display.height,
    //     width: settings.display.width,
    //     title: settings.title,
    //     draw_colour: settings.display.clear_colour,
    // };
    //
    // let mut graphics_system = match GraphicsSystem::new(&graphics_system_options) {
    //     Ok(v) => v,
    //     Err(err) => panic!("Failed to init graphics system: {}.", err),
    // };
    //
    // let texture_creator = graphics_system.canvas.texture_creator();
    //
    // let mut texture =
    //     match texture_creator.create_texture_target(sdl2::pixels::PixelFormatEnum::RGB24, 32, 32) {
    //         Ok(v) => v,
    //         Err(err) => panic!("Failed to generate texture: {}.", err),
    //     };
    //
    // let mut renderer = Renderer::new(32, 32, 3);
    // let mut rng = rand::rng();
    //
    // // Load ROM file
    // let rom_data = match fs::read(rom_path) {
    //     Ok(data) => data,
    //     Err(reason) => panic!("Failed to read ROM: {}.", reason),
    // };
    //
    // let rom = Loader::load(&rom_data);
    //
    // if rom.is_err() {
    //     panic!("ROM load error: {}", rom.err().unwrap());
    // }
    //
    // let mut nes = NES::default();
    //
    // nes.cpu.load_program(rom.unwrap());
    // nes.cpu.reset();
    //
    // graphics_system.clear();
    // texture.update(None, &renderer.buffer, 32 * 3).unwrap();
    // graphics_system.canvas.copy(&texture, None, None).unwrap();
    // graphics_system.swap();
    //
    // nes.cpu.run_with_callback(|cpu| {
    //     // let input_flags = Input::handle(cpu, &mut graphics_system.event_pump.poll_iter());
    //
    //     // if input_flags.contains(InputFlags::Quit) {
    //     //     println!("Thanks for playing Nessy!");
    //     //     std::process::exit(0);
    //     // }
    //
    //     cpu.bus.write(MEMORY_ADDRESS_RNG, rng.random_range(1..16));
    //
    //     if renderer.handle(cpu) {
    //         graphics_system.clear();
    //         texture.update(None, &renderer.buffer, 32 * 3).unwrap();
    //         graphics_system.canvas.copy(&texture, None, None).unwrap();
    //         graphics_system.swap();
    //     }
    //
    //     ::std::thread::sleep(Duration::new(0, 70_000));
    // });
}
