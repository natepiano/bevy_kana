use std::fmt;

use bevy::math::Vec3;
use bevy::reflect::Reflect;

/// Tolerance for checking if a vector is approximately unit length.
const UNIT_LENGTH_EPSILON: f32 = 1e-4;

/// Error returned when attempting to create a `Normal` from a zero-length vector.
#[derive(Debug, Clone, Copy)]
pub struct ZeroLengthError;

impl fmt::Display for ZeroLengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot create a `Normal` from a zero-length vector")
    }
}

impl std::error::Error for ZeroLengthError {}

/// A unit-length surface normal in 3D space.
///
/// `Normal` enforces the invariant that the wrapped `Vec3` always has
/// length 1.0. Arithmetic operations that would break this invariant
/// return a raw `Vec3` instead (type demotion), while operations that
/// preserve unit length (like negation) return `Normal`.
///
/// # Construction
///
/// Use [`Normal::new`] for validated construction (normalizes the input)
/// or [`Normal::new_unchecked`] when you know the input is already normalized.
///
/// ```
/// use bevy::math::Vec3;
/// use bevy_kana::Normal;
///
/// let normal = Normal::new(Vec3::new(1.0, 1.0, 0.0)).unwrap();
/// assert!((normal.length() - 1.0).abs() < 1e-6);
/// ```
///
/// # Arithmetic demotion
///
/// Adding or scaling normals may produce non-unit vectors, so these
/// operations return `Vec3`:
///
/// ```
/// use bevy::math::Vec3;
/// use bevy_kana::Normal;
///
/// let a = Normal::new(Vec3::X).unwrap();
/// let b = Normal::new(Vec3::Y).unwrap();
/// let sum: Vec3 = a + b; // not a `Normal` — sum isn't unit length
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[repr(transparent)]
pub struct Normal(Vec3);

impl Normal {
    /// Creates a `Normal` by normalizing the input vector.
    ///
    /// # Errors
    ///
    /// Returns [`ZeroLengthError`] if `value` has zero length.
    pub fn new(value: Vec3) -> Result<Self, ZeroLengthError> {
        let length = value.length();
        if length == 0.0 {
            return Err(ZeroLengthError);
        }
        Ok(Self(value / length))
    }

    /// Creates a `Normal` without verifying that the vector is unit length.
    ///
    /// In debug builds, this asserts that the vector is approximately unit
    /// length (within [`UNIT_LENGTH_EPSILON`]). In release builds, no check
    /// is performed.
    ///
    /// Use this on hot paths where you know the vector is already normalized
    /// (e.g., it just came out of [`Vec3::normalize`]).
    #[must_use]
    pub fn new_unchecked(value: Vec3) -> Self {
        debug_assert!(
            (value.length() - 1.0).abs() < UNIT_LENGTH_EPSILON,
            "Normal::new_unchecked called with non-unit vector (length: {})",
            value.length()
        );
        Self(value)
    }

    /// Consumes `self` and returns the inner `Vec3`.
    #[must_use]
    pub const fn into_inner(self) -> Vec3 { self.0 }
}

impl Default for Normal {
    /// Defaults to `Vec3::Y` (the conventional "up" normal).
    fn default() -> Self { Self(Vec3::Y) }
}

impl core::ops::Deref for Normal {
    type Target = Vec3;

    fn deref(&self) -> &Vec3 { &self.0 }
}

impl From<Normal> for Vec3 {
    fn from(normal: Normal) -> Self { normal.0 }
}

// --- Arithmetic: all return `Vec3` (type demotion) except `Neg` ---

impl core::ops::Add for Normal {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 { self.0 + rhs.0 }
}

impl core::ops::Sub for Normal {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 { self.0 - rhs.0 }
}

impl core::ops::Mul<f32> for Normal {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 { self.0 * rhs }
}

impl core::ops::Div<f32> for Normal {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 { self.0 / rhs }
}

/// Negation preserves unit length.
impl core::ops::Neg for Normal {
    type Output = Self;

    fn neg(self) -> Self { Self(-self.0) }
}

#[cfg(test)]
#[allow(clippy::float_cmp, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn new_normalizes_input() {
        let normal = Normal::new(Vec3::new(3.0, 0.0, 0.0)).unwrap();
        assert!((normal.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn new_rejects_zero_vector() {
        assert!(Normal::new(Vec3::ZERO).is_err());
    }

    #[test]
    fn add_demotes_to_vec3() {
        let a = Normal::new(Vec3::X).unwrap();
        let b = Normal::new(Vec3::Y).unwrap();
        let sum: Vec3 = a + b;
        assert_eq!(sum, Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn sub_demotes_to_vec3() {
        let a = Normal::new(Vec3::X).unwrap();
        let b = Normal::new(Vec3::Y).unwrap();
        let diff: Vec3 = a - b;
        assert_eq!(diff, Vec3::new(1.0, -1.0, 0.0));
    }

    #[test]
    fn mul_demotes_to_vec3() {
        let normal = Normal::new(Vec3::X).unwrap();
        let scaled: Vec3 = normal * 2.0;
        assert_eq!(scaled, Vec3::new(2.0, 0.0, 0.0));
    }

    #[test]
    fn neg_preserves_normal() {
        let normal = Normal::new(Vec3::X).unwrap();
        let negated: Normal = -normal;
        assert_eq!(negated.into_inner(), Vec3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn deref_provides_vec3_access() {
        let normal = Normal::new(Vec3::X).unwrap();
        assert_eq!(normal.x, 1.0);
        assert_eq!(normal.dot(Vec3::Y), 0.0);
    }

    #[test]
    fn default_is_up() {
        let normal = Normal::default();
        assert_eq!(normal.into_inner(), Vec3::Y);
    }
}
