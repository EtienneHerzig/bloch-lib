use fraction::DynaDecimal as Fraction;
use fraction::{
    GenericDecimal,
    DynaInt,
    BigUint
};

#[derive(Debug, Clone)]
pub struct ProbabilityAmplitude {
    real: Fraction<u16, u16>,
    imag: Fraction<u16, u16>,
}

impl ProbabilityAmplitude {
    fn new<T: Into<GenericDecimal<DynaInt<u16, BigUint>, u16>>, U: Into<GenericDecimal<DynaInt<u16, BigUint>, u16>>> (real: T, imag: U) -> Self {
        return Self { real: real.into(), imag: imag.into() };
    }
}

impl Default for ProbabilityAmplitude {
    fn default() -> Self {
        return Self { real: 0.into(), imag: 0.into() }
    }
}