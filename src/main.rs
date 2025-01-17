#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use portable_atomic::{AtomicBool, AtomicF64};
use {defmt_rtt as _, panic_probe as _};

mod button;
mod display;
mod heater;
mod sensor;
mod usart;

static TEMPERATURE: AtomicF64 = AtomicF64::new(0.0);
static PRESSURE: AtomicF64 = AtomicF64::new(0.0);
static HEATER_ON: AtomicBool = AtomicBool::new(false);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    unwrap!(spawner.spawn(usart::run_task(
        p.USART2,
        p.PA3,
        p.PA2,
        p.DMA1_CH7,
        p.DMA1_CH6,
        &TEMPERATURE,
        &PRESSURE
    )));

    unwrap!(spawner.spawn(sensor::run_task(
        p.I2C2,
        p.PA9,
        p.PA10,
        &TEMPERATURE,
        &PRESSURE,
        &HEATER_ON
    )));
    unwrap!(spawner.spawn(display::run_task(
        p.I2C1,
        p.PB6,
        p.PB7,
        &TEMPERATURE,
        &PRESSURE
    )));
    unwrap!(spawner.spawn(button::run_task(p.PA0, p.EXTI0, &HEATER_ON)));
    unwrap!(spawner.spawn(heater::run_task(p.PB12, &HEATER_ON)));
}
