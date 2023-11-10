use std::time::Instant;

pub struct StopWatch {
    start_time: Option<Instant>,
}

impl StopWatch {
    pub fn new() -> Self {
        Self { start_time: None }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&self, task_name: &str) {
        match self.start_time {
            Some(start) => {
                let duration = start.elapsed();
                if duration.as_millis() == 0 {
                    println!("Task '{}' took {} µs", task_name, duration.as_micros());
                } else {
                    println!("Task '{}' took {} ms", task_name, duration.as_millis());
                }
            }
            None => {
                println!("Stopwatch not started");
            }
        }
    }
}
