# A Virtual Silicon Chip (AVSC)

A Virtual Silicon Chip (AVSC) is a simulation platform that models the key components and behaviors of a digital integrated circuit (IC) across multiple abstraction layers. This includes logic, communication, memory, and user interaction, with interactive 2D/3D visualization using Rust and Bevy.

## Features
- Multi-layer abstraction: Logic gates, RTL, buses, memory, physical layout, and more
- Interactive visualization (zoom, pan, rotate, inspect)
- Modular and extensible architecture
- Built with Rust and Bevy game engine

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended)
- A desktop or laptop (Windows, macOS, or Linux)

### Setup
1. **Clone or download this repository**
2. **Navigate to the project directory:**
   ```bash
   cd a_virtual_silicon_chip
   ```
3. **Build and run the project:**
   ```bash
   cargo run
   ```

### Controls
- Use your mouse to pan, zoom, and rotate the view (to be implemented)
- Click on elements to inspect properties (to be implemented)

## Roadmap
- [x] Project scaffolding and Bevy setup
- [ ] Logic gate simulation and visualization
- [ ] RTL and system-level modeling
- [ ] Bus and memory layers
- [ ] Physical and 3D layout
- [ ] Advanced UI and inspection tools

## License
MIT