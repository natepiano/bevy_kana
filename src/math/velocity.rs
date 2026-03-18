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
    /// let vel = Velocity(Vec3::new(10.0, 0.0, 0.0));
    /// let dt = 0.016;
    /// let frame_vel = vel * dt;
    /// assert!((frame_vel.x - 0.16).abs() < 1e-6);
    /// ```
    Velocity, Vec3
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_mul_for_dt() {
        let vel = Velocity(Vec3::new(10.0, 0.0, 0.0));
        let frame = vel * 0.016;
        assert!((frame.x - 0.16).abs() < 1e-6);
    }

    #[test]
    fn add_combines_velocities() {
        let a = Velocity(Vec3::new(1.0, 0.0, 0.0));
        let b = Velocity(Vec3::new(0.0, 1.0, 0.0));
        let combined = a + b;
        assert_eq!(combined.into_inner(), Vec3::new(1.0, 1.0, 0.0));
    }
}
