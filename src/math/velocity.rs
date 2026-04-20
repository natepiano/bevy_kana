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
    /// Rate of position change in 3D space.
    ///
    /// Wraps `Vec3` to distinguish velocity from other vector quantities.
    /// Scaling a `Velocity` by a time delta (e.g., `vel * dt`) gives a
    /// per-frame displacement while preserving the `Velocity` type for
    /// the scaled result.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::math::Vec3;
    /// use bevy_kana::Velocity;
    ///
    /// let velocity = Velocity(Vec3::new(10.0, 0.0, 0.0));
    /// let dt = 0.016;
    /// let frame_velocity = velocity * dt;
    /// assert!((frame_velocity.x - 0.16).abs() < 1e-6);
    /// ```
    Velocity, Vec3
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_mul_for_dt() {
        let velocity = Velocity(Vec3::new(10.0, 0.0, 0.0));
        let frame_velocity = velocity * 0.016;
        assert!((frame_velocity.x - 0.16).abs() < 1e-6);
    }

    #[test]
    fn add_combines_velocities() {
        let left_velocity = Velocity(Vec3::new(1.0, 0.0, 0.0));
        let right_velocity = Velocity(Vec3::new(0.0, 1.0, 0.0));
        let combined = left_velocity + right_velocity;
        assert_eq!(combined.into_inner(), Vec3::new(1.0, 1.0, 0.0));
    }
}
