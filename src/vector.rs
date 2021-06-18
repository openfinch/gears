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
        self.y *= s;
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
