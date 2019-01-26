#[cfg(target_feature = "sse")]
pub mod x86_64_sse;

#[cfg(target_feature = "sse")]
pub use self::x86_64_sse::*;

#[cfg(not(target_feature = "sse"))]
pub mod fallback;

#[cfg(not(target_feature = "sse"))]
pub use self::fallback::*;

#[cfg(test)]
mod tests {
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
