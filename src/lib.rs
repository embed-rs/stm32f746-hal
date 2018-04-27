#![no_std]

extern crate cortex_m;
extern crate embedded_hal as hal;
pub extern crate stm32f7;

pub mod delay;
pub mod flash;
pub mod gpio;
pub mod rcc;
pub mod time;

use stm32f7::stm32f7x6::Peripherals;

pub fn take_peripherals() -> Option<Peripherals> {
    Peripherals::take()
}
