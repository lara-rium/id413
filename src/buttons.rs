use atmega_hal::port::{
    PD2, PD3, Pin,
    mode::{Floating, Input, PullUp},
};

pub struct Buttons {
    pub update_clock: Pin<Input<PullUp>, PD2>,
    pub wake_screen: Pin<Input<PullUp>, PD3>,
}

impl Buttons {
    pub fn new(pd2: Pin<Input<Floating>, PD2>, pd3: Pin<Input<Floating>, PD3>) -> Self {
        Self {
            update_clock: pd2.into_pull_up_input(),
            wake_screen: pd3.into_pull_up_input(),
        }
    }
}
