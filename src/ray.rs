use crate::vector::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    origin:    Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray{origin, direction}
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
        let dir = Vector3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(ori, dir);
        let p   = ray.at(0.5);
        assert_eq!(p[0], 1.5);
        assert_eq!(p[1], 2.0);
        assert_eq!(p[2], 2.5);
    }
}
