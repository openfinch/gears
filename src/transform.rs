use super::quaternion::Quaternion;
use super::vector::Vector3;

pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
