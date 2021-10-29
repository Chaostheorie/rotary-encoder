//! # Support for Rotary Encoder with `embedded_hal`
//!
//! A generic interface for a rotary encoder over [`embedded_hal`](https://github.com/rust-embedded/embedded-hal) with optional support for clickable encoders. Supports arbitrary counter length and an iterator for retrieving data continuously.
//!
//! ## Examples
//!
//! There's an example for outputting the read values from an [esp8266](https://en.wikipedia.org/wiki/ESP8266) over [esp8266-hal](https://github.com/esp-rs/esp8266-hal). You will need to configure [espflash](https://github.com/esp-rs/espflash) and the [rust-xtensa](https://github.com/MabezDev/rust-xtensa) compiler for this example to work (Consult the [esp8266-hal instructions for setting those two up](https://github.com/esp-rs/esp8266-hal#setting-up-the-compiler)).
//!
//! The example will output the current direction and rotation counter for a clickable rotation encoder over the serial connection. Additionally the onboard LED will be activated when the button on the encoder is pressed.
//! Pins are by default CLK -> `D7`, SW -> `D6`, DT -> `D5`. Change them as needed but keep in mind that the
//!
//! To flash to your esp: `cargo +xtensa espflash --monitor --features examples --example serial-led --release <USB device>`
//!
#![no_std]

use core::convert::Infallible;
use embedded_hal::digital::v2::InputPin;
use iter::{ClickableEncoderIter, EncoderIter};
use num_traits::PrimInt;

/// Iterators for conveniently reading encoders
pub mod iter;

/// Rotation direction of encoder
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
    Unknown,
}

impl core::fmt::Display for RotationDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            &Self::Clockwise => "CW",
            &Self::Unknown => "?",
            &Self::CounterClockwise => "CCW",
        })
    }
}

/// # Basic Rotary Encoder
/// Basic bind to a rotary encoder with support for arbitrary counter size
pub struct RotaryEncoder<
    CL: InputPin<Error = Infallible>,
    D: InputPin<Error = Infallible>,
    C: PrimInt,
> {
    clk: CL,
    clk_state: bool,
    dt: D,
    /// Rotation step Counter
    pub counter: C,
    /// Current rotation direction. Will be `RotationDirection::Unknown` until first read.
    pub direction: RotationDirection,
}

/// # Basic Rotary Encoder with integrated button
/// Basic bind to a rotary encoder with an integrated push button and support for arbitrary counter size
pub struct ClickableRotaryEncoder<
    S: InputPin<Error = Infallible>,
    CL: InputPin<Error = Infallible>,
    D: InputPin<Error = Infallible>,
    C: PrimInt,
> {
    switch: S,
    pub encoder: RotaryEncoder<CL, D, C>,
}

impl<CL: InputPin<Error = Infallible>, D: InputPin<Error = Infallible>, C: PrimInt>
    RotaryEncoder<CL, D, C>
{
    /// Create new instance with specified `counter`
    pub fn with_counter(clk: CL, dt: D, counter: C) -> Self {
        let clk_state = clk.is_high().unwrap();

        Self {
            clk,
            dt,
            clk_state,
            direction: RotationDirection::Unknown,
            counter,
        }
    }

    /// Create new instance with counter at `0`
    pub fn new(clk: CL, dt: D) -> Self {
        let clk_state = clk.is_high().unwrap();

        Self {
            clk,
            dt,
            clk_state,
            direction: RotationDirection::Unknown,
            counter: C::from(0).unwrap(),
        }
    }

    /// read values from data pins and update internal state
    /// Result: Will be `true` when a rotation step has happened since last read
    pub fn read(&mut self) -> bool {
        // This uses C::from([0, 1]) extensively. This might need to be unwraped
        // but is actually guaranteed to work since PrimInt aliases Num, which requires numbers to represent 0 and 1
        let current_clk_state = self.clk.is_high().unwrap();

        if current_clk_state != self.clk_state {
            if self.dt.is_high().unwrap() != current_clk_state {
                self.counter = match self.counter.checked_sub(&C::from(1).unwrap()) {
                    Some(counter) => counter,
                    None => self.counter,
                };

                self.direction = RotationDirection::CounterClockwise;
            } else {
                self.counter = match self.counter.checked_add(&C::from(1).unwrap()) {
                    Some(counter) => counter,
                    None => C::from(0).unwrap(),
                };

                self.direction = RotationDirection::Clockwise;
            }

            self.clk_state = current_clk_state;

            true
        } else {
            false
        }
    }

    /// create iter from current encoder
    pub fn iter(self) -> EncoderIter<CL, D, C> {
        EncoderIter::new(self)
    }
}

impl<
        S: InputPin<Error = Infallible>,
        CL: InputPin<Error = Infallible>,
        D: InputPin<Error = Infallible>,
        C: PrimInt,
    > ClickableRotaryEncoder<S, CL, D, C>
{
    /// Create new instance with specified `counter`
    pub fn with_counter(switch: S, clk: CL, dt: D, counter: C) -> Self {
        let clk_state = clk.is_high().unwrap();

        Self {
            switch,
            encoder: RotaryEncoder {
                clk,
                dt,
                clk_state,
                direction: RotationDirection::Unknown,
                counter,
            },
        }
    }

    /// Create new instance with counter at `0`
    pub fn new(switch: S, clk: CL, dt: D) -> Self {
        let clk_state = clk.is_high().unwrap();

        Self {
            switch,
            encoder: RotaryEncoder {
                clk,
                dt,
                clk_state,
                direction: RotationDirection::Unknown,
                counter: C::from(0).unwrap(),
            },
        }
    }

    /// Check if rotary encoder is pressed down
    pub fn switch_pressed(&self) -> bool {
        return self.switch.is_low().unwrap();
    }

    /// read values from data pins and update internal state
    /// Result (change, button):
    /// - change: Will be `true` when a change has happened since last read
    /// - button: Will be `true` when button is pressed (this can be checked with `RotaryEncoder.switch_pressed` too)
    pub fn read(&mut self) -> (bool, bool) {
        (self.encoder.read(), self.switch_pressed())
    }

    /// create iter from current encoder
    pub fn iter(self) -> ClickableEncoderIter<S, CL, D, C> {
        ClickableEncoderIter::new(self)
    }
}
