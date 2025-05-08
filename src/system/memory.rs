use anyhow::Result;
use procfs::Meminfo;

pub struct MemoryStats {
    info: Meminfo,
}

impl MemoryStats {
    pub fn new() -> Result<Self> {
        Ok(Self {
            info: Meminfo::new()?,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.info = Meminfo::new()?;
        Ok(())
    }
} 