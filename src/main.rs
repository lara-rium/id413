#![feature(
    multiple_supertrait_upcastable,
    non_exhaustive_omitted_patterns_lint,
    strict_provenance_lints,
    unqualified_local_imports
)]
#![no_std]
#![no_main]

mod display;

use atmega_hal::{clock::MHz16, delay::Delay};
use embedded_hal::delay::DelayNs as _;
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let peripherals = atmega_hal::Peripherals::take().unwrap();

    let mut delay = Delay::<MHz16>::new();

    let mut display = display::init(peripherals);
    display::print("12:50", &mut display);

    loop {
        delay.delay_ms(1000);
    }
}
