#![no_std]
#![no_main]
#![feature(trait_alias)]

use core::fmt::Write;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use panic_halt as _;
use rotary_encoder::ClickableRotaryEncoder;

#[entry]
fn main() -> ! {
    // init peripherals and get bind to GPIOs
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();

    // Status led for pressed button of encoder
    let mut led = pins.gpio2.into_push_pull_output();
    led.set_low().unwrap();

    // timers for loop delay
    let (mut timer, _) = dp.TIMER.timers();

    // init encoder with GPIO pins.
    // You can use any pins that can reliable be used as an input
    let mut encoder: ClickableRotaryEncoder<_, _, _, u32> = ClickableRotaryEncoder::new(
        pins.gpio12.into_pull_up_input(),  // d6
        pins.gpio13.into_floating_input(), // d7
        pins.gpio14.into_floating_input(), // d5
    );

    // serial for demo purposes
    // You can see the output with `cargo +xtensa espflash --monitor …`
    let mut serial = dp
        .UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    write!(
        serial,
        "\nThank you for testing this example. I hope the crate will be useful to you.\n(PS: there's a 20 ms delay between reads to prevent jittering) \n"
    )
    .unwrap();

    loop {
        timer.delay_ms(20);

        let (change, button) = encoder.read();

        if change {
            write!(
                serial,
                "button: {} | direction: {} | counter: {}\n",
                match button {
                    true => "pressed",
                    false => "released",
                },
                encoder.encoder.counter,
                encoder.encoder.direction
            )
            .unwrap();
        }

        if !button {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
