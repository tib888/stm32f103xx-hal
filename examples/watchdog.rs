//! Prints "Hello, world" on the OpenOCD console

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate embedded_hal;
extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;

use core::fmt::Write;
use embedded_hal::watchdog::{Watchdog, WatchdogEnable};
use hal::delay::Delay;
use hal::prelude::*;
use hal::watchdog::IndependentWatchdog;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();
    let mut watchdog = IndependentWatchdog::new(dp.IWDG);
    watchdog.start(2_000_000u32.us());

    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    watchdog.feed();
    let mut hstdout = sh::hio::hstdout().unwrap();
    writeln!(hstdout, "hello, world").unwrap();

    loop {
        watchdog.feed();
        delay.delay_ms(1600_u16); //if this time is too big, will reset and print hello, world again
    }
}
