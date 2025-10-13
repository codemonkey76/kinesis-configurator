# Kinesis Advantage 360 Configurator

A native Linux GUI application for configuring the Kinesis Advantage 360 keyboard. Since the official SmartSet app doesn't work on Linux, this tool provides a graphical interface to edit the keyboard's configuration files directly on the V-Drive.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

## Features

- 🎨 **Visual Keyboard Layout** - Interactive split keyboard visualization matching the actual Kinesis 360 layout
- 🔄 **9 Layout Support** - Switch between and manage all 9 keyboard layouts
- 📋 **Layout Copying** - Easily duplicate layouts to speed up configuration
- 🎯 **Key Remapping** - Visual interface for remapping keys (coming soon)
- 💾 **Direct V-Drive Access** - Reads and writes configuration files directly to the keyboard
- 🐧 **Native Linux** - Built with GTK4 and Rust for a fast, native experience

## Screenshots

*Screenshots coming soon*

## Installation

### Prerequisites

- Rust 1.70 or later
- GTK4 development libraries
- libadwaita development libraries
- cairo development libraries

#### Arch Linux

```bash
sudo pacman -S gtk4 libadwaita cairo
```

#### Ubuntu/Debian

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev libcairo2-dev
```

#### Fedora

```bash
sudo dnf install gtk4-devel libadwaita-devel cairo-devel
```

### Building from Source

```bash
git clone https://github.com/codemonkey76/kinesis-configurator.git
cd kinesis-configurator
cargo build --release
```

The binary will be at `target/release/kinesis-configurator`

### Running

```bash
cargo run
```

Or install globally:

```bash
cargo install --path .
kinesis-configurator
```

## Usage

### 1. Enable V-Drive Mode

Press **SmartSet + Hk3** on your Kinesis Advantage 360 keyboard to enable V-Drive mode. The keyboard will appear as a USB storage device.

### 2. Mount the V-Drive (Linux)

The V-Drive typically doesn't auto-mount on Linux. You can either:

**Option A: Manual mount**

```bash
# Find the device
lsblk

# Mount it (replace /dev/sdX1 with your device)
sudo mkdir -p /mnt/kinesis
sudo mount /dev/sdX1 /mnt/kinesis
```

**Option B: Use your file manager**

Most Linux file managers will show the device and allow you to mount it with a click.

### 3. Detect and Load Configuration

1. Click **Detect Keyboard** to find the mounted V-Drive
2. Click **Load Config** to load your current keyboard configuration
3. Switch between layouts using the numbered buttons (1-9)
4. Make changes to your configuration
5. Click **Save Config** to write changes back to the keyboard

### 4. Exit V-Drive Mode

Press **SmartSet + Hk3** again to return to normal keyboard operation.

## Project Structure

```
kinesis-configurator/
├── src/
│   ├── main.rs              # Application entry point
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── main_window.rs   # Main application window
│   │   └── keyboard_view.rs # Visual keyboard component
│   ├── config/
│   │   ├── mod.rs           # Configuration data structures
│   │   ├── parser.rs        # Config file parser
│   │   └── generator.rs     # Config file generator
│   └── vdrive/
│       └── mod.rs           # V-Drive detection and I/O
├── Cargo.toml
└── README.md
```

## Roadmap

- [x] V-Drive detection
- [x] Visual keyboard layout
- [x] Layout switching (1-9)
- [x] Layout copying
- [ ] Key remapping interface
- [ ] Macro editor
- [ ] Lighting configuration
- [ ] Import/export configurations
- [ ] Undo/redo support
- [ ] Configuration presets

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development

```bash
# Run with debug output
RUST_LOG=debug cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Technical Details

- **Language**: Rust
- **GUI Framework**: GTK4 with libadwaita
- **Architecture**: Relm4 (Elm-inspired framework for GTK apps)
- **Drawing**: Cairo for keyboard visualization

## Known Issues

- V-Drive auto-mounting doesn't work on some Linux distributions
- Configuration file format parsing is still in development
- Key remapping UI not yet implemented

## FAQ

**Q: Why doesn't the V-Drive auto-mount?**  
A: This is a Linux limitation with certain USB storage devices. Manual mounting is required on most systems.

**Q: Will this work with the original Kinesis Advantage (non-360)?**  
A: Not currently, as the configuration format is different. PRs welcome!

**Q: Can I brick my keyboard with this?**  
A: Unlikely, but always keep a backup of your configuration files. The keyboard's firmware is separate from the configuration.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Built with [Relm4](https://relm4.org/)
- Inspired by the need for Linux support in the Kinesis community
- Thanks to all contributors and testers

## Support

If you encounter issues or have questions:

- Open an issue on GitHub
- Check existing issues for solutions
- Join the discussion in the Kinesis community forums

---

**Note**: This is an unofficial tool and is not affiliated with or endorsed by Kinesis Corporation.
