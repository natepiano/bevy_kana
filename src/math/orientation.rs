use std::ops::Deref;
use std::ops::Mul;
use std::ops::MulAssign;

use bevy::math::Quat;
use bevy::math::Vec3;
use bevy::reflect::Reflect;

/// A rotation in 3D space.
///
/// Wraps `Quat` with semantic meaning. Unlike the semantic `Vec3` types,
/// `Orientation` has custom arithmetic:
///
/// - `Orientation * Orientation → Orientation` (rotation composition)
/// - `Orientation * Vec3 → Vec3` (rotate a vector)
///
/// Other `Quat` methods are available through `Deref`.
///
/// # Examples
///
/// ```
/// use bevy::math::Quat;
/// use bevy::math::Vec3;
/// use bevy_kana::Orientation;
///
/// let orientation = Orientation::from(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2));
/// let rotated = orientation * Vec3::X;
/// assert!((rotated - Vec3::NEG_Z).length() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Orientation(pub Quat);

impl Orientation {
    /// Consumes `self` and returns the inner `Quat`.
    #[must_use]
    pub const fn into_inner(self) -> Quat { self.0 }

    /// Returns the inverse rotation.
    #[must_use]
    pub fn inverse(self) -> Self { Self(self.0.inverse()) }

    /// Spherical linear interpolation between `self` and `other`.
    #[must_use]
    pub fn slerp(self, other: Self, t: f32) -> Self { Self(self.0.slerp(other.0, t)) }

    /// Linear interpolation between `self` and `other`.
    ///
    /// Faster than [`Orientation::slerp`] but less accurate for large
    /// angular differences.
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self { Self(self.0.lerp(other.0, t)) }
}

impl Deref for Orientation {
    type Target = Quat;

    fn deref(&self) -> &Quat { &self.0 }
}

impl From<Quat> for Orientation {
    fn from(value: Quat) -> Self { Self(value) }
}

impl From<Orientation> for Quat {
    fn from(value: Orientation) -> Self { value.0 }
}

/// Rotation composition: applying `rhs` then `self`.
impl Mul for Orientation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0) }
}

impl MulAssign for Orientation {
    fn mul_assign(&mut self, rhs: Self) { self.0 = self.0 * rhs.0; }
}

/// Rotates a vector by this orientation.
impl Mul<Vec3> for Orientation {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 { self.0 * rhs }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::*;

    #[test]
    fn rotation_composition() {
        let first_orientation = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let second_orientation = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let composed = first_orientation * second_orientation;
        let result = composed * Vec3::X;
        assert!((result - Vec3::NEG_X).length() < 1e-5);
    }

    #[test]
    fn rotate_vector() {
        let orientation = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let result = orientation * Vec3::X;
        assert!((result - Vec3::NEG_Z).length() < 1e-6);
    }

    #[test]
    fn inverse_undoes_rotation() {
        let orientation = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let inverse_orientation = orientation.inverse();
        let composed = orientation * inverse_orientation;
        let result = composed * Vec3::X;
        assert!((result - Vec3::X).length() < 1e-6);
    }

    #[test]
    fn slerp_halfway() {
        let start_orientation = Orientation::from(Quat::IDENTITY);
        let end_orientation = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let midpoint_orientation = start_orientation.slerp(end_orientation, 0.5);
        let result = midpoint_orientation * Vec3::X;
        let angle = result.angle_between(Vec3::X);
        assert!((angle - std::f32::consts::FRAC_PI_4).abs() < 1e-5);
    }

    #[test]
    fn deref_provides_quat_access() {
        let orientation = Orientation::from(Quat::IDENTITY);
        assert!((orientation.w - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn from_into_roundtrip() {
        let quat = Quat::from_rotation_y(FRAC_PI_2);
        let orientation = Orientation::from(quat);
        let back: Quat = orientation.into();
        assert_eq!(quat, back);
    }
}
