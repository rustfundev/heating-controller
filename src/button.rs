use defmt::*;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::peripherals;
use embassy_time::Timer;
use portable_atomic::{AtomicBool, Ordering};

#[embassy_executor::task]
pub async fn run_task(
    button_pin: peripherals::PA0,
    led_pin: peripherals::PA5,
    extio: peripherals::EXTI0,
    heater_on: &'static AtomicBool,
    button_pressed: &'static AtomicBool,
) {
    let button = ExtiInput::new(button_pin, extio, Pull::Down);
    let mut manual_mode_led = Output::new(led_pin, Level::Low, Speed::Low);

    info!("Press the USER button...");

    loop {
        if button.is_high() {
            // Button is pressed. The heater control is now manual.
            button_pressed.store(true, Ordering::Relaxed);
            manual_mode_led.set_high();
            if !heater_on.load(Ordering::Relaxed) {
                heater_on.store(true, Ordering::Relaxed);
                info!("Power heater on");
            } else {
                heater_on.store(false, Ordering::Relaxed);
                info!("Power heater off");
            }
        }
        Timer::after_secs(3).await;
    }
}
