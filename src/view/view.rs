use tokio::sync::broadcast::Receiver;

use crate::model::event::Event;
use crate::model::traits::AsyncReceiver;
use crate::model::{cpu_info::CpuInfo, memory_info::MemoryInfo};
use crate::view::dial::Dial;
use crate::view::layout::Layout;
pub struct View {}

impl View {
    pub fn new(receiver: Receiver<Event>) -> Self {
        Self {}
    }

    pub fn render(&self, memory_info: MemoryInfo, cpu_info: CpuInfo) {
        print!("\x1B[2J\x1B[H");
        let used_memory_in_megabyte = memory_info.used_memory / 1024;
        let total_memory_in_megabyte = memory_info.total_memory / 1024;
        let percentage = match total_memory_in_megabyte {
            0 => 0.00,
            _ => used_memory_in_megabyte as f64 / total_memory_in_megabyte as f64 * 100.00,
        };

        println!(
            "Memory used: {} MB / {} MB ({:.2}%)",
            used_memory_in_megabyte, total_memory_in_megabyte, percentage
        );

        println!("CPU Usage: {:.2}%", cpu_info.percentage);
        let diameter = 20;

        let canvas = Dial::render_dial(&(cpu_info.percentage as f32), &diameter);
        let canvas2 = Dial::render_dial(&(percentage as f32), &diameter);
        let mut vec_of_vecs = Vec::new();
        vec_of_vecs.push(canvas);
        vec_of_vecs.push(canvas2);

        Layout::render_together(vec_of_vecs);
    }
}

impl AsyncReceiver for View {
    async fn receive(&mut self, receiver: &mut Receiver<Event>) {
        loop {
            match receiver.recv().await {
                Ok(event) => self.render(event.memory_info, event.cpu_info),
                Err(_) => println!("Error fetching event"),
            }
        }
    }
}
