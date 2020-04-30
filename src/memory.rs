use crate::constant::Constant;
use std::collections::HashMap;
use std::mem;

// This is a more efficient version of hashing a float value
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
        let bits: u64 = unsafe { mem::transmute(item) };
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

#[derive(Debug)]
pub struct ConstantPool {
    pool: Vec<Constant>,
    cache: HashMap<Constant, usize>,
}

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool {
            pool: vec![],
            cache: HashMap::<Constant, usize>::new(),
        }
    }

    pub fn add(&mut self, constant: Constant) -> usize {
        if self.cache.contains_key(&constant) {
            *self.cache.get(&constant).unwrap()
        } else {
            self.cache.insert(constant.clone(), self.pool.len());
            self.pool.push(constant);
            self.pool.len() - 1
        }
    }

    pub fn get(&self, constant_index: usize) -> &Constant {
        &self.pool[constant_index]
    }
}
#[cfg(test)]
mod tests {
    use super::ConstantPool;
    use super::Distance;

    use super::Constant;

    fn add_constant(constant: Constant) -> usize {
        let mut pool = ConstantPool::new();
        pool.add(constant)
    }

    #[test]
    fn test() {
        assert_eq!(
            Distance::new(5066549580791808, -50, 1),
            Distance::from(4.5 as f64)
        );

        assert_eq!(add_constant(Constant::from(0 as f64)), 0);
    }
}
