#include "clock.h"
#include "display.h"
#include "library-headers.h"

namespace {
    RTC_Millis clock;
}

void setup_clock() {
    clock.begin(DateTime(F(__DATE__), F(__TIME__)));
}

void show_clock(const bool show_colon) {
    char time[] = "hh:mm";
    clock.now().toString(&time[0]);

    show_text([time, show_colon]() -> void {
        center(&time[0]);

        print(time[0]);
        print(time[1]);

        print(":", true);

        print(&time[3]);
    });
}
