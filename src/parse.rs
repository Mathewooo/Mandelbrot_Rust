pub mod parse {
    use std::str::FromStr;
    use num_complex::Complex;

    pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
        match s.find(separator) {
            None => None,
            Some(index) => {
                match (T::from_str(&s[..index]),
                       T::from_str(&s[index + 1..])
                ) {
                    (Ok(l), Ok(r))
                    => Some((l, r)),
                    _ => None
                }
            }
        }
    }

    pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
        match parse_pair(s, ',') {
            Some((re, im)) => Some(
                Complex { re, im }
            ),
            None => None
        }
    }
}