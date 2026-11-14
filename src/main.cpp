#include "display.h"
#include "clock.h"
#include "library-headers.h"

// NOLINTNEXTLINE(misc-use-internal-linkage)
void setup() {
    setup_display();
    setup_clock();
}

// NOLINTNEXTLINE(misc-use-internal-linkage)
void loop() {
    show_clock();
    delay(500);
    show_clock(false);
    delay(500);
}
