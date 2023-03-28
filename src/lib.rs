#![allow(dead_code)]


mod amplitude;
mod qubit;


#[cfg(test)]
mod tests {
    use super::amplitude::ProbabilityAmplitude;
    use rand;

    #[test]
    fn default_fmt() {
        let expected = "0 + 0i".to_owned();
        let result = format!("{}", ProbabilityAmplitude::default());

        assert_eq!(expected, result);
    }

    #[test]
    fn rand_new_fmt() {
        let real:f64 = rand::random::<f64>();
        let imag: f64 = rand::random::<f64>();
        let expected = format!("{:.2} + {:.2}i", real, imag);
        let result: String = format!("{}", ProbabilityAmplitude::new(real, imag));
        assert_eq!(expected, result);
    }
}
