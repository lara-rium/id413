// TODO: make serial operations no-op on prod

use atmega_hal::{
    Usart,
    clock::MHz16,
    pac::USART0,
    port::{
        PD0, PD1, Pin,
        mode::{Floating, Input, Output},
    },
    prelude::_ufmt_uWrite as _,
    usart::Baudrate,
};

pub struct Serial(Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>);

impl Serial {
    pub fn new(
        usart0: USART0,
        pd0: Pin<Input<Floating>, PD0>,
        pd1: Pin<Input<Floating>, PD1>,
    ) -> Self {
        Self(Usart::new(
            usart0,
            pd0,
            pd1.into_output(),
            Baudrate::new(57600),
        ))
    }

    pub fn print(&mut self, text: &str) {
        self.0.write_str(text).unwrap();
        self.0.write_char('\n').unwrap();
    }
}
