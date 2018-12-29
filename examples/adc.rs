//! Makes an analog reading on channel 0 and prints it to itm

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;
extern crate stm32f103xx_hal;

//use nb::block;

use core::fmt::Write;

use cortex_m_semihosting::hio;

use stm32f103xx_hal::prelude::*;

use crate::rt::{entry, ExceptionFrame};
use stm32f103xx_hal::adc::{self};
use embedded_hal::adc::OneShot;

#[entry]
fn main() -> ! {
    // Aquire the peripherals
    let p = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
   
    let mut rcc = p.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .hclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(72.mhz())
        .adcclk(14.mhz())
        .freeze(&mut flash.acr);

    // Configure gpioa 0 as an analog input
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    let mut pb1 = gpiob.pb1.into_analog_input(&mut gpiob.crl);
    let mut pb0 = gpiob.pb0.into_analog_input(&mut gpiob.crl);

    // Set up the ADC1
    let mut adc1 = adc::Adc::adc1(p.ADC1, &mut rcc.apb2, &clocks);

    // Set up the ADC2
    let mut adc2 = adc::Adc::adc2(p.ADC2, &mut rcc.apb2, &clocks);

    loop {
        // Aquire stdout and print the result of an analog reading
        // NOTE: This will probably freeze when running without a debugger connected.

        if let Ok(reading1) = adc1.read(&mut pb1) {
            writeln!(hio::hstdout().unwrap(), "ADC1 reading: {}", reading1).unwrap();
        }

        if let Ok(reading2) = adc2.read(&mut pb0) {
            writeln!(hio::hstdout().unwrap(), "ADC2 reading: {}", reading2).unwrap();
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
