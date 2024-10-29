use crate::info;
use embassy_time::Timer;
use epd_waveshare::{
    epd1in54b,
    prelude::{WaveshareDisplay, WaveshareThreeColorDisplay},
};
use esp_backtrace as _;
use esp_hal::{
    gpio::{ErasedPin, Input, Level, Output, Pull},
    peripherals::SPI2,
    spi::{master::Spi, SpiMode},
};
use fugit::HertzU32;

#[embassy_executor::task]
pub async fn epd_task(
    spi2: SPI2,
    sclk: ErasedPin,
    mosi: ErasedPin,
    miso: ErasedPin,
    cs: ErasedPin,
    busy_in: ErasedPin,
    dc: ErasedPin,
    rst: ErasedPin,
) {
    // Timer::after_secs(1).await;
    let cs = Output::new(cs, Level::Low);
    let busy_in = Input::new(busy_in, Pull::Up); // Display busy refreshing pin: Display pull this high when its busy
    let dc = Output::new(dc, Level::Low); // Display Data/Command pin: Determines whether the transmitted data is a command or display data.
    let rst = Output::new(rst, Level::High); // Display reset pin: Active low pin that resets the E-ink module to known state
                                             // let mut delay = esp_hal::delay::Delay::new();
    let mut delay = embassy_time::Delay;

    info!("Creating spi device");

    let mut spi = Spi::new(spi2, HertzU32::MHz(4), SpiMode::Mode0)
        .with_sck(sclk)
        .with_mosi(mosi)
        .with_miso(miso);

    info!("Setup EPD");
    let mut epd =
        epd_waveshare::epd1in54b::Epd1in54b::new(&mut spi, cs, busy_in, dc, rst, &mut delay)
            .expect("issue constructing Epd1in54b driver");
    info!("Setup EPD Done");

    info!("Wake EPD");
    epd.wake_up(&mut spi, &mut delay).expect("wake up failed");
    info!("Clearing EPD screen");
    epd.clear_frame(&mut spi, &mut delay)
        .expect("clear frame failed");
    epd.display_frame(&mut spi, &mut delay)
        .expect("display frame failed");

    // Create empty buffers for black/white and chromatic layers
    let black_buffer = [0xFFu8; (epd1in54b::WIDTH * epd1in54b::HEIGHT / 8) as usize]; // White background
    let mut chromatic_buffer = [0x00u8; (epd1in54b::WIDTH * epd1in54b::HEIGHT / 8) as usize]; // Black background

    // Set a square of chromatic color in the chromatic buffer (e.g., a 50x50 square in the center)
    let square_size = 50;
    let square_x = (epd1in54b::WIDTH - square_size) / 2;
    let square_y = (epd1in54b::HEIGHT - square_size) / 2;

    for y in square_y..(square_y + square_size) {
        for x in square_x..(square_x + square_size) {
            let byte_index = ((y * epd1in54b::WIDTH + x) / 8) as usize;
            let bit_index = x % 8;
            // Set the pixel to chromatic color by setting the corresponding bit in the chromatic buffer
            chromatic_buffer[byte_index] |= 0x80 >> bit_index;
        }
    }

    info!("Updating color frame");
    epd.update_color_frame(&mut spi, &black_buffer, &chromatic_buffer)
        .expect("update color frame failed");
    info!("Displaying color frame");
    epd.display_frame(&mut spi, &mut delay)
        .expect("display frame failed");

    // Set the EPD to sleep
    Timer::after_secs(3).await;
    info!("Sleeping epd");
    epd.sleep(&mut spi, &mut delay).expect("sleep failed");

    loop {
        Timer::after_secs(1000).await;
    }
}
