//! RGB/RGBA colors.
//!
//! It contains color by `f32`.
//! It supports Add, Mul, Div for RGB. Alpha blending is not supported.
//! RGB acts as 3D Vector, but Mul is implemented as Hadamard product.

pub trait Color {
    type Value;

    fn r(&self) -> Self::Value;
    fn g(&self) -> Self::Value;
    fn b(&self) -> Self::Value;
    fn a(&self) -> Self::Value;

    fn set_r(&mut self, v: Self::Value);
    fn set_g(&mut self, v: Self::Value);
    fn set_b(&mut self, v: Self::Value);
    fn set_a(&mut self, v: Self::Value);
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(align(16))]
pub struct RGBA {
    colors: [f32; 4],
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(align(16))]
pub struct RGB  {
    colors: [f32; 4], // the last element is for padding
}

impl RGBA {
    /// constructs a new RGBA color.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        RGBA {colors: [r, g, b, a]}
    }

    /// extracts RGB part. Note that it does not perform alpha blending.
    pub fn rgb(&self) -> RGB {
        RGB::new(self.r(), self.g(), self.b())
    }
}

impl RGB {
    /// construct new RGB color.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        RGB {colors: [r, g, b, 0.0]}
    }

    // private method to use SIMD is possible
    fn as_ptr(&self) -> *const f32 {
        self.colors.as_ptr()
    }
    // private method to use SIMD is possible
    fn as_mut_ptr(&mut self) -> *mut f32 {
        self.colors.as_mut_ptr()
    }
}

impl std::convert::From<RGBA> for RGB {
    /// convert RGBA image into RGB by blending with the RGB white
    fn from(rgba: RGBA) -> RGB {
        let s = rgba.a();
        let t = 1.0 - s;
        RGB::new(rgba.r(), rgba.g(), rgba.b()) * s + RGB::new(1.0, 1.0, 1.0) * t
    }
}
impl std::convert::From<RGB> for RGBA {
    /// convert RGB image into RGBA assuming alpha = 1.0
    fn from(rgb: RGB) -> RGBA {
        RGBA::new(rgb.colors[0], rgb.colors[1], rgb.colors[2], 1.0)
    }
}

impl Color for RGB {
    type Value = f32;

    fn r(&self) -> Self::Value {self.colors[0]}
    fn g(&self) -> Self::Value {self.colors[1]}
    fn b(&self) -> Self::Value {self.colors[2]}
    fn a(&self) -> Self::Value {1.0}

    fn set_r(&mut self, v: Self::Value) {self.colors[0] = v}
    fn set_g(&mut self, v: Self::Value) {self.colors[1] = v}
    fn set_b(&mut self, v: Self::Value) {self.colors[2] = v}
    fn set_a(&mut self, _: Self::Value) {}
}

impl Color for RGBA {
    type Value = f32;

    fn r(&self) -> Self::Value {self.colors[0]}
    fn g(&self) -> Self::Value {self.colors[1]}
    fn b(&self) -> Self::Value {self.colors[2]}
    fn a(&self) -> Self::Value {self.colors[3]}

    fn set_r(&mut self, v: Self::Value) {self.colors[0] = v}
    fn set_g(&mut self, v: Self::Value) {self.colors[1] = v}
    fn set_b(&mut self, v: Self::Value) {self.colors[2] = v}
    fn set_a(&mut self, v: Self::Value) {self.colors[3] = v}
}

// --------------------------------------------------------------------------
// operators for RGB. they behave as a 3D vector.
// For RGB * RGB case, it performs Hadamard product.
// Subtracts are not supported.
// --------------------------------------------------------------------------

impl std::ops::Add for RGB {
    type Output = RGB;

    #[cfg(target_feature = "sse")]
    fn add(self, other: RGB) -> RGB {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_add_ps;
        unsafe {
            let mut retval = RGB::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_add_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
            return retval;
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn add(self, other: RGB) -> RGB {
        RGB::new(self.r() + other.r(),
                 self.g() + other.g(),
                 self.b() + other.b())
    }
}

impl std::ops::AddAssign for RGB {
    #[cfg(target_feature = "sse")]
    fn add_assign(&mut self, other: RGB) {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_add_ps;
        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_add_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn add_assign(&mut self, other: RGB) {
        self.colors[0] += other.r();
        self.colors[1] += other.g();
        self.colors[2] += other.b();
    }
}

impl std::ops::Mul<f32> for RGB {
    type Output = RGB;

    #[cfg(target_feature = "sse")]
    fn mul(self, other: f32) -> RGB {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_set1_ps;
        use std::arch::x86_64::_mm_mul_ps;

        unsafe {
            let mut retval = RGB::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
            return retval
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn mul(self, other: f32) -> RGB {
        RGB::new(self.r() * other,
                 self.g() * other,
                 self.b() * other)
    }
}

impl std::ops::Mul<RGB> for f32 {
    type Output = RGB;

    #[cfg(target_feature = "sse")]
    fn mul(self, other: RGB) -> RGB {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_set1_ps;
        use std::arch::x86_64::_mm_mul_ps;

        unsafe {
            let mut retval = RGB::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_mul_ps(
                _mm_set1_ps(self), _mm_load_ps(other.as_ptr())));
            return retval
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn mul(self, other: RGB) -> RGB {
        RGB::new(self * other.r(),
                 self * other.g(),
                 self * other.b())
    }
}

impl std::ops::Mul<RGB> for RGB {
    type Output = RGB;

    #[cfg(target_feature = "sse")]
    fn mul(self, other: RGB) -> RGB {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_mul_ps;

        unsafe {
            let mut retval = RGB::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
            return retval
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn mul(self, other: RGB) -> RGB {
        RGB::new(self.r() * other.r(),
                 self.g() * other.g(),
                 self.b() * other.b())
    }
}

impl std::ops::MulAssign<f32> for RGB {
    #[cfg(target_feature = "sse")]
    fn mul_assign(&mut self, other: f32) {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_set1_ps;
        use std::arch::x86_64::_mm_mul_ps;

        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn mul_assign(&mut self, other: f32) {
        self.colors[0] *= other;
        self.colors[1] *= other;
        self.colors[2] *= other;
    }
}

impl std::ops::MulAssign<RGB> for RGB {
    #[cfg(target_feature = "sse")]
    fn mul_assign(&mut self, other: RGB) {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_mul_ps;

        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn mul_assign(&mut self, other: RGB) {
        self.colors[0] *= other.r();
        self.colors[1] *= other.g();
        self.colors[2] *= other.b();
    }
}

impl std::ops::Div<f32> for RGB {
    type Output = RGB;

    #[cfg(target_feature = "sse")]
    fn div(self, other: f32) -> RGB {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_set1_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_div_ps;

        unsafe {
            let mut retval = RGB::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_div_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
            return retval
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn div(self, other: f32) -> RGB {
        RGB::new(self.r() / other,
                 self.g() / other,
                 self.b() / other)
    }
}

impl std::ops::Div<RGB> for RGB {
    type Output = RGB;

    #[cfg(target_feature = "sse")]
    fn div(self, other: RGB) -> RGB {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_div_ps;
        unsafe {
            let mut retval = RGB::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_div_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
            return retval
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn div(self, other: RGB) -> RGB {
        RGB::new(self.r() / other.r(),
                 self.g() / other.g(),
                 self.b() / other.b())
    }
}

impl std::ops::DivAssign<f32> for RGB {
    #[cfg(target_feature = "sse")]
    fn div_assign(&mut self, other: f32) {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_set1_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_div_ps;

        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_div_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn div_assign(&mut self, other: f32) {
        self.colors[0] /= other;
        self.colors[1] /= other;
        self.colors[2] /= other;
    }
}

impl std::ops::DivAssign<RGB> for RGB {
    #[cfg(target_feature = "sse")]
    fn div_assign(&mut self, other: RGB) {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_div_ps;

        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_div_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
        }
    }

    #[cfg(not(target_feature = "sse"))]
    fn div_assign(&mut self, other: RGB) {
        self.colors[0] /= other.r();
        self.colors[1] /= other.g();
        self.colors[2] /= other.b();
    }
}

#[cfg(test)]
mod tests {
    use crate::color::*;

    #[test]
    fn add() {
        let u = RGB::new(1.0, 2.0, 3.0);
        let v = RGB::new(2.0, 3.0, 4.0);
        let w = u + v;
        assert_eq!(w.r(), 1.0 + 2.0);
        assert_eq!(w.g(), 2.0 + 3.0);
        assert_eq!(w.b(), 3.0 + 4.0);
    }
    #[test]
    fn add_assign() {
        let mut u = RGB::new(1.0, 2.0, 3.0);
        let v     = RGB::new(10., 10., 10.);
        u += v;
        assert_eq!(u.r(), 1.0 + 10.0);
        assert_eq!(u.g(), 2.0 + 10.0);
        assert_eq!(u.b(), 3.0 + 10.0);
    }

    #[test]
    fn mul() {
        {
            let u = RGB::new(1.0, 2.0, 3.0);
            let v = 2.0;
            let w = u * v;
            assert_eq!(w.r(), 1.0 * 2.0);
            assert_eq!(w.g(), 2.0 * 2.0);
            assert_eq!(w.b(), 3.0 * 2.0);
        }
        {
            let u = RGB::new(1.0, 2.0, 3.0);
            let v = RGB::new(2.0, 3.0, 4.0);
            let w = u * v;
            assert_eq!(w.r(), 1.0 * 2.0);
            assert_eq!(w.g(), 2.0 * 3.0);
            assert_eq!(w.b(), 3.0 * 4.0);
        }
    }
    #[test]
    fn mul_assign() {
        {
            let mut u = RGB::new(1.0, 2.0, 3.0);
            let v     = 2.0;
            u *= v;
            assert_eq!(u.r(), 1.0 * 2.0);
            assert_eq!(u.g(), 2.0 * 2.0);
            assert_eq!(u.b(), 3.0 * 2.0);
        }
        {
            let mut u = RGB::new(1.0, 2.0, 3.0);
            let v     = RGB::new(10., 10., 10.);
            u *= v;
            assert_eq!(u.r(), 1.0 * 10.0);
            assert_eq!(u.g(), 2.0 * 10.0);
            assert_eq!(u.b(), 3.0 * 10.0);
        }
    }

    #[test]
    fn div() {
        {
            let u = RGB::new(1.0, 2.0, 3.0);
            let v = 2.0;
            let w = u / v;
            assert_eq!(w.r(), 1.0 / 2.0);
            assert_eq!(w.g(), 2.0 / 2.0);
            assert_eq!(w.b(), 3.0 / 2.0);
        }
        {
            let u = RGB::new(1.0, 2.0, 3.0);
            let v = RGB::new(2.0, 3.0, 4.0);
            let w = u / v;
            assert_eq!(w.r(), 1.0 / 2.0);
            assert_eq!(w.g(), 2.0 / 3.0);
            assert_eq!(w.b(), 3.0 / 4.0);
        }
    }
    #[test]
    fn div_assign() {
        {
            let mut u = RGB::new(1.0, 2.0, 3.0);
            let v     = 2.0;
            u /= v;
            assert_eq!(u.r(), 1.0 / 2.0);
            assert_eq!(u.g(), 2.0 / 2.0);
            assert_eq!(u.b(), 3.0 / 2.0);
        }
        {
            let mut u = RGB::new(1.0, 2.0, 3.0);
            let v     = RGB::new(10., 10., 10.);
            u /= v;
            assert_eq!(u.r(), 1.0 / 10.0);
            assert_eq!(u.g(), 2.0 / 10.0);
            assert_eq!(u.b(), 3.0 / 10.0);
        }
    }
}
