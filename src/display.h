#pragma once
#include "library-headers.h"

extern U8G2_SSD1306_128X64_NONAME_1_HW_I2C display;

void setup_display();

template<typename Function>
void show_text(Function page) {
    display.firstPage();

    page();
    while (display.nextPage() != 0) {
        page();
    }
}

void center(const char *text);

void print(const char *text, bool hidden = false);
