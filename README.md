🖥️ System Monitor (Rust Edition)

A desktop system monitor built in Rust using Dear ImGui for the user interface. This application displays live information on CPU, memory, processes, thermal state, fan performance, and network activity by leveraging the Linux /proc filesystem.

## 📌 Project Objectives

This project demonstrates:
- Proficiency in Rust systems programming
- Integration with external GUI libraries using FFI bindings
- Use of Dear ImGui to build a responsive, interactive GUI
- Hands-on experience with Linux system monitoring via /proc

## 🧱 Project Structure

```
system-monitor/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application logic
│   ├── ui/
│   │   ├── mod.rs          # UI module declarations
│   │   ├── windows/        # Individual window components
│   │   │   ├── mod.rs
│   │   │   ├── system.rs   # System overview window
│   │   │   ├── cpu.rs      # CPU monitoring window
│   │   │   ├── memory.rs   # Memory monitoring window
│   │   │   ├── processes.rs # Process list window
│   │   │   └── network.rs  # Network monitoring window
│   │   └── widgets/        # Reusable UI components
│   │       ├── mod.rs
│   │       ├── graphs.rs   # Graph rendering components
│   │       └── tables.rs   # Table rendering components
│   ├── system/
│   │   ├── mod.rs          # System monitoring module
│   │   ├── cpu.rs          # CPU monitoring logic
│   │   ├── memory.rs       # Memory monitoring logic
│   │   ├── processes.rs    # Process monitoring logic
│   │   ├── thermal.rs      # Thermal monitoring logic
│   │   └── network.rs      # Network monitoring logic
│   └── utils/
│       ├── mod.rs          # Utility functions
│       └── formatting.rs   # Data formatting utilities
├── assets/                 # Static assets
│   └── fonts/             # Custom fonts
├── tests/                 # Integration tests
├── examples/              # Example usage
├── Cargo.toml            # Project dependencies
├── Cargo.lock            # Dependency lock file
├── .gitignore           # Git ignore rules
├── LICENSE              # MIT License
└── README.md           # Project documentation
```

## ⚙️ Planned Features

### 🧠 System Overview
- OS type, hostname, current user
- Task overview: running, sleeping, zombie, etc.
- CPU model and real-time core usage

### 🧮 CPU / Fan / Thermal Monitoring
- Live graphs:
  - CPU usage
  - Fan speed
  - CPU temperature
- Customizable:
  - FPS and Y-scale sliders
  - Pause/resume button
  - Overlay percentages

### 🧵 Memory & Processes
- RAM, SWAP, and Disk usage visualization
- Process table with:
  - PID
  - Name
  - State
  - CPU %
  - Memory %
- Search/filter support and multi-selection

### 🌐 Network Monitoring
- Active IPv4 interfaces detection (e.g. lo, wlan0, etc.)
- RX and TX metrics:
  - Bytes, packets, errors, drops, collisions
- Dynamic usage bars with human-readable scaling

## 🛠️ Getting Started

### ✅ Prerequisites
- Linux-based OS
- Rust & Cargo (rustup recommended)
- SDL2 & OpenGL libraries

### 🔧 Installation

1. Install system dependencies:
```bash
sudo apt update
sudo apt install libsdl2-dev libgl1-mesa-dev
```

2. Clone the repository:
```bash
git clone https://github.com/fredmunene/system-monitor.git
cd system-monitor
```

3. Build and run:
```bash
cargo run
```

## 📚 Learning Outcomes

By building this project, you will:
- Work with FFI bindings to native libraries in Rust
- Understand how to build immediate-mode GUIs with Dear ImGui
- Monitor Linux system resources in real-time using /proc
- Practice modular design and multi-threading in Rust

## 🧪 Planned Libraries & Tools
- imgui-rs
- sdl2
- procfs
- OpenGL and SDL2 as Dear ImGui backends

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 👨‍💻 Author

Fred Gitonga
- GitHub: [@fredmunene](https://github.com/fredmunene)

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.