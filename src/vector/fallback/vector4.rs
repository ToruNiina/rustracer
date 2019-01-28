#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4 {
    values: [f32; 4],
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector3{values: [x, y, z, w]}
    }
    pub fn zero() -> Vector4 {
        Vector4{values: [0.0, 0.0, 0.0, 0.0]}
    }
}

impl std::ops::Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.values[idx]
    }
}

impl std::ops::IndexMut<usize> for Vector4 {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {
        &mut self.values[idx]
    }
}

impl std::ops::Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Vector4::new(-self[0], -self[1], -self[2], -self[3])
    }
}

impl std::ops::Add for Vector4 {
    type Output = Vector4;

    fn add(self, other: Vector4) -> Self::Output {
        Vector4::new(self[0]+other[0], self[1]+other[1],
                     self[2]+other[2], self[3]+other[3])
    }
}

impl std::ops::AddAssign for Vector4 {
    fn add_assign(&mut self, other: Vector4) {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
        self[3] += other[3];
    }
}

impl std::ops::Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, other: Vector4) -> Self::Output {
        Vector4::new(self[0]-other[0], self[1]-other[1],
                     self[2]-other[2], self[3]-other[3])
    }
}

impl std::ops::SubAssign for Vector4 {
    fn sub_assign(&mut self, other: Vector4) {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
        self[3] -= other[3];
    }
}

impl std::ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, other: f32) -> Self::Output {
        Vector4::new(self[0]*other, self[1]*other, self[2]*other, self[3]*other)
    }
}
impl std::ops::Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, other: Vector4) -> Self::Output {
        Vector4::new(self*other[0], self*other[1], self*other[2], self*other[3])
    }
}

impl std::ops::MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, other: f32) {
        self[0] *= other;
        self[1] *= other;
        self[2] *= other;
        self[3] *= other;
    }
}

impl std::ops::Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, other: f32) -> Self::Output {
        Vector3::new(self[0]/other, self[1]/other, self[2]/other, self[3]/other)
    }
}

impl std::ops::DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, other: f32) {
        self[0] /= other;
        self[1] /= other;
        self[2] /= other;
        self[3] /= other;
    }
}
