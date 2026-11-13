#include "display.h"
#include "library-headers.h"

namespace {
    constexpr int width = 128;
    constexpr int height = 64;

    U8G2_SSD1306_128X64_NONAME_1_HW_I2C display(U8G2_R0);

    void show_page(const char *text) {
        display.setCursor((width - display.getStrWidth(text)) / 2, height);
        display.print(text);
    }
}


void setup_display() {
    display.begin();
    display.setFont(&u8g2_font_logisoso32_tr[0]);
    display.setFontPosBaseline();
}


void show_text(const char *text) {
    display.firstPage();

    show_page(text);
    while (display.nextPage() != 0) {
        show_page(text);
    }
}
