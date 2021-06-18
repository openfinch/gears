// Quaternions
// Implements: https://docs.unity3d.com/ScriptReference/Quaternion.html

use super::vector::Vector3;
use std::f32::consts::PI;
use std::fmt;
use std::ops::Index;

/// Quaternions are used to represent rotations.
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// "Static Properties"
impl Quaternion {
    /// The identity rotation
    pub fn identity() -> Quaternion {
        Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

// "Properties"
impl Quaternion {
    /// Returns the euler angle representation of the rotation
    pub fn euler_angles(&self) -> Vector3 {
        let (x, y, z) = Quaternion::quaternion_to_euler(self.w, self.x, self.y, self.z);
        Vector3 { x, y, z }
    }

    /// Returns this quaternion with a magnitude of 1
    pub fn normalized(&self) -> Quaternion {
        let magnitude =
            (self.w.powf(2.0) + self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        let w = self.w / magnitude;
        let x = self.x / magnitude;
        let y = self.y / magnitude;
        let z = self.z / magnitude;
        Quaternion { w, x, y, z }
    }
}

// Constructors
impl Quaternion {
    /// Constructs new Quaternion with given x,y,z,w components
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Quaternion {
        Quaternion { w, x, y, z }
    }
    /// Constructs new Quaternion with given Vector3
    pub fn new_from_euler(vector: Vector3) -> Quaternion {
        // REFACTOR: This should be an unpack/destructure.
        let (x, y, z, w) = Quaternion::euler_to_quaternion(vector.x, vector.y, vector.z);
        Quaternion { w, x, y, z }
    }
}

// Public Methods
impl Quaternion {
    /// Set x, y, z and w components of an existing Quaternion
    pub fn set(&mut self, w: f32, x: f32, y: f32, z: f32) {
        self.w = w;
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Sets the euler angle representation of the rotation.
    pub fn set_from_euler(&mut self, vector: Vector3) {
        let c = Quaternion::euler_to_quaternion(vector.x, vector.y, vector.z);
        self.x = c.0;
        self.y = c.1;
        self.z = c.2;
        self.w = c.3;
    }

    /// Creates a rotation which rotates from 'from' to 'to'
    pub fn set_from_to_rotation(from: Vector3, to: Vector3) {
        // TODO: Work out how to do this...
    }

    /// Creates a rotation with the specified forward directions using default "up"
    pub fn set_look_rotation(view: Vector3) {
        // TODO: Work out how to do this...
        let up = Vector3::up();
    }

    /// Creates a rotation with the specified forward and upwards directions
    pub fn set_look_rotation_with_up(view: Vector3, up: Vector3) {
        // TODO: Work out how to do this...
    }

    /// Converts a rotation to angle-axis representation (angles in degrees)
    pub fn to_angle_axis(&self, angle: &f32, axis: &Vector3) {}
}

// Static Methods
impl Quaternion {
    /// Returns the angle in degrees between two rotations a and b
    pub fn angle(a: Quaternion, b: Quaternion) -> f32 {
        let f = Quaternion::dot(a, b);
        (((f).abs().min(1.0)) * 2.0 * 57.29578).acos()
    }

    /// Returns a rotation which rotates `angle` degrees around `axis`.
    pub fn angle_axis(angle: f32, axis: Vector3) -> Quaternion {
        Quaternion {
            w: angle.cos(),
            x: axis.x * angle,
            y: axis.y * angle,
            z: axis.z * angle,
        }
    }

    /// Returns the dot product between two rotations
    pub fn dot(a: Quaternion, b: Quaternion) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}

// Std Trait Implementations
impl Index<i8> for Quaternion {
    type Output = f32;

    /// Access the x, y, z, w components using [0], [1], [2], [3] respectively
    fn index(&self, i: i8) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of range, got {}.", i),
        }
    }
}

impl fmt::Display for Quaternion {
    /// Returns a formatted string of the Quaternion
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{{{}, {}, {}, {}}}", self.x, self.y, self.z, self.w)
        )
    }
}

// Private Methods
impl Quaternion {
    fn euler_to_quaternion(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32) {
        // Abbreviations for the various angular functions
        let cy = (z * 0.5).cos();
        let sy = (z * 0.5).sin();
        let cp = (y * 0.5).cos();
        let sp = (y * 0.5).sin();
        let cr = (x * 0.5).cos();
        let sr = (x * 0.5).sin();

        let _w = cr * cp * cy + sr * sp * sy;
        let _x = sr * cp * cy - cr * sp * sy;
        let _y = cr * sp * cy + sr * cp * sy;
        let _z = cr * cp * sy - sr * sp * cy;

        (_x, _y, _z, _w)
    }

    fn quaternion_to_euler(w: f32, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        // roll (x-axis rotation)
        let sinr_cosp = 2.0 * (w * x + y * z);
        let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
        let ex = sinr_cosp.atan2(cosr_cosp);

        // pitch (y-axis rotation)
        let ey: f32;
        let sinp = 2.0 * (w * y - z * x);
        if sinp.abs() >= 1.0 {
            ey = (PI / 2.0).copysign(sinp); // use 90 degrees if out of range
        } else {
            ey = sinp.asin();
        }

        // yaw (z-axis rotation)
        let siny_cosp = 2.0 * (w * z + x * y);
        let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
        let ez = siny_cosp.atan2(cosy_cosp);

        (ex, ey, ez)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn quaternion_identity_is_zeroed_by_default() {
        let q = Quaternion::identity();

        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
        assert_eq!(q.w, 1.0);
    }

    #[test]
    fn quaternion_can_derive_euler_angles() {
        let q = Quaternion {
            x: 90.0,
            y: 90.0,
            z: 90.0,
            w: 1.0,
        };
        let eq = q.euler_angles();

        assert_eq!(eq.x, 2.6734982);
        assert_eq!(eq.y, -FRAC_PI_2);
        assert_eq!(eq.z, 2.6734982);
    }

    #[test]
    fn quaternion_can_be_normalised() {
        let q = Quaternion {
            x: 90.0,
            y: 90.0,
            z: 90.0,
            w: 1.0,
        };
        let eq = q.normalized();

        assert_eq!(eq.x, 0.57733834);
        assert_eq!(eq.y, 0.57733834);
        assert_eq!(eq.z, 0.57733834);
        assert_eq!(eq.w, 0.0064148707);
    }

    #[test]
    fn quaternion_can_be_constructed_with_new() {
        let q = Quaternion::new(1.0, 90.0, 90.0, 90.0);

        assert_eq!(q.x, 90.0);
        assert_eq!(q.y, 90.0);
        assert_eq!(q.z, 90.0);
        assert_eq!(q.w, 1.0);
    }

    #[test]
    #[ignore]
    fn quaternion_can_be_constructed_from_euler_vector3() {
        // TODO: This is broken..
        let v = Vector3 {
            x: 2.6734982,
            y: -FRAC_PI_2,
            z: 2.6734982,
        };
        let q = Quaternion::new_from_euler(v);

        assert_eq!(q.x, 90.0);
        assert_eq!(q.y, 90.0);
        assert_eq!(q.z, 90.0);
        assert_eq!(q.w, 1.0);
    }

    #[test]
    fn quaternion_can_be_set_to_a_new_value() {
        let mut q = Quaternion::new(1.0, 90.0, 90.0, 90.0);
        q.set(2.0, 180.0, 35.666666, 0.0);

        assert_eq!(q.x, 180.0);
        assert_eq!(q.y, 35.666666);
        assert_eq!(q.z, 0.0);
        assert_eq!(q.w, 2.0);
    }

    #[test]
    fn quaternion_can_be_set_to_a_new_value_from_euler() {
        let mut q = Quaternion::new(1.0, 90.0, 90.0, 90.0);
        let v = Vector3 {
            x: 2.6734982,
            y: -FRAC_PI_2,
            z: 2.6734982,
        };
        q.set_from_euler(v);

        assert_eq!(q.x, 0.3190371);
        assert_eq!(q.y, 0.6310431);
        assert_eq!(q.z, 0.3190371);
        assert_eq!(q.w, -0.6310431);
    }

    #[test]
    #[ignore]
    fn can_calculate_the_angle_between_two_quaternions() {
        // TODO: This doesn't work, qv being returned as NaN
        let q0 = Quaternion::new(1.0, 180.0, 180.0, 180.0);
        let q1 = Quaternion::new(0.5, 90.0, 90.0, 90.0);

        let qv = Quaternion::angle(q0, q1);

        assert_eq!(qv, 90.0);
    }

    #[test]
    fn can_calculate_the_rotation_of_a_angle_about_an_axis() {
        // TODO: This doesn't work, qv being returned as NaN
        let v = Vector3 {
            x: 2.6734982,
            y: -FRAC_PI_2,
            z: 2.6734982,
        };

        let q = Quaternion::angle_axis(180.0, v);

        assert_eq!(q.x, 481.22968);
        assert_eq!(q.y, -282.74335);
        assert_eq!(q.z, 481.22968);
        assert_eq!(q.w, -0.5984601);
    }

    #[test]
    fn can_get_quaternion_attributes_by_index() {
        // TODO: This doesn't work, qv being returned as NaN
        let q = Quaternion::new(0.5, 90.0, 90.0, 90.0);

        assert_eq!(q[0], 90.0);
        assert_eq!(q[1], 90.0);
        assert_eq!(q[2], 90.0);
        assert_eq!(q[3], 0.5);
    }

    #[test]
    #[should_panic(expected = "Index out of range, got 4.")]
    fn getting_quaternion_attributes_by_index_panics_if_out_of_range() {
        // TODO: This doesn't work, qv being returned as NaN
        let q = Quaternion::new(0.5, 90.0, 90.0, 90.0)[4];
    }

    #[test]
    fn can_get_quaternion_attributes_as_str() {
        // TODO: This doesn't work, qv being returned as NaN
        let q = Quaternion::new(0.5, 90.0, 90.0, 90.0);
        let qs = format!("{}", q);

        assert_eq!(qs, "{90, 90, 90, 0.5}");
    }
}
