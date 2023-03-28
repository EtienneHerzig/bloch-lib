mod amplitude;

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
}
