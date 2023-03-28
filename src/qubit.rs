use std::fmt::Display;

use crate::amplitude::ProbabilityAmplitude;

#[derive(Debug)]
pub struct Qubit {
    alpha: ProbabilityAmplitude,
    beta: ProbabilityAmplitude
}

impl Qubit {
    pub fn new(alpha: ProbabilityAmplitude, beta: ProbabilityAmplitude) -> Self {
        return Self { alpha: alpha, beta: beta };
    }
}

impl Default for Qubit {
    fn default() -> Self {
        return Self { alpha:ProbabilityAmplitude::new(1, 0), beta: ProbabilityAmplitude::default() };
    }
}

impl Display for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "|ψ〉= {:.2}|0〉+ {:.2}|1〉", self.alpha.normalize().real, self.beta.normalize().real);
    }
}