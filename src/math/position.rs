use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use bevy::math::Vec3;
use bevy::reflect::Reflect;

semantic_newtype!(
    /// A point in 3D space.
    ///
    /// Wraps `Vec3` to distinguish spatial positions from other vector
    /// quantities like velocity or displacement. All arithmetic operations
    /// return `Position`, and `Deref` provides transparent access to
    /// `Vec3` fields and methods.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::math::Vec3;
    /// use bevy_kana::Position;
    ///
    /// let start_position = Position(Vec3::new(1.0, 0.0, 0.0));
    /// let end_position = Position(Vec3::new(3.0, 0.0, 0.0));
    ///
    /// // Centroid of two points
    /// let midpoint = (start_position + end_position) / 2.0;
    /// assert_eq!(midpoint.into_inner(), Vec3::new(2.0, 0.0, 0.0));
    /// ```
    Position, Vec3
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_self() {
        let left_position = Position(Vec3::new(1.0, 0.0, 0.0));
        let right_position = Position(Vec3::new(0.0, 1.0, 0.0));
        let result = left_position + right_position;
        assert_eq!(result.into_inner(), Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn scalar_mul() {
        let position = Position(Vec3::new(1.0, 2.0, 3.0));
        let result = position * 2.0;
        assert_eq!(result.into_inner(), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn scalar_div() {
        let position = Position(Vec3::new(2.0, 4.0, 6.0));
        let result = position / 2.0;
        assert_eq!(result.into_inner(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn neg() {
        let position = Position(Vec3::new(1.0, -2.0, 3.0));
        let result = -position;
        assert_eq!(result.into_inner(), Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn deref_provides_vec3_access() {
        let position = Position(Vec3::new(1.0, 2.0, 3.0));
        assert!((position.x - 1.0).abs() < f32::EPSILON);
        assert!((position.length() - Vec3::new(1.0, 2.0, 3.0).length()).abs() < f32::EPSILON);
    }

    #[test]
    fn from_into_roundtrip() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let position = Position::from(vec3);
        let back: Vec3 = position.into();
        assert_eq!(vec3, back);
    }

    #[test]
    fn add_assign() {
        let mut position = Position(Vec3::new(1.0, 0.0, 0.0));
        position += Position(Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(position.into_inner(), Vec3::new(1.0, 1.0, 0.0));
    }
}
