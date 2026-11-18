use atmega_hal::port::{
    PD2, Pin,
    mode::{Floating, Input},
};

pub struct Button(Pin<Input<Floating>, PD2>);

impl Button {
    pub fn is_pressed(&self) -> bool {
        self.0.is_low()
    }

    pub const fn new(pin: Pin<Input<Floating>, PD2>) -> Self {
        Self(pin)
    }
}
