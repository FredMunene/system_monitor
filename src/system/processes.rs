use anyhow::Result;
use procfs::process::Process;
use std::collections::HashMap;

pub struct ProcessStats {
    processes: HashMap<i32, Process>,
}

impl ProcessStats {
    pub fn new() -> Result<Self> {
        Ok(Self {
            processes: HashMap::new(),
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.processes.clear();
        for process in procfs::process::all_processes()? {
            if let Ok(process) = process {
                self.processes.insert(process.pid(), process);
            }
        }
        Ok(())
    }
} 