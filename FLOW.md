# System Monitor - Project Flow and Architecture

## 1. Overview

The System Monitor is a real-time system monitoring application built in Rust using Dear ImGui for the user interface. It provides live information about various system metrics including CPU usage, memory utilization, process information, thermal state, and network activity.

## 2. Architecture

### 2.1 Core Components

```
system-monitor/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application loop and UI coordination
│   ├── system/              # System monitoring modules
│   │   ├── mod.rs          # System info aggregation
│   │   ├── cpu.rs          # CPU statistics
│   │   ├── memory.rs       # Memory statistics
│   │   ├── processes.rs    # Process monitoring
│   │   ├── thermal.rs      # Thermal monitoring
│   │   └── network.rs      # Network statistics
│   ├── ui/                 # User interface components
│   │   ├── mod.rs         # UI traits and common functionality
│   │   ├── windows/       # Window components
│   │   └── widgets/       # Reusable UI widgets
│   └── utils/             # Utility functions
```

## 3. Data Flow

### 3.1 Main Application Loop

1. **Initialization**
   - SDL2 and OpenGL context setup
   - Dear ImGui context initialization
   - System monitors initialization
   - UI components creation

2. **Event Loop**
   ```
   Main Loop
   ├── Poll SDL Events
   ├── Update System Data
   │   ├── CPU Stats
   │   ├── Memory Stats
   │   ├── Process List
   │   ├── Thermal Info
   │   └── Network Stats
   └── Render UI
       ├── System Overview
       ├── CPU Window
       ├── Memory Window
       ├── Process Window
       └── Network Window
   ```

### 3.2 System Monitoring

#### CPU Monitoring (`system/cpu.rs`)
- Uses `procfs` to read `/proc/cpuinfo` and `/proc/stat`
- Tracks per-core usage and overall CPU utilization
- Maintains history for graphing

#### Memory Monitoring (`system/memory.rs`)
- Reads `/proc/meminfo` for memory statistics
- Tracks RAM and swap usage
- Calculates memory pressure and available memory

#### Process Monitoring (`system/processes.rs`)
- Scans `/proc/[pid]` directories
- Maintains a HashMap of active processes
- Tracks per-process CPU and memory usage

#### Network Monitoring (`system/network.rs`)
- Monitors network interfaces through `/proc/net`
- Tracks TCP/UDP connections
- Calculates bandwidth usage

#### Thermal Monitoring (`system/thermal.rs`)
- Reads thermal zones from `/sys/class/thermal`
- Monitors CPU and system temperatures
- Tracks thermal throttling events

## 4. User Interface

### 4.1 Window Components

Each window component (`ui/windows/`) implements the `Window` trait:
```rust
pub trait Window {
    fn render(&mut self, ui: &Ui);
}
```

#### System Overview Window
- Displays system summary
- Shows OS information
- Lists critical metrics

#### CPU Window
- Real-time CPU usage graphs
- Per-core utilization
- Frequency and temperature data

#### Memory Window
- RAM and swap usage graphs
- Memory allocation breakdown
- Cache and buffer statistics

#### Process Window
- Sortable process table
- Resource usage per process
- Process control options

#### Network Window
- Network interface statistics
- Connection tables
- Bandwidth graphs

### 4.2 Widgets (`ui/widgets/`)

Reusable UI components implementing the `Widget` trait:
```rust
pub trait Widget {
    fn render(&mut self, ui: &Ui);
}
```

#### LineGraph
- Time-series data visualization
- Auto-scaling
- Configurable history length

#### Table
- Sortable columns
- Customizable headers
- Row selection support

## 5. Data Flow Example

Example of CPU usage monitoring flow:

```
1. System Loop
   └── CpuStats::update()
       ├── Read /proc/stat
       ├── Calculate CPU usage
       └── Update history

2. UI Rendering
   └── CpuWindow::render()
       ├── Display current usage
       ├── Update LineGraph
       └── Show per-core stats
```

## 6. Error Handling

- Uses `anyhow` for error propagation
- Graceful degradation on partial failures
- Logging via `log` and `env_logger`

## 7. Performance Considerations

- Asynchronous data collection
- Efficient data structures (HashMap for processes)
- Optimized rendering with Dear ImGui
- Minimal system impact through throttled updates

## 8. Future Enhancements

1. GPU Monitoring
2. Disk I/O Statistics
3. Power Management
4. System Alerts
5. Data Export
6. Custom Themes
7. Plugin System 