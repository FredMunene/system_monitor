mod app;
mod system;
mod ui;
mod utils;

use anyhow::Result;
use log::info;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting System Monitor...");

    // Create and run the application
    let mut app = app::App::new()?;
    app.run()?;

    Ok(())
} 