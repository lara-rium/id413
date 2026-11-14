#include "display.h"
#include "library-headers.h"

namespace {
    constexpr int width = 128;
    constexpr int height = 64;

    U8G2_SSD1306_128X64_NONAME_1_HW_I2C display(U8G2_R0);
}


void setup_display() {
    display.begin();
    display.setFont(&u8g2_font_logisoso34_tr[0]);
    display.setFontPosBaseline();
    display.setFontMode(1);
}

void center(const char *text) {
    display.setCursor((width - display.getStrWidth(text)) / 2, height);
}

void print(const char *text, const bool hidden = false) {
    if (hidden) {
        display.setDrawColor(0);
    }

    display.print(text);
}
