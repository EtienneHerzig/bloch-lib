use std::fmt::Display;
use fraction::BigFraction as Fraction; // TODO: BigFraction is pretty slow

#[derive(Debug, Clone)]
pub struct ProbabilityAmplitude {
    real: Fraction,
    imag: Fraction,
}

impl ProbabilityAmplitude {
    pub fn new<T: Into<Fraction>, U: Into<Fraction>> (real: T, imag: U) -> Self {
        return Self { real: real.into(), imag: imag.into() };
    }
}

impl Default for ProbabilityAmplitude {
    fn default() -> Self {
        return Self { real: 0.into(), imag: 0.into() }
    }
}

impl Display for ProbabilityAmplitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} + {:.2}i", self.real, self.imag)
    }
}