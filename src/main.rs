#![feature(
    multiple_supertrait_upcastable,
    non_exhaustive_omitted_patterns_lint,
    strict_provenance_lints,
    unqualified_local_imports
)]
#![no_std]
#![no_main]

use atmega_hal::{clock::MHz16, delay::Delay};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut led = pins.pb5.into_output();
    let mut delay = Delay::<MHz16>::new();

    loop {
        led.toggle();
        delay.delay_ms(1000);
    }
}
