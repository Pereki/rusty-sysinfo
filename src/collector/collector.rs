use crate::model::{cpu_info::CpuInfo, event::Event, memory_info::MemoryInfo, traits::AsyncSender};
use sysinfo::System;
use tokio::sync::broadcast::Sender;

pub struct Collector {
    system: System,
    sender: Sender<Event>,
}

impl Collector {
    pub fn new(sender: Sender<Event>) -> Self {
        let mut system = System::new();
        system.refresh_all();

        Collector { system, sender }
    }

    fn collect_memory(&self) -> MemoryInfo {
        MemoryInfo::new(self.system.used_memory(), self.system.total_memory())
    }

    fn collect_cpu(&self) -> CpuInfo {
        CpuInfo::new(self.system.global_cpu_usage())
    }

    fn refresh_all(&mut self) {
        self.system.refresh_all();
    }
}

impl AsyncSender for Collector {
    async fn run(&mut self) {
        println!("Starting collecting data.....");
        let one_second = tokio::time::Duration::from_secs(1);
        loop {
            println!("refreshing.");
            self.refresh_all();
            let memory_info = self.collect_memory();
            let cpu_info = self.collect_cpu();
            let result = self.sender.send(Event::new(
                crate::model::event::EventType::UPDATE,
                memory_info,
                cpu_info,
            ));

            match result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to send event: {}", e);
                }
            };

            tokio::time::sleep(one_second).await;
        }
    }
}
