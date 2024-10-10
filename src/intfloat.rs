use num_traits::{Num, One, Pow, ToPrimitive, Zero};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, Mul, Rem, Sub, SubAssign};

#[derive(Clone, Copy, Hash, Eq, PartialOrd, Default, Debug)]
/// Simplified alternative to rust_decimal::Decimal to that allows hashing. Standard
/// library floats can't be hashed which is needed when using (for example) HashMaps.
/// The number is converted to a x 10^-b, with a and b being integers (and thus Hashable).
///
/// Note that when accuracy is important, rust_decimal::Decimal is a safer alternative. But when
/// speed is needed, this implementation is faster.
///
/// # Examples
///
/// ```
/// use intfloat::IntFloat;
/// let a = IntFloat::from(10 as f32, 0);
/// let b = IntFloat::from(5.2, 0);
/// assert_eq!(a, b+b);
///
/// let c = IntFloat::from(5.2, 1);
/// assert_ne!(b, c);
/// ```
pub struct IntFloat {
    base: isize,
    pow: isize,
}

impl IntFloat {
    pub fn new(base: isize, pow: isize) -> Self {
        IntFloat { base, pow }
    }

    pub fn from(float: f32, decimals: isize) -> Self {
        IntFloat {
            base: (float * isize::pow(10, decimals as u32) as f32).round() as isize,
            pow: decimals,
        }
    }

    pub fn print(self) -> String {
        if self.pow >= 1 {
            self.to_f32().unwrap().to_string()
        } else {
            self.to_i64().unwrap().to_string()
        }
    }
}

impl Display for IntFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl Add<Self> for IntFloat {
    type Output = IntFloat;

    fn add(self, rhs: Self) -> IntFloat {
        if rhs.pow > self.pow {
            IntFloat {
                base: self.base * isize::pow(10, (rhs.pow - self.pow) as u32) + rhs.base,
                pow: rhs.pow,
            }
        } else {
            IntFloat {
                base: rhs.base * isize::pow(10, (self.pow - rhs.pow) as u32) + self.base,
                pow: self.pow,
            }
        }
    }
}

impl Add<IntFloat> for &mut IntFloat {
    type Output = IntFloat;

    fn add(self, rhs: IntFloat) -> IntFloat {
        *self + rhs
    }
}

impl AddAssign for IntFloat {
    fn add_assign(&mut self, rhs: Self) {
        let new = self.clone() + rhs;
        self.pow = new.pow;
        self.base = new.base;
    }
}

impl Zero for IntFloat {
    fn zero() -> Self {
        IntFloat { base: 0, pow: 0 }
    }

    fn is_zero(&self) -> bool {
        self.base == 0
    }
}

impl Mul<Self> for IntFloat {
    type Output = IntFloat;

    fn mul(self, rhs: Self) -> IntFloat {
        IntFloat {
            base: self.base * rhs.base,
            pow: self.pow + rhs.pow,
        }
    }
}

impl One for IntFloat {
    fn one() -> Self {
        IntFloat { base: 1, pow: 0 }
    }
}

impl Sub<Self> for IntFloat {
    type Output = IntFloat;

    fn sub(self, rhs: Self) -> IntFloat {
        let new_rhs = IntFloat {
            base: -rhs.base,
            pow: rhs.pow,
        };
        self.add(new_rhs)
    }
}

impl SubAssign for IntFloat {
    fn sub_assign(&mut self, rhs: Self) {
        let new = self.clone() - rhs;
        self.pow = new.pow;
        self.base = new.base;
    }
}

impl Div<Self> for IntFloat {
    type Output = IntFloat;

    fn div(self, rhs: Self) -> IntFloat {
        IntFloat {
            base: self.base / rhs.base,
            pow: self.pow - rhs.pow,
        }
    }
}

impl Rem<Self> for IntFloat {
    type Output = IntFloat;

    fn rem(self, rhs: Self) -> IntFloat {
        self - self.div(rhs).mul(rhs)
    }
}

impl Num for IntFloat {
    type FromStrRadixErr = ParseIntError;

    fn from_str_radix(str: &str, radix: u32) -> Result<IntFloat, ParseIntError> {
        let this_base = isize::from_str_radix(str, radix);
        let this_base = match this_base {
            Err(parse_int_error) => return Err(parse_int_error),
            Ok(num) => num,
        };
        Ok(IntFloat {
            base: this_base,
            pow: 0,
        })
    }
}

impl ToPrimitive for IntFloat {
    fn to_i64(&self) -> Option<i64> {
        Option::from((self.base as f64 * 10_f64.pow(-self.pow as f64)) as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Option::from((self.base as f64 * 10_f64.pow(-self.pow as f64)) as u64)
    }

    fn to_f32(&self) -> Option<f32> {
        Option::from(self.base as f32 * 10_f32.pow(-self.pow as f32))
    }

    fn to_f64(&self) -> Option<f64> {
        Option::from(self.base as f64 * 10_f64.pow(-self.pow as f64))
    }
}

impl std::iter::Sum for IntFloat {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut this = IntFloat::one();
        for i in iter {
            this += i;
        }
        this
    }
}

impl PartialEq for IntFloat {
    fn eq(&self, other: &Self) -> bool {
        if other.pow > self.pow {
            other.base == self.base * isize::pow(10, (other.pow - self.pow) as u32)
        } else {
            self.base == other.base * isize::pow(10, (self.pow - other.pow) as u32)
        }
    }
}

// TODO: test equality when pow is specificied differently, eg. (100,2) == (1,0)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let this = IntFloat::new(534, 2);
        assert_eq!(this.base, 534);
        assert_eq!(this.pow, 2);
    }

    #[test]
    fn test_from() {
        let this = IntFloat::from(5.34, 2);
        let that = IntFloat::new(534, 2);
        assert_eq!(this, that);

        let this = IntFloat::from(5.34234, 2);
        assert_eq!(this, that);
    }

    #[test]
    fn test_print() {
        // Accuracy of conversion will be tested in respective conversion function
        let this = IntFloat::new(534, 2);
        assert!(this.print().contains('.'));
        let this = IntFloat::new(534, 0);
        assert!(!this.print().contains('.'));
        let this = IntFloat::new(534, -2);
        assert!(!this.print().contains('.'));
    }

    #[test]
    fn test_add() {
        // Accuracy of conversion will be tested in respective conversion function
        let this = IntFloat::new(534, -2);
        let that = IntFloat::new(1068, -2);
        assert_eq!(this + this, that);

        let this = IntFloat::new(534, -2);
        let that = IntFloat::new(1068, -2);
        assert_eq!(this + this, that);

        let this = IntFloat::new(534, 0);
        let this_too = IntFloat::new(100, 2);
        let that = IntFloat::new(53500, 2);
        assert_eq!(this + this_too, that);
    }

    #[test]
    fn test_add_assign() {
        // Accuracy of conversion will be tested in respective conversion function
        let mut this = IntFloat::new(534, -2);
        this += this;
        let that = IntFloat::new(1068, -2);
        assert_eq!(this, that);
    }

    #[test]
    fn test_sub() {
        // Accuracy of conversion will be tested in respective conversion function
        let this = IntFloat::new(534, -2);
        let that = IntFloat::new(1068, -2);
        assert_eq!(that - this, this);

        let this = IntFloat::new(534, -2);
        let that = IntFloat::new(1068, -2);
        assert_eq!(that - this, this);

        let this = IntFloat::new(534, 0);
        let this_too = IntFloat::new(100, 2);
        let that = IntFloat::new(53500, 2);
        assert_eq!(that - this, this_too);
    }

    #[test]
    fn test_sub_assign() {
        // Accuracy of conversion will be tested in respective conversion function
        let this = IntFloat::new(534, -2);
        let mut that = IntFloat::new(1068, -2);
        that -= this;
        assert_eq!(this, that);
    }

    #[test]
    fn test_mul() {
        // Accuracy of conversion will be tested in respective conversion function
        let this = IntFloat::new(500, -2);
        let that = IntFloat::new(250000, -4);
        assert_eq!(this * this, that);

        let this = IntFloat::new(500, 2);
        let that = IntFloat::new(250000, 4);
        assert_eq!(this * this, that);
    }

    #[test]
    fn test_div() {
        // Accuracy of conversion will be tested in respective conversion function
        let this = IntFloat::new(500, -2);
        let that = IntFloat::new(250000, -4);
        assert_eq!(that / this, this);

        let this = IntFloat::new(500, 2);
        let that = IntFloat::new(250000, 4);
        assert_eq!(that / this, this);
    }

    #[test]
    fn test_rem() {
        // Accuracy of conversion will be tested in respective conversion function
        let this = IntFloat::new(499, -2);
        let that = IntFloat::new(250000, -4);
        let such = IntFloat::new(1, -4);
        assert_eq!(that - (that / this) * this, such);

        let this = IntFloat::new(499, 2);
        let that = IntFloat::new(250000, 4);
        let such = IntFloat::new(1, 4);
        assert_eq!(that - (that / this) * this, such);
    }

    #[test]
    fn test_to_int() {
        let this = IntFloat::new(499, -2);
        assert_eq!(this.to_i64().unwrap(), 49900);
        assert_eq!(this.to_u64().unwrap(), 49900);

        let this = IntFloat::new(499, 2);
        assert_eq!(this.to_i64().unwrap(), 4);
        assert_eq!(this.to_u64().unwrap(), 4);
    }

    #[test]
    fn test_to_float() {
        let this = IntFloat::new(499, -2);
        assert_eq!(this.to_f32().unwrap(), 49900.0);
        assert_eq!(this.to_f64().unwrap(), 49900.0);

        let this = IntFloat::new(499, 2);
        assert_eq!(this.to_f32().unwrap(), 4.99);
        assert_eq!(this.to_f64().unwrap(), 4.99);
    }

    #[test]
    fn test_partial_eq() {
        let this = IntFloat::new(50000, 2);
        let that = IntFloat::new(500, 0);
        let such = IntFloat::new(5, -2);
        assert_eq!(this, that);
        assert_eq!(such, that);
    }
}
