use crate::vector::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub origin:    Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray{origin, direction: Vector3::unit(direction)}
    }
    pub fn at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::*;
    #[test]
    fn point_at() {
        let ori = Vector3::new(1.0, 1.0, 1.0);
        let dir = Vector3::new(1.0, 2.0, 3.0).unit();
        let ray = Ray::new(ori, dir);
        let p   = ray.at(0.5);
        assert!((p[0] - (ori[0] + dir[0] * 0.5)).abs() < 3.0 / 4096.0);
        assert!((p[1] - (ori[1] + dir[1] * 0.5)).abs() < 3.0 / 4096.0);
        assert!((p[2] - (ori[2] + dir[2] * 0.5)).abs() < 3.0 / 4096.0);
    }
}
