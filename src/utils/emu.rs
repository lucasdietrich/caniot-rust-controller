#[cfg(feature = "emu-delay")]
use std::time::Duration;

#[cfg(feature = "emu-delay")]
const EMU_DELAY_MS: u64 = 100;

pub async fn emulated_delay_async() {
    #[cfg(feature = "emu-delay")]
    {
        warn!("Asynchronous emulated delay introduced: {}ms", EMU_DELAY_MS);
        tokio::time::sleep(Duration::from_millis(EMU_DELAY_MS)).await;
    }
}

#[allow(dead_code)]
pub fn emulated_delay() {
    #[cfg(feature = "emu-delay")]
    {
        warn!("Emulated delay introduced: {}ms", EMU_DELAY_MS);
        std::thread::sleep(Duration::from_millis(EMU_DELAY_MS));
    }
}
