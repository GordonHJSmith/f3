//! Serial interface echo server
//!
//! In this example every received byte will be sent back to the sender. You can test this example
//! with serial terminal emulator like `minicom`.
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

extern crate f3;
#[macro_use(block)]
extern crate nb;

use f3::hal::prelude::*;
use f3::hal::serial::Serial;
use f3::hal::stm32f30x;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    let serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
    let (mut tx, mut rx) = serial.split();

    loop {
        let byte = block!(rx.read()).unwrap();
        block!(tx.write(byte)).ok();
    }
}
