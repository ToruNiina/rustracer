#[cfg(target_feature = "sse")]
pub mod x86_64_sse;

#[cfg(target_feature = "sse")]
pub use self::x86_64_sse::*;

#[cfg(not(target_feature = "sse"))]
pub mod fallback;

#[cfg(not(target_feature = "sse"))]
pub use self::fallback::*;

use rand::distributions::Distribution;
use rand::Rng;

pub fn pick_in_sphere(rng: &mut rand::rngs::ThreadRng) -> Vector3 {
    let u = rng.gen_range(0.0f32, 1.0f32);
    let normal = rand::distributions::StandardNormal;
    Vector3::unit(Vector3::new(normal.sample(&mut *rng) as f32,
                               normal.sample(&mut *rng) as f32,
                               normal.sample(&mut *rng) as f32)) * u.cbrt()
}

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * Vector3::dot(v, n) * n
}
/// Snell's law
pub fn refract(v: Vector3, n: Vector3, ni_over_nt: f32) -> std::option::Option<Vector3> {
    assert!((n.len() - 1.0).abs() < 1e-4);
    let uv = v.unit();
    let dt = Vector3::dot(uv, n);
    let d  = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * d.sqrt());
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::f32;
    use crate::vector::*;
    #[test]
    fn index_3() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }
    #[test]
    fn index_mut_3() {
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v[0] += 10.0;
        v[1] += 10.0;
        v[2] += 10.0;
        assert_eq!(v[0], 11.0);
        assert_eq!(v[1], 12.0);
        assert_eq!(v[2], 13.0);
    }

    #[test]
    fn add_3() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(2.0, 3.0, 4.0);
        let w = u + v;
        assert_eq!(w[0], 3.0);
        assert_eq!(w[1], 5.0);
        assert_eq!(w[2], 7.0);
    }
    #[test]
    fn add_assign_3() {
        let mut u = Vector3::new(1.0, 2.0, 3.0);
        let v     = Vector3::new(10., 10., 10.);
        u += v;
        assert_eq!(u[0], 11.0);
        assert_eq!(u[1], 12.0);
        assert_eq!(u[2], 13.0);
    }

    #[test]
    fn sub_3() {
        let u = Vector3::new(2.0, 3.0, 4.0);
        let v = Vector3::new(1.0, 2.0, 3.0);
        let w = u - v;
        assert_eq!(w[0], 1.0);
        assert_eq!(w[1], 1.0);
        assert_eq!(w[2], 1.0);
    }
    #[test]
    fn sub_assign_3() {
        let mut u = Vector3::new(10., 10., 10.);
        let v     = Vector3::new(1.0, 2.0, 3.0);
        u -= v;
        assert_eq!(u[0], 9.0);
        assert_eq!(u[1], 8.0);
        assert_eq!(u[2], 7.0);
    }

    #[test]
    fn mul_3() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = 2.0;
        let w = u * v;
        assert_eq!(w[0], 2.0);
        assert_eq!(w[1], 4.0);
        assert_eq!(w[2], 6.0);
    }
    #[test]
    fn mul_assign_3() {
        let mut u = Vector3::new(1.0, 2.0, 3.0);
        let v     = 2.0;
        u *= v;
        assert_eq!(u[0], 2.0);
        assert_eq!(u[1], 4.0);
        assert_eq!(u[2], 6.0);
    }

    #[test]
    fn div_3() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = 2.0;
        let w = u / v;
        assert_eq!(w[0], 0.5);
        assert_eq!(w[1], 1.0);
        assert_eq!(w[2], 1.5);
    }
    #[test]
    fn div_assign_3() {
        let mut u = Vector3::new(1.0, 2.0, 3.0);
        let v     = 2.0;
        u /= v;
        assert_eq!(u[0], 0.5);
        assert_eq!(u[1], 1.0);
        assert_eq!(u[2], 1.5);
    }

    #[test]
    fn len() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let l = Vector3::len(u);
        assert_eq!(l, (1.0f32 + 4.0f32 + 9.0f32).sqrt());
    }
    #[test]
    fn len_sq() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let l = Vector3::len_sq(u);
        assert_eq!(l, 1.0 + 4.0 + 9.0);
    }

    #[test]
    fn dot() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(2.0, 3.0, 4.0);
        let d = Vector3::dot(u, v);
        assert_eq!(d, 2.0 + 6.0 + 12.0);
    }
    #[test]
    fn cross() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(2.0, 3.0, 4.0);
        let c = Vector3::cross(u, v);
        let d1 = Vector3::dot(u, c);
        let d2 = Vector3::dot(v, c);
        assert_eq!(d1, 0.0);
        assert_eq!(d2, 0.0);
    }
    #[test]
    fn unit() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let n = Vector3::unit(u);
        assert!((n.len() - 1.0).abs() < 1e-4 );
    }

    #[test]
    fn index_4() {
        let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
        assert_eq!(v[3], 4.0);
    }
    #[test]
    fn index_mut_4() {
        let mut v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        v[0] += 10.0;
        v[1] += 10.0;
        v[2] += 10.0;
        v[3] += 10.0;
        assert_eq!(v[0], 11.0);
        assert_eq!(v[1], 12.0);
        assert_eq!(v[2], 13.0);
        assert_eq!(v[3], 14.0);
    }

    #[test]
    fn add_4() {
        let u = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v = Vector4::new(2.0, 3.0, 4.0, 5.0);
        let w = u + v;
        assert_eq!(w[0], 3.0);
        assert_eq!(w[1], 5.0);
        assert_eq!(w[2], 7.0);
        assert_eq!(w[3], 9.0);
    }
    #[test]
    fn add_assign_4() {
        let mut u = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v     = Vector4::new(10., 10., 10., 10.);
        u += v;
        assert_eq!(u[0], 11.0);
        assert_eq!(u[1], 12.0);
        assert_eq!(u[2], 13.0);
        assert_eq!(u[3], 14.0);
    }

    #[test]
    fn sub_4() {
        let u = Vector4::new(2.0, 3.0, 4.0, 5.0);
        let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let w = u - v;
        assert_eq!(w[0], 1.0);
        assert_eq!(w[1], 1.0);
        assert_eq!(w[2], 1.0);
        assert_eq!(w[3], 1.0);
    }
    #[test]
    fn sub_assign_4() {
        let mut u = Vector4::new(10., 10., 10., 10.);
        let v     = Vector4::new(1.0, 2.0, 3.0, 4.0);
        u -= v;
        assert_eq!(u[0], 9.0);
        assert_eq!(u[1], 8.0);
        assert_eq!(u[2], 7.0);
        assert_eq!(u[3], 6.0);
    }

    #[test]
    fn mul_4() {
        let u = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v = 2.0;
        let w = u * v;
        assert_eq!(w[0], 2.0);
        assert_eq!(w[1], 4.0);
        assert_eq!(w[2], 6.0);
        assert_eq!(w[3], 8.0);
    }
    #[test]
    fn mul_assign_4() {
        let mut u = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v     = 2.0;
        u *= v;
        assert_eq!(u[0], 2.0);
        assert_eq!(u[1], 4.0);
        assert_eq!(u[2], 6.0);
        assert_eq!(u[3], 8.0);
    }

    #[test]
    fn div_4() {
        let u = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v = 2.0;
        let w = u / v;
        assert_eq!(w[0], 0.5);
        assert_eq!(w[1], 1.0);
        assert_eq!(w[2], 1.5);
        assert_eq!(w[3], 2.0);
    }
    #[test]
    fn div_assign_4() {
        let mut u = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v     = 2.0;
        u /= v;
        assert_eq!(u[0], 0.5);
        assert_eq!(u[1], 1.0);
        assert_eq!(u[2], 1.5);
        assert_eq!(u[3], 2.0);
    }
}
