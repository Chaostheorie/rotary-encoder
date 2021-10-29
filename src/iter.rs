use crate::{ClickableRotaryEncoder, RotaryEncoder};
use core::convert::Infallible;
use embedded_hal::digital::v2::InputPin;
use num_traits::PrimInt;

/// Iterator to conveniently use `RotaryEncoder::read`
pub struct ClickableEncoderIter<
    S: InputPin<Error = Infallible>,
    CL: InputPin<Error = Infallible>,
    D: InputPin<Error = Infallible>,
    C: PrimInt,
>(ClickableRotaryEncoder<S, CL, D, C>);
pub struct EncoderIter<
    CL: InputPin<Error = Infallible>,
    D: InputPin<Error = Infallible>,
    C: PrimInt,
>(RotaryEncoder<CL, D, C>);

impl<CL: InputPin<Error = Infallible>, D: InputPin<Error = Infallible>, C: PrimInt> Iterator
    for EncoderIter<CL, D, C>
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.0.read());
    }
}

impl<
        S: InputPin<Error = Infallible>,
        CL: InputPin<Error = Infallible>,
        D: InputPin<Error = Infallible>,
        C: PrimInt,
    > Iterator for ClickableEncoderIter<S, CL, D, C>
{
    type Item = (bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.0.read());
    }
}

impl<CL: InputPin<Error = Infallible>, D: InputPin<Error = Infallible>, C: PrimInt>
    EncoderIter<CL, D, C>
{
    pub fn new(encoder: RotaryEncoder<CL, D, C>) -> Self {
        Self(encoder)
    }

    pub fn encoder(self) -> RotaryEncoder<CL, D, C> {
        self.0
    }
}

impl<
        S: InputPin<Error = Infallible>,
        CL: InputPin<Error = Infallible>,
        D: InputPin<Error = Infallible>,
        C: PrimInt,
    > ClickableEncoderIter<S, CL, D, C>
{
    pub fn new(encoder: ClickableRotaryEncoder<S, CL, D, C>) -> Self {
        Self(encoder)
    }

    pub fn encoder(self) -> ClickableRotaryEncoder<S, CL, D, C> {
        self.0
    }
}
