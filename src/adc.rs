use crate::info;
use embassy_time::Timer;
use esp_hal::{
    analog::adc::{Adc, AdcPin},
    gpio::GpioPin,
    peripherals::ADC1,
    prelude::nb::block,
};

#[embassy_executor::task]
pub async fn adc_task(mut adc1: Adc<'static, ADC1>, mut adc_pin: AdcPin<GpioPin<0>, ADC1>) {
    loop {
        Timer::after_millis(500).await;

        let Ok(adc_val) = block!(adc1.read_oneshot(&mut adc_pin)) else {
            info!("uv_ref_val error");
            break;
        };

        info!("ADC: val {}", adc_val);
    }
}
