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
    /// A delta or offset in 3D space.
    ///
    /// Wraps `Vec3` to distinguish spatial offsets from other vector
    /// quantities like position or velocity. Use `Displacement` to
    /// represent the difference between two points, a movement delta,
    /// or any directional distance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::math::Vec3;
    /// use bevy_kana::Displacement;
    ///
    /// let step = Displacement(Vec3::new(0.0, 0.0, -1.0));
    /// let double_step = step + step;
    /// assert_eq!(double_step.into_inner(), Vec3::new(0.0, 0.0, -2.0));
    /// ```
    Displacement, Vec3
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_self() {
        let left_displacement = Displacement(Vec3::new(1.0, 0.0, 0.0));
        let right_displacement = Displacement(Vec3::new(0.0, 1.0, 0.0));
        let result = left_displacement + right_displacement;
        assert_eq!(result.into_inner(), Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn from_into_roundtrip() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let displacement = Displacement::from(vec3);
        let back: Vec3 = displacement.into();
        assert_eq!(vec3, back);
    }
}
