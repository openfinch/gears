use std::ops::Add;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn up() -> f32 {
        0.0
    }

    /// Return magnitude of vector.
    fn magnitude(&self) -> f32 {
        let mag = self.x * self.x + self.y * self.y + self.z * self.z;
        mag.sqrt()
    }

    /// Normalize vector.
    fn normalize(&mut self) {
        let length = self.magnitude();
        if length > 0.0 {
            self.x /= length;
            self.y /= length;
            self.z /= length;
        }
    }

    /// Scale vector.
    fn scale(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl Add for Vector3 {
    type Output = Self;

    /// Add two Vectors, return new Vector.
    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_vector3() {
        let v = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn vector3_as_default_up() {
        assert_eq!(Vector3::up(), 0.0);
    }

    #[test]
    fn can_calculate_magnitude_of_vector3() {
        let v = Vector3 {
            x: 1.45698,
            y: 42.6554,
            z: -4.15254654,
        };
        let vn = v.magnitude();

        assert_eq!(vn, 42.88181);
    }

    #[test]
    fn can_normalise_vector3() {
        let mut v = Vector3 {
            x: 1.45698,
            y: 42.6554,
            z: -4.15254654,
        };
        v.normalize();

        assert_eq!(v.x, 0.033976644);
        assert_eq!(v.y, 0.99472016);
        assert_eq!(v.z, -0.096837014);
    }

    #[test]
    fn can_scale_vector3() {
        let mut v = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        v.scale(2.0);

        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 2.0);
    }

    #[test]
    fn can_add_two_vector3() {
        let v0 = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let v1 = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let v2 = v0 + v1;

        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, 2.0);
    }
}
