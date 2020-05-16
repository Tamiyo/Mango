use std::cmp;
use std::ops;

// This is a more efficient version of hashing a float value
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Distance {
    mantissa: u64,
    exponent: i16,
    sign: i8,
}

impl Distance {
    pub fn new(mantissa: u64, exponent: i16, sign: i8) -> Self {
        Distance {
            mantissa,
            exponent,
            sign,
        }
    }
}

impl From<f64> for Distance {
    fn from(item: f64) -> Self {
        let bits: u64 = item.to_bits();
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };

        exponent -= 1023 + 52;

        Distance {
            mantissa,
            exponent,
            sign,
        }
    }
}

impl Into<f64> for Distance {
    fn into(self) -> f64 {
        (self.sign as f64) * (self.mantissa as f64) * (2f64.powf(self.exponent as f64))
    }
}

impl Into<f64> for &Distance {
    fn into(self) -> f64 {
        (self.sign as f64) * (self.mantissa as f64) * (2f64.powf(self.exponent as f64))
    }
}

impl Into<usize> for Distance {
    fn into(self) -> usize {
        ((self.sign as f64) * (self.mantissa as f64) * (2f64.powf(self.exponent as f64))) as usize
    }
}

impl ops::Add<Distance> for Distance {
    type Output = f64;

    fn add(self, _rhs: Distance) -> f64 {
        Into::<f64>::into(self) + Into::<f64>::into(_rhs)
    }
}

impl ops::Sub<Distance> for Distance {
    type Output = f64;

    fn sub(self, _rhs: Distance) -> f64 {
        Into::<f64>::into(self) - Into::<f64>::into(_rhs)
    }
}

impl ops::Mul<Distance> for Distance {
    type Output = f64;

    fn mul(self, _rhs: Distance) -> f64 {
        Into::<f64>::into(self) * Into::<f64>::into(_rhs)
    }
}

impl ops::Div<Distance> for Distance {
    type Output = f64;

    fn div(self, _rhs: Distance) -> f64 {
        Into::<f64>::into(self) / Into::<f64>::into(_rhs)
    }
}

impl ops::Rem<Distance> for Distance {
    type Output = f64;

    fn rem(self, _rhs: Distance) -> f64 {
        Into::<f64>::into(self) % Into::<f64>::into(_rhs)
    }
}

impl cmp::PartialOrd for Distance {
    fn partial_cmp(&self, other: &Distance) -> Option<cmp::Ordering> {
        let n1 = Into::<f64>::into(self.clone());
        let n2 = Into::<f64>::into(other.clone());
        n1.partial_cmp(&n2)
    }
}
