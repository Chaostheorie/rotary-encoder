# Support for Rotary Encoder with `embedded_hal`

A generic interface for a rotary encoder over [`embedded_hal`](https://github.com/rust-embedded/embedded-hal) with optional support for clickable encoders. Supports arbitrary counter length and an iterator for retrieving data continuously.

## Examples

There's an example for outputting the read values from an [esp8266](https://en.wikipedia.org/wiki/ESP8266) over [esp8266-hal](https://github.com/esp-rs/esp8266-hal). You will need to configure [espflash](https://github.com/esp-rs/espflash) and the [rust-xtensa](https://github.com/MabezDev/rust-xtensa) compiler for this example to work (Consult the [esp8266-hal instructions for setting those two up](https://github.com/esp-rs/esp8266-hal#setting-up-the-compiler)).

The example will output the current direction and rotation counter for a clickable rotation encoder over the serial connection. Additionally the onboard LED will be activated when the button on the encoder is pressed.
Pins are by default CLK -> `D7`, SW -> `D6`, DT -> `D5`. Change them as needed but keep in mind that the

To flash to your esp: `cargo +xtensa espflash --monitor --features examples --example serial-led --release <USB device>`
