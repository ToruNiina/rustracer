use std::arch::x86_64::_mm_load_ps;
use std::arch::x86_64::_mm_store_ps;
use std::arch::x86_64::_mm_set1_ps;
use std::arch::x86_64::_mm_add_ps;
use std::arch::x86_64::_mm_sub_ps;
use std::arch::x86_64::_mm_mul_ps;
use std::arch::x86_64::_mm_div_ps;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(align(16))]
pub struct Vector3 {
    /// the last item is for padding
    values: [f32; 4],
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{values: [x, y, z, 0.0]}
    }

    pub(crate) fn as_ptr(&self) -> *const f32 {
        self.values.as_ptr()
    }
    pub(crate) fn as_mut_ptr(&mut self) -> *mut f32 {
        self.values.as_mut_ptr()
    }
}

impl std::ops::Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.values[idx]
    }
}

impl std::ops::IndexMut<usize> for Vector3 {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {
        &mut self.values[idx]
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        unsafe {
            let mut retval = Vector3::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(-1.0)));
            return retval
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        unsafe {
            let mut retval = Vector3::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_add_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
            return retval
        }
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_add_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        unsafe {
            let mut retval = Vector3::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_sub_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
            return retval
        }
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_sub_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Self::Output {
        unsafe {
            let mut retval = Vector3::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
            return retval
        }
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
        }
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Self::Output {
        unsafe {
            let mut retval = Vector3::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.as_mut_ptr(), _mm_div_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
            return retval
        }
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        unsafe {
            _mm_store_ps(self.as_mut_ptr(), _mm_div_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
        }
    }
}
