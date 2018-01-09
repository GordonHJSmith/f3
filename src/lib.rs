//! Board support crate for the STM32F3DISCOVERY

#![feature(never_type)]
#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;
pub extern crate stm32f30x;

pub mod delay;
pub mod flash;
pub mod gpio;
pub mod i2c;
pub mod led;
pub mod prelude;
pub mod rcc;
pub mod serial;
pub mod spi;
pub mod time;
pub mod timer;
