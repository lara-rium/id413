// TODO: adapt once DS1302 arrives

use heapless::String;
use time::{Duration, Timestamp, UtcOffset};
use ufmt::uwrite;

use crate::{display::Text, error::Error};

const OFFSET: UtcOffset = {
    let Ok(offset) = UtcOffset::from_hms(3, 0, 0) else {
        panic!()
    };
    offset
};

pub struct Clock {
    now: Timestamp,
    previous_timestamp: Timestamp,
    timestamp: Timestamp,
}

impl Clock {
    fn format(&self) -> Result<Text<String<11>>, Error> {
        let datetime = self.timestamp.to_offset(OFFSET);
        let now = self.now.to_offset(OFFSET);

        let date = if datetime.date() == now.date() {
            String::new()
        } else if datetime.date() == now.date().previous_day().unwrap() {
            "D\u{fc}n\n".try_into()?
        } else {
            let mut date = String::new();
            let month = datetime.month() as u8;

            uwrite!(
                &mut date,
                "{}/{}{}/{}\n",
                datetime.day(),
                padding(month),
                month,
                datetime.year()
            )?;

            date
        };

        let mut time = String::new();
        let hour = datetime.hour();
        let minute = datetime.minute();

        uwrite!(
            &mut time,
            "{}{}{}{}{}",
            padding(hour),
            hour,
            if now.second() % 2 == 0 { " " } else { ":" },
            padding(minute),
            minute
        )?;

        Ok(Text {
            small: date,
            large: time,
        })
    }

    pub fn new() -> Self {
        let now = Timestamp::from_seconds(env!("BUILD_TIMESTAMP").parse().unwrap()).unwrap();

        // TODO: remove this dev comment after prod
        // 2026/11/18 23:59:45: 1795035585
        // 2026/11/17 23:59:45: 1794949185
        Self {
            now,
            timestamp: now,
            previous_timestamp: now,
        }
    }

    pub fn tick(&mut self) -> Result<Text<String<11>>, Error> {
        self.now += Duration::seconds(1);

        self.format()
    }

    pub const fn undo(&mut self) {
        self.timestamp = self.previous_timestamp;
    }

    pub const fn update(&mut self) {
        self.timestamp = self.now;
    }
}

const fn padding(integer: u8) -> &'static str {
    if integer < 10 { "0" } else { "" }
}
