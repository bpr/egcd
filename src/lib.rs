use num_bigint::BigInt;
use num_traits::Zero;


#[macro_export]
macro_rules! bi {
    ($x: literal) => {
        $crate::bi!($x, 10)
    };
    ($x: literal, $base: literal) => {
        num_bigint::BigInt::parse_bytes($x.as_bytes(), $base).unwrap()
    }
}

pub fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, (BigInt, BigInt), (BigInt, BigInt)) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (BigInt::from(1), BigInt::from(0));
    let (mut old_t, mut t) = (BigInt::from(0), BigInt::from(1));
    
    while r != BigInt::zero() {
        let quotient = old_r.clone() / r.clone();
        let tmp = r.clone();
        (old_r, r) = (r, old_r - quotient.clone() * tmp);
        let tmp = s.clone();
        (old_s, s) = (s, old_s - quotient.clone() * tmp);
        let tmp = t.clone();
        (old_t, t) = (t, old_t - quotient.clone() * tmp);
    }
    (old_r, (old_s, old_t), (s, t))
}

pub fn mod_inverse(a: BigInt, m: BigInt) -> BigInt {
    let (gcd, (s, _), _) = extended_gcd(a.clone(), m.clone());
    if gcd != BigInt::from(1) {
        panic!("{} and {} are not coprime", a, m);
    }
    if s < BigInt::zero() {
        m + s
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let (gcd, (s, t), (old_s, old_t)) = extended_gcd(bi!("240"), bi!("46"));
        assert_eq!([gcd, s, t, old_s, old_t],
            [bi!("2"), bi!("-9"), bi!("47"), bi!("23"), bi!("-120")]);
    }

    #[test]
    fn mod_inverse_test() {
        assert_eq!(mod_inverse(bi!("3"), bi!("11")), bi!("4"));
        assert_eq!(mod_inverse(bi!("5"), bi!("11")), bi!("9"));
        assert_eq!(mod_inverse(bi!("7"), bi!("11")), bi!("8"));
    }
}
