use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    /// Calculate complex conjugate of self.
    pub fn conj(&self) -> Self {
        Self { real: self.real, imag: -self.imag }
    }

    #[allow(unused)]
    /// Calculate complex argument of self.
    pub fn arg(&self) -> f64 {
        f64::atan(self.imag / self.real)
    }

    /// Calculate modulus of a complex number.
    pub fn modulus(&self) -> f64 {
        f64::sqrt(self.real * self.real + self.imag * self.imag)
    }
}

/// Overload '+' operator so it reflects addition of complex numbers.
///
/// Overload for pass-by-value semantics.
impl ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.real + rhs.real, self.imag + rhs.imag)
    }
}

impl ops::Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.real - rhs.real, self.imag - rhs.imag)
    }
}


// (a + ib) (c + id) = ac + iad + ibc - bd = ac - bd + i(ad + bc)
impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.imag * rhs.imag, 
            self.real * rhs.imag + self.imag * rhs.real
        )
    }
}

impl ops::Div<f64> for Complex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.real / rhs, self.imag / rhs )
    }
}

/// Overload of division ( / ) operator for Complex struct.
impl ops::Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denum = rhs.real * rhs.real + rhs.imag * rhs.imag;
        self * rhs.conj() / denum
    }
}

#[cfg(test)]
mod tests {
    use crate::complex::Complex;

    // I use the following macro to test operator overloads implemented above.
    // Simple equality check on floating point numbers can yield false results.
    macro_rules! assert_almost_eq {
        ($a: expr, $b: expr) => {
            assert!(($a - $b).abs() <= 1e-10)
        };
        ($a: expr, $b: expr, $eps: expr) => {
            assert!(($a - $b).abs() <= $eps)
        };
    }

    #[test]
    fn test_addition_1() {
        let (c1, c2) = (Complex::new(1.0, 1.0), Complex::new(2.0, 2.0));
        let sum = c1 + c2;
        assert_almost_eq!(sum.real, 3.0);
        assert_almost_eq!(sum.imag, 3.0);
    }

    #[test]
    fn test_addition_2() {
        let (c1, c2) = (Complex::new(1.0, 1.0), Complex::new(-2.0, -2.0));
        let sum = c1 + c2;
        assert_almost_eq!(sum.real, -1.0);
        assert_almost_eq!(sum.imag, -1.0);
    }

    #[test]
    fn test_addition_3() {
        let (c1, c2) = (Complex::new(1.0, 1.0), Complex::new(0.0, 0.0));
        let sum = c1 + c2;
        assert_almost_eq!(sum.real, 1.0);
        assert_almost_eq!(sum.imag, 1.0);
    }

    #[test]
    fn test_subtraction_1() {
        let (c1, c2) = (Complex::new(1.0, 1.0), Complex::new(2.0, 2.0));
        let sum = c1 - c2;
        assert_almost_eq!(sum.real, -1.0);
        assert_almost_eq!(sum.imag, -1.0);
    }

    #[test]
    fn test_subtraction_2() {
        let (c1, c2) = (Complex::new(1.0, 1.0), Complex::new(-2.0, -2.0));
        let sum = c1 - c2;
        assert_almost_eq!(sum.real, 3.0);
        assert_almost_eq!(sum.imag, 3.0);
    }

    #[test]
    fn test_subtraction_3() {
        let (c1, c2) = (Complex::new(1.0, 1.0), Complex::new(0.0, 0.0));
        let sum = c1 - c2;
        assert_almost_eq!(sum.real, 1.0);
        assert_almost_eq!(sum.imag, 1.0);
    }

    #[test]
    fn test_multiplication_1() {
        let (c1, c2) = (Complex::new(1.0, 0.0), Complex::new(4.0, 0.0));
        let mul = c1 * c2;
        assert_almost_eq!(mul.real, 4.0);
        assert_almost_eq!(mul.imag, 0.0);
    }


    // ac - bd + i(ad + bc)
    #[test]
    fn test_multiplication_2() {
        let (c1, c2) = (Complex::new(1.0, 1.0), Complex::new(1.0, 2.0));
        let mul = c1 * c2;
        assert_almost_eq!(mul.real, -1.0);
        assert_almost_eq!(mul.imag, 3.0);
    }

    #[test]
    fn test_multiplication_3() {
        let (c1, c2) = (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));
        let mul = c1 * c2;
        assert_almost_eq!(mul.real, 0.0);
        assert_almost_eq!(mul.imag, 0.0);
    }

    // TODO: add tests for division, conjugate, argument and module.
}


