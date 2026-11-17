use atmega_hal::{
    Atmega, I2c, Peripherals,
    clock::MHz16,
    i2c::I2c0,
    pac::TWI0,
    pins,
    port::{PC4, PC5, Pin, mode::Input},
};
use embedded_graphics::{geometry::Point, pixelcolor::BinaryColor};
use ssd1306::{
    I2CDisplayInterface, Ssd1306,
    mode::{BufferedGraphicsMode, DisplayConfig as _},
    prelude::{DisplayRotation, DisplaySize128x64, I2CInterface},
};
use u8g2_fonts::{
    FontRenderer,
    types::{FontColor, HorizontalAlignment, VerticalPosition},
};

const FONT_RENDERER: FontRenderer =
    FontRenderer::new_from_raw_data(include_bytes!("../assets/font.u8g2"));

type Display = Ssd1306<
    I2CInterface<I2c<Atmega, TWI0, Pin<Input, PC4>, Pin<Input, PC5>, MHz16>>,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;

pub fn init(peripherals: Peripherals) -> Display {
    let pins = pins!(peripherals);

    let i2c = I2c0::<MHz16>::new(
        peripherals.TWI0,
        pins.pc4.into_pull_up_input(),
        pins.pc5.into_pull_up_input(),
        100_000,
    );

    let mut display = Ssd1306::new(
        I2CDisplayInterface::new(i2c),
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();
    display.init().unwrap();

    display
}

pub fn print(text: &str, display: &mut Display) {
    FONT_RENDERER
        .render_aligned(
            text,
            Point::new(64, 32),
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::On),
            display,
        )
        .unwrap();

    display.flush().unwrap();
}
