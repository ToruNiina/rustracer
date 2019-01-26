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
}

impl std::ops::Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.values[idx]
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        use std::arch::x86_64::_mm_load_ps;
        use std::arch::x86_64::_mm_store_ps;
        use std::arch::x86_64::_mm_add_ps;
        unsafe {
            let mut retval = Vector3::new(0.0, 0.0, 0.0);
            _mm_store_ps(retval.values.as_mut_ptr(), _mm_add_ps(
                _mm_load_ps(self.values.as_ptr()),
                _mm_load_ps(other.values.as_ptr())));
            return retval
        }
    }
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "sse")))]
impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Vector3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}
