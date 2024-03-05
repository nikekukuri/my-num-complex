use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex<T> {
    re: T,
    im: T,
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;

impl<T> Complex<T> {
    pub const fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

// (a + i b) + (c + i d) == (a + c) + i (b + d)
impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.re + rhs.re, self.im + rhs.im)
    }
}

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl<T: Sub<Output = T>> Sub for Complex<T> 
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.re - rhs.re, self.im - rhs.im)
    }
}

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl<T> Mul<Complex<T>> for Complex<T> 
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Sub<Output = T> 
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let re = self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone();
        let im = self.im * rhs.re + self.re * rhs.im;
        Self::Output::new(re, im)
    }
}

/// TODO: Rerun Error Code if denominator == 0
// (a + i b) / (c + i d) == [(a + i b) * (c - i d)] / (c*c + d*d)
//   == [(a*c + b*d) / (c*c + d*d)] + i [(b*c - a*d) / (c*c + d*d)]
impl<T> Div for Complex<T> 
where
    T: Clone + Div<Output = T> + Mul<Output = T> + Add<Output = T> + Sub<Output = T> 
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let deno = rhs.re.clone() * rhs.re.clone() + rhs.im.clone() * rhs.im.clone(); // Denominator
        let re_tmp = self.re.clone() * rhs.re.clone() + self.im.clone() * rhs.im.clone();
        let im_tmp = self.im * rhs.re - self.re * rhs.im;
        Self::Output::new(re_tmp / deno.clone(), im_tmp / deno)
    }
}

trait Norm {
    fn norm(&self) -> f64;
}

trait Arg {
    fn arg(&self) -> f64;
}

impl Norm for Complex64 {
    fn norm(&self) -> f64 { 
        let t1 = self.re * self.re;
        let t2 = self.im * self.im;
        let pow = t1 + t2;
        pow.sqrt()
    }
}

impl Arg for Complex64 {
    fn arg(&self) -> f64 { 
        self.im.atan2(self.re)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        let result = Complex64{ re: 1.0, im: 2.0 };
        assert_eq!(result.re, 1.0);
        assert_eq!(result.im, 2.0);
    }

    #[test]
    fn test_add() {
        let c1 = Complex64{ re: 1.0, im: 2.0 };
        let c2 = Complex64{ re: 2.0, im: 3.0 };
        let result = c1 + c2;
        assert_eq!(result.re, 3.0);
        assert_eq!(result.im, 5.0);
    }

    #[test]
    fn test_sub() {
        let c1 = Complex64{ re: 1.0, im: 2.0 };
        let c2 = Complex64{ re: 2.0, im: 3.0 };
        let result = c2 - c1;
        assert_eq!(result.re, 1.0);
        assert_eq!(result.im, 1.0);
    }

    #[test]
    fn test_mul() {
        let c1 = Complex64{ re: 1.0, im: 2.0 };
        let c2 = Complex64{ re: 2.0, im: 3.0 };
        let result = c2 * c1;
        assert_eq!(result.re, -4.0);
        assert_eq!(result.im, 7.0);

        let c1 = Complex64{ re: -1.0, im: 2.0 };
        let c2 = Complex64{ re: 2.0, im: 3.0 };
        let result = c2 * c1;
        assert_eq!(result.re, -8.0);
        assert_eq!(result.im, 1.0);
    }

    #[test]
    fn test_div() {
        let c1 = Complex64{ re: 1.0, im: 2.0 };
        let c2 = Complex64{ re: 2.0, im: 3.0 };
        let result = c1 / c2;
        assert_eq!(result.re, 8.0 / 13.0);
        assert_eq!(result.im, 1.0 / 13.0);
    }

    #[test]
    fn test_norm() {
        let c1 = Complex64{ re: 1.0, im: 2.0 };
        let expect_tmp: f64 = 1.0 * 1.0 + 2.0 * 2.0;
        let expect: f64 = expect_tmp.sqrt();
        assert_eq!(c1.norm(), expect);
    }

    #[test]
    fn test_arg() {
        let c1 = Complex64{ re: 1.0, im: 2.0 };
        let expect_tmp: f64 = 2.0;
        let expect: f64 = expect_tmp.atan2(1.0);
        assert_eq!(c1.arg(), expect);
    }
}
