# nessy

A NES (Nintendo Entertainment System) emulator written in Rust.

## Build

Clone the repository and build the emulator:

```bash
git clone https://github.com/MattRyder/nessy.git
cd nessy
cargo build --release
```

## Run

To run the emulator with your cool ROM:

```bash
cargo run --release -- path/to/your/rom.nes
```

## Testing

There are unit and integration tests covering CPU instructions and memory behavior.

To run tests:

```bash
cargo test
```

## Contributing

- Fork the repository.
- Create a feature branch (git checkout -b feature/your-feature).
- Write tests for your changes.
- Submit a pull request.

Please follow idiomatic Rust patterns and include documentation for public APIs.

## License

This project is open source under the terms of the AGPLv3 License.

## Bibliography / Acknowledgements

- [Writing a NES emulator in Rust](https://bugzmanov.github.io/nes_ebook)
- [Obelisk 6502 Guide](https://www.nesdev.org/obelisk-6502-guide/)
