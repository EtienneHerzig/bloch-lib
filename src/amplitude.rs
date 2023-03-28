use std::fmt::Display;
use fraction::BigFraction as Fraction; // TODO: BigFraction is pretty slow
use std::ops::{Add, Sub, Mul, Div};


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

impl Add for ProbabilityAmplitude {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::new(self.real + rhs.real, self.imag + rhs.imag);
    }
}

impl Sub for ProbabilityAmplitude {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new(&self.real - &rhs.real, &self.imag - &rhs.imag);
    }
}

impl Mul for ProbabilityAmplitude {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Self::new(&self.real * &rhs.real - &self.imag * &rhs.imag, &self.real * &rhs.imag + &self.imag * &rhs.real);
    }
}

impl Div for ProbabilityAmplitude {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        return Self::new((&self.real * &rhs.real + &self.imag * &rhs.imag)/(&rhs.real * &rhs.real + &rhs.imag * &rhs.imag), (&self.imag * &rhs.real - &self.real * &rhs.imag)/(&rhs.real * &rhs.real + &rhs.imag * &rhs.imag))
    }
}

impl ProbabilityAmplitude {
    pub fn normalize(&self) -> Self {
        return self.clone() * self.clone();
    }
}