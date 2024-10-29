use crate::info;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{ErasedPin, Input};

use crate::blink::{LED_BLINK_SPEEDS, LED_IDX};

const DEBOUNCE_DURATION: Duration = Duration::from_millis(50);

#[embassy_executor::task]
pub async fn button_task(button: ErasedPin) {
    let mut button = Input::new(button, esp_hal::gpio::Pull::Up);

    loop {
        if debounce_buttonpress(&mut button).await {
            let idx = LED_IDX.load(core::sync::atomic::Ordering::Relaxed);
            let idx = (idx + 1) % LED_BLINK_SPEEDS.len() as u32;
            LED_IDX.store(idx, core::sync::atomic::Ordering::Relaxed);
            info!("button pressed, led speed increased");
        }
    }
}

async fn debounce_buttonpress(button: &mut Input<'_>) -> bool {
    button.wait_for_low().await;
    Timer::after(DEBOUNCE_DURATION).await;
    if button.is_low() {
        button.wait_for_high().await;
        return true;
    }
    false
}
