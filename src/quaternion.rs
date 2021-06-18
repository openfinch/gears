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
            w: 0.0,
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }
}

// "Properties"
impl Quaternion {
    /// Returns the euler angle representation of the rotation
    pub fn euler_angles(&self) -> Vector3 {
        let c = Quaternion::quaternion_to_euler(self.w, self.x, self.y, self.z);
        Vector3 {
            x: c.0,
            y: c.1,
            z: c.2,
        }
    }

    /// Returns this quaternion with a magnitude of 1
    pub fn normalized(&self) -> Quaternion {
        // TODO: Work out how to do this...
        Quaternion::identity()
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
        let c = Quaternion::euler_to_quaternion(vector.x, vector.y, vector.z);
        Quaternion {
            x: c.0,
            y: c.1,
            z: c.2,
            w: c.3,
        }
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

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = cr * sp * cy + sr * cp * sy;
        let z = cr * cp * sy - sr * sp * cy;

        (x, y, z, w)
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
