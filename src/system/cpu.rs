use anyhow::Result;
use procfs::CpuInfo;

pub struct CpuStats {
    info: CpuInfo,
}

impl CpuStats {
    pub fn new() -> Result<Self> {
        Ok(Self {
            info: CpuInfo::new()?,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.info = CpuInfo::new()?;
        Ok(())
    }
} 