use std::arch::x86_64::_mm_load_ps;
use std::arch::x86_64::_mm_store_ps;
use std::arch::x86_64::_mm_set1_ps;
use std::arch::x86_64::_mm_set_ps;
use std::arch::x86_64::_mm_add_ps;
use std::arch::x86_64::_mm_sub_ps;
use std::arch::x86_64::_mm_mul_ps;
use std::arch::x86_64::_mm_div_ps;

use std::arch::x86_64::_mm_set_ss;
use std::arch::x86_64::_mm_rsqrt_ss;
use std::arch::x86_64::_mm_cvtss_f32;

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
    pub fn zero() -> Vector3 {
        Vector3{values: [0.0, 0.0, 0.0, 0.0]}
    }
    pub(crate) fn as_ptr(&self) -> *const f32 {
        self.values.as_ptr()
    }
    pub(crate) fn as_mut_ptr(&mut self) -> *mut f32 {
        self.values.as_mut_ptr()
    }

    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
//         XXX this code causes wierd image.
//         let lsq = self.len_sq();
//         unsafe {
//             lsq * _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(lsq)))
//         }
    }
    pub fn len_sq(self) -> f32 {
        self.dot(self)
    }
    pub fn rlen(self) -> f32 {
        1.0 / self.len()
//         XXX this code causes wierd image.
//         let lsq = self.len_sq();
//         unsafe {
//             _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(lsq)))
//         }
    }
    pub fn unit(self) -> Vector3 {
        self * self.rlen()
    }
    pub fn dot(self, other: Vector3) -> f32 {
        unsafe {
            let mut retval = Vector3::zero();
            _mm_store_ps(retval.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_load_ps(other.as_ptr())));
            retval[0] + retval[1] + retval[2]
        }
    }
    pub fn cross(self, other: Vector3) -> Vector3 {
         unsafe {
            let mut v1 = Vector3::zero();
            let mut v2 = Vector3::zero();
            _mm_store_ps(v1.as_mut_ptr(), _mm_mul_ps(
                _mm_set_ps(0.0,  self[0],  self[2],  self[1]),
                _mm_set_ps(0.0, other[1], other[0], other[2])));
            _mm_store_ps(v2.as_mut_ptr(), _mm_mul_ps(
                _mm_set_ps(0.0,  self[1],  self[0],  self[2]),
                _mm_set_ps(0.0, other[0], other[2], other[1])));
            v1 - v2
        }
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
            let mut retval = Vector3::zero();
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
            let mut retval = Vector3::zero();
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
            let mut retval = Vector3::zero();
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
            let mut retval = Vector3::zero();
            _mm_store_ps(retval.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(self.as_ptr()), _mm_set1_ps(other)));
            return retval
        }
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        unsafe {
            let mut retval = Vector3::zero();
            _mm_store_ps(retval.as_mut_ptr(), _mm_mul_ps(
                _mm_load_ps(other.as_ptr()), _mm_set1_ps(self)));
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
            let mut retval = Vector3::zero();
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
