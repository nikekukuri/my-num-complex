use std::ops::{Add, Sub, Mul, Div};
use std::f64::consts::PI;

#[derive(Debug)]
struct Complex64{
    re: f64,
    im: f64,
}

impl Add for Complex64 {
    type Output = Self;
    fn add(self, rhs: Self) -> <Self as Add>::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> <Self as Sub>::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> <Self as Mul>::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

/// TODO: Rerun Error Code if denominator == 0
impl Div for Complex64 {
    type Output = Self;
    fn div(self, rhs: Self) -> <Self as Div>::Output {
        let deno = rhs.re * rhs.re + rhs.im * rhs.im;
        let re_tmp = self.re * rhs.re + self.im * rhs.im;
        let im_tmp = self.im * rhs.re - self.re * rhs.im;
        Self {
            re: re_tmp / deno,
            im: im_tmp / deno,
        }
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
