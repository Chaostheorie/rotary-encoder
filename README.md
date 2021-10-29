# Suport for Rotary Encoder with `embedded_hal`

A generic interface for a rotary encoder with `embedded_hal` with optional support for clickable encoders. Supports arbitrary precision for the internal counter and an iterator for retrieving data continously.

## Examples

There's an example for outputting the read values from an [esp8266]() over [esp8266-hal](). You will need to configure [espflash]() aand the rust-xtensa compiler for this example to work (Consult the [esp8266-hal instructions for setting those two up]()).

The example will output the current direction and rotation counter for a clickable rotation encoder over the serial connection. Additionally the onboard LED will be activated when the button on the encoder is pressed.

To flash to your esp: `cargo +xtensa espflash --monitor --features examples --example serial-led --release <USB device>`
