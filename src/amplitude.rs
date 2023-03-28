use fraction::DynaDecimal as Fraction;


#[derive(Debug, Clone)]
pub struct ProbabilityAmplitude {
    real: Fraction<u16, u16>,
    imag: Fraction<u16, u16>,
}