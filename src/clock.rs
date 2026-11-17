// TODO: adapt once DS1302 arrives

use atmega_hal::{clock::MHz16, delay::Delay};
use embedded_hal::delay::DelayNs as _;
use heapless::{String, format};
use time::{Duration, Timestamp, UtcOffset};

use crate::{display::Text, error::Error};

const OFFSET: UtcOffset = {
    let Ok(offset) = UtcOffset::from_hms(3, 0, 0) else {
        panic!()
    };
    offset
};

pub struct Clock {
    delay: Delay<MHz16>,
    now: Timestamp,
}

impl Clock {
    pub fn format(&self, timestamp: Timestamp) -> Result<Text<String<11>>, Error> {
        let datetime = timestamp.to_offset(OFFSET);
        let now = self.now.to_offset(OFFSET);

        // if datetime.date() != now.date() {
        let date = if datetime.date() == now.date() {
            format!(
                "{day}/{month:02}/{year}\n",
                day = datetime.day(),
                month = datetime.month() as u8,
                year = datetime.year()
            )?
        } else {
            String::new()
        };

        let time = format!(
            "{hour:02}:{minute:02}",
            hour = datetime.hour(),
            minute = datetime.minute()
        )?;

        Ok(Text {
            small: date,
            large: time,
        })
    }

    pub fn new() -> Self {
        Self {
            now: Timestamp::from_seconds(env!("BUILD_TIMESTAMP").parse().unwrap()).unwrap(),
            delay: Delay::new(),
        }
    }

    pub const fn now(&self) -> Timestamp {
        self.now
    }

    pub fn tick(&mut self) {
        self.delay.delay_ms(1000);
        self.now += Duration::seconds(1);
    }
}
