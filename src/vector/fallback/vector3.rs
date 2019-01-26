#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    values: [f32; 3],
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{values: [x, y, z, 0.0]}
    }
    pub fn zero() -> Vector3 {
        Vector3{values: [0.0, 0.0, 0.0]}
    }

    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
    }
    pub fn len_sq(self) -> f32 {
        self.dot(self)
    }
    pub fn dot(self, other: Vector3) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3::new(self[1] * other[2] - self[2] * other[1],
                     self[2] * other[0] - self[0] * other[2],
                     self[0] * other[1] - self[1] * other[0])
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
        Vector3::new(-self[0], -self[1], -self[2])
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Vector3::new(self[0]+other[0], self[1]+other[1], self[2]+other[2])
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        Vector3::new(self[0]-other[0], self[1]-other[1], self[2]-other[2])
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Self::Output {
        Vector3::new(self[0]*other, self[1]*other, self[2]*other)
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        self[0] *= other;
        self[1] *= other;
        self[2] *= other;
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Self::Output {
        Vector3::new(self[0]/other, self[1]/other, self[2]/other)
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        self[0] /= other;
        self[1] /= other;
        self[2] /= other;
    }
}
