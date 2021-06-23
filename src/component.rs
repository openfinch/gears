use crate::quaternion::Quaternion;
use crate::vector::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub(crate) dx: f32,
    pub(crate) dy: f32,
    pub(crate) dz: f32,
}
