// Appropriated from https://github.com/simon-andrews/rust-modinverse

/// Finds the greatest common denominator of two integers *a* and *b*, and two
/// integers *x* and *y* such that *ax* + *by* is the greatest common
/// denominator of *a* and *b* (Bézout coefficients).
///
/// This function is an implementation of the [extended Euclidean
/// algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).
pub fn ext_euclid(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = ext_euclid(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

/// Calculates the [modular multiplicative
/// inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse) *x*
/// of an integer *a* such that *ax* ≡ 1 (mod *m*).
///
/// Such an integer may not exist. If so, this function will return `None`.
/// Otherwise, the inverse will be returned wrapped up in a `Some`.
pub fn mod_mult_inv(a: i128, m: i128) -> Option<i128> {
    let (g, x, _) = ext_euclid(a, m);
    if g != 1 { None } else { Some((x % m + m) % m) }
}

#[cfg(test)]
mod tests {
    use crate::math::mod_mult_inv;

    // Much credit to https://stackoverflow.com/a/50670084
    const M: i128 = 101;
    const X: i128 = 387420489;
    #[rustfmt::skip]
    const OBFUSCATIONS: [(i128, i128); 10] = [
        (1, 43), (2, 86),
        (3, 28), (4, 71),
        (5, 13), (6, 56),
        (7, 99), (8, 41),
        (9, 84), (10, 26),
    ];

    #[test]
    fn obfuscate() {
        for (i, obf) in OBFUSCATIONS {
            let mult_inv = mod_mult_inv(X, M).unwrap();

            let obfuscated = i * X % M;
            let original = obfuscated * mult_inv % M;

            assert_eq!(obfuscated, obf);
            assert_eq!(original, i);
        }
    }
}
