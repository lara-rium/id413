#![feature(
    multiple_supertrait_upcastable,
    non_exhaustive_omitted_patterns_lint,
    strict_provenance_lints,
    unqualified_local_imports
)]
#![no_std]
#![no_main]

mod buttons;
mod clock;
mod display;
mod error;
mod serial;

use atmega_hal::{clock::MHz16, delay::Delay, pins};
use embedded_hal::delay::DelayNs as _;
use heapless::String;
use panic_halt as _;

use crate::{
    buttons::Buttons,
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
    let buttons = Buttons::new(pins.pd2, pins.pd3);
    let mut delay = Delay::<MHz16>::new();
    let mut clock = Clock::new();

    loop {
        if let Err(error) = iteration(&mut display, &buttons, &mut delay, &mut clock) {
            let error_message = String::from(error);

            serial.print(&error_message);
            let _: Result<(), Error> = display.print(&Text {
                large: "",
                small: &error_message,
            });
        }
    }
}

fn iteration(
    display: &mut Display,
    buttons: &Buttons,
    delay: &mut Delay<MHz16>,
    clock: &mut Clock,
) -> Result<(), Error> {
    if buttons.update_clock.is_low() {
        clock.update();
    }

    if buttons.wake_screen.is_low() {
        display.wake()?;
    }

    let clock_format = clock.tick()?;
    display.tick()?;

    display.print(&clock_format)?;

    delay.delay_ms(1000);

    Ok(())
}
