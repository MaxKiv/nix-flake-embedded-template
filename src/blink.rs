use core::sync::atomic::AtomicU32;

use crate::info;
use embassy_time::Timer;
use esp_hal::gpio::{ErasedPin, Level, Output};

pub const LED_BLINK_SPEEDS: [u64; 3] = [1, 2, 4];

pub static LED_IDX: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
pub async fn blink_task(led: ErasedPin) {
    // configure pin as Output
    let mut led = Output::new(led, Level::High);

    loop {
        info!("blinking led...");
        blink_heartbeat(&mut led).await;
    }
}

/// toggles OutputPin in Heartbeat pattern
async fn blink_heartbeat(led: &mut Output<'_>) {
    let idx = LED_IDX.load(core::sync::atomic::Ordering::Relaxed) as usize;
    Timer::after_millis(500 / LED_BLINK_SPEEDS[idx]).await;
    led.set_high();
    Timer::after_millis(100 / LED_BLINK_SPEEDS[idx]).await;
    led.set_low();

    Timer::after_millis(200 / LED_BLINK_SPEEDS[idx]).await;
    led.set_high();
    Timer::after_millis(100 / LED_BLINK_SPEEDS[idx]).await;
    led.set_low();
    Timer::after_millis(100 / LED_BLINK_SPEEDS[idx]).await;
}
