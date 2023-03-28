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