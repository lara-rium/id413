use core::fmt;

use heapless::{CapacityError, String};
use ufmt::uwrite;

use crate::display::U8g2Error;

#[derive(Debug)]
pub enum Error {
    Capacity,
    Fmt,
    U8g2(U8g2Error),
}

impl From<U8g2Error> for Error {
    fn from(value: U8g2Error) -> Self {
        Self::U8g2(value)
    }
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Self::Fmt
    }
}

impl From<CapacityError> for Error {
    fn from(_: CapacityError) -> Self {
        Self::Capacity
    }
}

impl From<&Error> for u8 {
    fn from(value: &Error) -> Self {
        match value {
            Error::Capacity => 1,
            Error::Fmt => 2,
            Error::U8g2(error) => match error {
                U8g2Error::BackgroundColorNotSupported => 3,
                U8g2Error::GlyphNotFound(_) => 4,
                U8g2Error::DisplayError(_) => 5,
            },
        }
    }
}

impl From<Error> for String<2> {
    fn from(value: Error) -> Self {
        let mut message = Self::new();
        uwrite!(&mut message, "E{}", u8::from(&value)).unwrap();
        message
    }
}
