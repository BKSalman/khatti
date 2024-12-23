use crate::vectors::Vec3;
use core::ops::Mul;

/// A 3x3 column-major matrix
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    x_axis: Vec3,
    y_axis: Vec3,
    z_axis: Vec3,
}

impl core::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut res = self.x_axis.mul(rhs.x);
        res += self.y_axis.mul(rhs.y);
        res += self.z_axis.mul(rhs.z);
        res
    }
}

impl Mat3 {
    pub const fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    pub fn add_mat3(&self, rhs: &Mat3) -> Mat3 {
        Mat3::from_cols(
            self.x_axis + rhs.x_axis,
            self.y_axis + rhs.y_axis,
            self.z_axis + rhs.z_axis,
        )
    }

    pub fn mul_mat3(&self, rhs: &Mat3) -> Mat3 {
        Self::from_cols(
            self.mul(rhs.x_axis),
            self.mul(rhs.y_axis),
            self.mul(rhs.z_axis),
        )
    }
}

/// A 4x4 column-major matrix
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    x_axis: Vec3,
    y_axis: Vec3,
    z_axis: Vec3,
    w_axis: Vec3,
}

impl core::ops::Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut res = self.x_axis.mul(rhs.x);
        res += self.y_axis.mul(rhs.y);
        res += self.z_axis.mul(rhs.z);
        res
    }
}

impl Mat4 {
    pub const fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3, w_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    pub fn add_mat4(&self, rhs: &Mat4) -> Mat4 {
        Mat4::from_cols(
            self.x_axis + rhs.x_axis,
            self.y_axis + rhs.y_axis,
            self.z_axis + rhs.z_axis,
            self.w_axis + rhs.w_axis,
        )
    }

    pub fn mul_mat4(&self, rhs: &Mat4) -> Mat4 {
        Self::from_cols(
            self.mul(rhs.x_axis),
            self.mul(rhs.y_axis),
            self.mul(rhs.z_axis),
            self.mul(rhs.w_axis),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat3_mul() {
        //
        // | 3  3  4 |
        // | 1  5  2 |
        // | 4  8  6 |
        //
        let mat1 = Mat3::from_cols(
            Vec3::new(3.0, 1.0, 4.0),
            Vec3::new(3.0, 5.0, 8.0),
            Vec3::new(4.0, 2.0, 6.0),
        );

        //
        // | 2  3  9 |
        // | 3  5  6 |
        // | 4  6  1 |
        //
        let mat2 = Mat3::from_cols(
            Vec3::new(2.0, 3.0, 4.0),
            Vec3::new(3.0, 5.0, 6.0),
            Vec3::new(9.0, 6.0, 1.0),
        );

        assert_eq!(
            mat1.mul_mat3(&mat2),
            Mat3::from_cols(
                Vec3::new(
                    3.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0,
                    1.0 * 2.0 + 5.0 * 3.0 + 2.0 * 4.0,
                    4.0 * 2.0 + 8.0 * 3.0 + 6.0 * 4.0
                ),
                Vec3::new(
                    3.0 * 3.0 + 3.0 * 5.0 + 4.0 * 6.0,
                    1.0 * 3.0 + 5.0 * 5.0 + 2.0 * 6.0,
                    4.0 * 3.0 + 8.0 * 5.0 + 6.0 * 6.0
                ),
                Vec3::new(
                    3.0 * 9.0 + 3.0 * 6.0 + 4.0 * 1.0,
                    1.0 * 9.0 + 5.0 * 6.0 + 2.0 * 1.0,
                    4.0 * 9.0 + 8.0 * 6.0 + 6.0 * 1.0
                )
            )
        );
    }
}
