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
        let a = Displacement(Vec3::new(1.0, 0.0, 0.0));
        let b = Displacement(Vec3::new(0.0, 1.0, 0.0));
        let result = a + b;
        assert_eq!(result.into_inner(), Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn from_into_roundtrip() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let d = Displacement::from(v);
        let back: Vec3 = d.into();
        assert_eq!(v, back);
    }
}
