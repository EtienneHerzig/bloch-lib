use crate::amplitude::ProbabilityAmplitude;

#[derive(Debug)]
pub struct Qubit {
    alpha: ProbabilityAmplitude,
    beta: ProbabilityAmplitude
}