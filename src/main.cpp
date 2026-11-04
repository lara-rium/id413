#include <Arduino.h>

void setup() {
    pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {
    constexpr int second = 1000;

    digitalWrite(LED_BUILTIN, HIGH);
    delay(second);

    digitalWrite(LED_BUILTIN, LOW);
    delay(second);
}
