#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate nb;
extern crate panic_halt;
extern crate stm32f4xx_hal;

mod rpn_calculator;

use cortex_m_rt::entry;
use stm32f4xx_hal::{pac::Peripherals, prelude::*, serial};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let gpioa = peripherals.GPIOA.split();
    let tx = gpioa.pa0.into_alternate::<8>();
    let rx = gpioa.pa1.into_alternate::<8>();

    let clocks = peripherals.RCC.constrain().cfgr.freeze();
    let config = serial::config::Config::default();
    let mut serial = serial::Serial::new(peripherals.UART4, (tx, rx), config, &clocks).unwrap();

    rpn_calculator::run_repl(&mut serial);

    loop {}
}
