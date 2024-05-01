use std::time::Duration;

#[derive(Clone, Default, Debug)]
pub struct ProcessContext {
    // Time to wait before processing the device again
    pub next_process: Option<Duration>,
}

impl ProcessContext {
    pub fn reset(&mut self) {
        self.next_process = None;
    }

    pub fn request_process_in(&mut self, delay: Duration) {
        self.next_process = Some(delay);
    }

    pub fn request_process_in_ms(&mut self, delay: u64) {
        self.request_process_in(Duration::from_millis(delay));
    }

    pub fn request_process_in_s(&mut self, delay: u64) {
        self.request_process_in(Duration::from_secs(delay));
    }

    pub fn request_process_immediate(&mut self) {
        self.request_process_in_ms(0);
    }

    pub fn request_process_never(&mut self) {
        self.next_process = None;
    }
}
