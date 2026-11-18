#![feature(
    multiple_supertrait_upcastable,
    non_exhaustive_omitted_patterns_lint,
    strict_provenance_lints,
    unqualified_local_imports
)]
#![no_std]
#![no_main]

mod clock;
mod display;
mod error;
mod serial;

use atmega_hal::pins;
use heapless::String;
use panic_halt as _;

use crate::{
    clock::Clock,
    display::{Display, Text},
    error::Error,
    serial::Serial,
};

#[atmega_hal::entry]
fn main() -> ! {
    let peripherals = atmega_hal::Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut serial = Serial::new(peripherals.USART0, pins.pd0, pins.pd1);
    let mut display = Display::new(peripherals.TWI0, pins.pc4, pins.pc5);
    let mut clock = Clock::new();

    loop {
        clock.tick();

        if let Err(error) = iteration(&mut display, &clock) {
            let error_message = String::from(error);
            serial.print(&error_message);
            let _: Result<(), Error> = display.print(&Text {
                large: "",
                small: &error_message,
            });
        }
    }
}

fn iteration(display: &mut Display, clock: &Clock) -> Result<(), Error> {
    display.print(&clock.format(clock.now())?)?;

    Ok(())
}
