use core::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl core::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn element_product(&self, rhs: &Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }

    /// The perpendicular dot product of self and rhs. Also known as the wedge product, 2D cross product, and determinant.
    pub fn perp_dot(&self, rhs: &Self) -> f64 {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl core::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl core::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl core::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl core::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl core::ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        *self = self.add(rhs);
    }
}

impl core::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn element_product(&self, rhs: &Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

#[derive(Debug, PartialEq)]
pub struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl core::ops::Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Self) -> Self::Output {
        Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    pub fn element_product(&self, rhs: &Self) -> Self {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_addition() {
        let vec = Vec2::new(1.0, 2.0);
        let other_vec = Vec2::new(3.0, 4.0);

        assert_eq!(vec + other_vec, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec3_addition() {
        let vec = Vec3::new(1.0, 2.0, 1.0);
        let other_vec = Vec3::new(3.0, 4.0, 0.0);

        assert_eq!(vec + other_vec, Vec3::new(4.0, 6.0, 1.0));
    }

    #[test]
    fn test_vec4_addition() {
        let vec = Vec4::new(1.0, 2.0, 1.0, 5.0);
        let other_vec = Vec4::new(3.0, 4.0, 0.0, 3.0);

        assert_eq!(vec + other_vec, Vec4::new(4.0, 6.0, 1.0, 8.0));
    }

    #[test]
    fn test_vec3_dot_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a.dot(&b), 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0);
    }

    #[test]
    fn test_vec3_element_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a.element_product(&b);
        assert_eq!(result.x, 1.0 * 4.0);
        assert_eq!(result.y, 2.0 * 5.0);
        assert_eq!(result.z, 3.0 * 6.0);
    }

    #[test]
    fn test_vec3_cross_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let result = a.cross(&b);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0); // Right-hand rule: X cross Y = Z
    }

    #[test]
    fn test_vec4_dot_product() {
        let a = Vec4::new(1.0, 2.0, 3.0, 11.0);
        let b = Vec4::new(4.0, 5.0, 6.0, 2.0);
        assert_eq!(a.dot(&b), 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0 + 11.0 * 2.0);
    }

    #[test]
    fn test_vec4_element_product() {
        let a = Vec4::new(1.0, 2.0, 3.0, 11.0);
        let b = Vec4::new(4.0, 5.0, 6.0, 2.0);
        let result = a.element_product(&b);
        assert_eq!(result.x, 1.0 * 4.0);
        assert_eq!(result.y, 2.0 * 5.0);
        assert_eq!(result.z, 3.0 * 6.0);
        assert_eq!(result.w, 11.0 * 2.0);
    }
}
