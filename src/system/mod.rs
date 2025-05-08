pub mod cpu;
pub mod memory;
pub mod processes;
pub mod thermal;
pub mod network;

use anyhow::Result;

pub struct SystemInfo {
    // TODO: Add system information fields
}

impl SystemInfo {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn update(&mut self) -> Result<()> {
        Ok(())
    }
} 