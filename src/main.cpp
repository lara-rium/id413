#include "display.h"
#include <Arduino.h>

// NOLINTNEXTLINE(misc-use-internal-linkage)
void setup() {
    setup_display();

    show_text("01:40");
}

// NOLINTNEXTLINE(misc-use-internal-linkage)
void loop() {
}
