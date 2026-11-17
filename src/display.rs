use atmega_hal::{
    Atmega, I2c,
    clock::MHz16,
    i2c::I2c0,
    pac::TWI0,
    port::{
        PC4, PC5, Pin,
        mode::{Floating, Input},
    },
};
use embedded_graphics::{draw_target::DrawTarget, geometry::Point, pixelcolor::BinaryColor};
use ssd1306::{
    I2CDisplayInterface,
    mode::{BufferedGraphicsMode, DisplayConfig as _},
    prelude::{DisplayRotation, DisplaySize128x64, I2CInterface},
};
use u8g2_fonts::{
    FontRenderer,
    types::{FontColor, HorizontalAlignment, VerticalPosition},
};

use crate::error::Error;

avr_progmem::progmem! {
    static progmem FONT_32: [u8; include_bytes!("../assets/font_32.u8g2font").len()] =
        *include_bytes!("../assets/font_32.u8g2font");
    static progmem FONT_16: [u8; include_bytes!("../assets/font_16.u8g2font").len()] =
        *include_bytes!("../assets/font_16.u8g2font");
}

type Ssd1306 = ssd1306::Ssd1306<
    I2CInterface<I2c<Atmega, TWI0, Pin<Input, PC4>, Pin<Input, PC5>, MHz16>>,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;

pub type EgError = <Ssd1306 as DrawTarget>::Error;
pub type U8g2Error = u8g2_fonts::Error<EgError>;

pub struct Text<T: AsRef<str>> {
    pub large: T,
    pub small: T,
}

pub struct Display {
    font_16_renderer: FontRenderer,
    font_32_renderer: FontRenderer,
    ssd1306: Ssd1306,
}

impl Display {
    pub fn new(twi0: TWI0, pc4: Pin<Input<Floating>, PC4>, pc5: Pin<Input<Floating>, PC5>) -> Self {
        let i2c = I2c0::<MHz16>::new(
            twi0,
            pc4.into_pull_up_input(),
            pc5.into_pull_up_input(),
            100_000,
        );

        let mut ssd1306 = ssd1306::Ssd1306::new(
            I2CDisplayInterface::new(i2c),
            DisplaySize128x64,
            DisplayRotation::Rotate0,
        );
        ssd1306.init().unwrap();

        Self {
            ssd1306: ssd1306.into_buffered_graphics_mode(),
            font_32_renderer: FontRenderer::new_from_progmem(FONT_32.as_slice()),
            font_16_renderer: FontRenderer::new_from_progmem(FONT_16.as_slice()),
        }
    }

    pub fn print<T>(&mut self, text: &Text<T>) -> Result<(), Error>
    where
        T: AsRef<str>,
    {
        self.ssd1306.clear_buffer();

        if !text.small.as_ref().is_empty() {
            self.font_16_renderer.render_aligned(
                text.small.as_ref(),
                Point::new(63, 0),
                VerticalPosition::Top,
                HorizontalAlignment::Center,
                FontColor::Transparent(BinaryColor::On),
                &mut self.ssd1306,
            )?;
        }

        if !text.large.as_ref().is_empty() {
            self.font_32_renderer.render_aligned(
                text.large.as_ref(),
                Point::new(63, 63),
                VerticalPosition::Bottom,
                HorizontalAlignment::Center,
                FontColor::Transparent(BinaryColor::On),
                &mut self.ssd1306,
            )?;
        }

        self.ssd1306.flush().map_err(U8g2Error::DisplayError)?;

        Ok(())
    }
}
