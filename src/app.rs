use anyhow::{Result, Context};
use sdl2::event::Event;
use sdl2::video::Window;
use imgui::{Context as ImguiContext, Ui};
use imgui_sdl2::ImguiSdl2;
use crate::ui::windows::{
    system::SystemWindow,
    cpu::CpuWindow,
    memory::MemoryWindow,
    processes::ProcessWindow,
    network::NetworkWindow,
};
use crate::ui::Window as WindowTrait;

pub struct App {
    sdl_context: sdl2::Sdl,
    window: Window,
    imgui: ImguiContext,
    imgui_sdl2: ImguiSdl2,
    system_window: SystemWindow,
    cpu_window: CpuWindow,
    memory_window: MemoryWindow,
    process_window: ProcessWindow,
    network_window: NetworkWindow,
}

impl App {
    pub fn new() -> Result<Self> {
        // Initialize SDL2
        let sdl_context = sdl2::init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize SDL2: {}", e))?;
            
        let video_subsystem = sdl_context.video()
            .map_err(|e| anyhow::anyhow!("Failed to initialize video subsystem: {}", e))?;

        // Create window
        let window = video_subsystem
            .window("System Monitor", 1280, 720)
            .position_centered()
            .opengl()
            .build()
            .context("Failed to create window")?;

        // Initialize ImGui
        let mut imgui = ImguiContext::create();
        imgui.set_ini_filename(None);

        // Initialize imgui-sdl2
        let imgui_sdl2 = ImguiSdl2::new(&mut imgui, &window);

        Ok(Self {
            sdl_context,
            window,
            imgui,
            imgui_sdl2,
            system_window: SystemWindow::new(),
            cpu_window: CpuWindow::new(),
            memory_window: MemoryWindow::new(),
            process_window: ProcessWindow::new(),
            network_window: NetworkWindow::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut event_pump = self.sdl_context.event_pump()
            .map_err(|e| anyhow::anyhow!("Failed to create event pump: {}", e))?;

        let mut running = true;

        while running {
            for event in event_pump.poll_iter() {
                self.imgui_sdl2.handle_event(&mut self.imgui, &event);
                if let Event::Quit { .. } = event {
                    running = false;
                }
            }

            self.imgui_sdl2.prepare_frame(
                self.imgui.io_mut(),
                &self.window,
                &event_pump.mouse_state(),
            );

            let ui = self.imgui.frame();
            {
                self.system_window.render(&ui);
                self.cpu_window.render(&ui);
                self.memory_window.render(&ui);
                self.process_window.render(&ui);
                self.network_window.render(&ui);
            }

            // TODO: Render frame
        }

        Ok(())
    }
} 