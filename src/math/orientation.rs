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
/// let rot = Orientation::from(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2));
/// let rotated = rot * Vec3::X;
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

impl core::ops::Deref for Orientation {
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
impl core::ops::Mul for Orientation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0) }
}

impl core::ops::MulAssign for Orientation {
    fn mul_assign(&mut self, rhs: Self) { self.0 = self.0 * rhs.0; }
}

/// Rotates a vector by this orientation.
impl core::ops::Mul<Vec3> for Orientation {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 { self.0 * rhs }
}

#[cfg(test)]
#[allow(
    clippy::float_cmp,
    reason = "exact equality is appropriate for deterministic math tests"
)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::*;

    #[test]
    fn rotation_composition() {
        let a = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let b = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let composed = a * b;
        let result = composed * Vec3::X;
        assert!((result - Vec3::NEG_X).length() < 1e-5);
    }

    #[test]
    fn rotate_vector() {
        let rot = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let result = rot * Vec3::X;
        assert!((result - Vec3::NEG_Z).length() < 1e-6);
    }

    #[test]
    fn inverse_undoes_rotation() {
        let rot = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let inv = rot.inverse();
        let composed = rot * inv;
        let result = composed * Vec3::X;
        assert!((result - Vec3::X).length() < 1e-6);
    }

    #[test]
    fn slerp_halfway() {
        let a = Orientation::from(Quat::IDENTITY);
        let b = Orientation::from(Quat::from_rotation_y(FRAC_PI_2));
        let mid = a.slerp(b, 0.5);
        let result = mid * Vec3::X;
        let angle = result.angle_between(Vec3::X);
        assert!((angle - std::f32::consts::FRAC_PI_4).abs() < 1e-5);
    }

    #[test]
    fn deref_provides_quat_access() {
        let rot = Orientation::from(Quat::IDENTITY);
        assert_eq!(rot.w, 1.0);
    }

    #[test]
    fn from_into_roundtrip() {
        let q = Quat::from_rotation_y(FRAC_PI_2);
        let o = Orientation::from(q);
        let back: Quat = o.into();
        assert_eq!(q, back);
    }
}
