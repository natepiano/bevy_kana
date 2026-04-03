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
    /// let a = Position(Vec3::new(1.0, 0.0, 0.0));
    /// let b = Position(Vec3::new(3.0, 0.0, 0.0));
    ///
    /// // Centroid of two points
    /// let midpoint = (a + b) / 2.0;
    /// assert_eq!(midpoint.into_inner(), Vec3::new(2.0, 0.0, 0.0));
    /// ```
    Position, Vec3
);

#[cfg(test)]
#[allow(
    clippy::float_cmp,
    reason = "exact equality is appropriate for deterministic math tests"
)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_self() {
        let a = Position(Vec3::new(1.0, 0.0, 0.0));
        let b = Position(Vec3::new(0.0, 1.0, 0.0));
        let result = a + b;
        assert_eq!(result.into_inner(), Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn scalar_mul() {
        let p = Position(Vec3::new(1.0, 2.0, 3.0));
        let result = p * 2.0;
        assert_eq!(result.into_inner(), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn scalar_div() {
        let p = Position(Vec3::new(2.0, 4.0, 6.0));
        let result = p / 2.0;
        assert_eq!(result.into_inner(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn neg() {
        let p = Position(Vec3::new(1.0, -2.0, 3.0));
        let result = -p;
        assert_eq!(result.into_inner(), Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn deref_provides_vec3_access() {
        let p = Position(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(p.x, 1.0);
        assert_eq!(p.length(), Vec3::new(1.0, 2.0, 3.0).length());
    }

    #[test]
    fn from_into_roundtrip() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let p = Position::from(v);
        let back: Vec3 = p.into();
        assert_eq!(v, back);
    }

    #[test]
    fn add_assign() {
        let mut p = Position(Vec3::new(1.0, 0.0, 0.0));
        p += Position(Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(p.into_inner(), Vec3::new(1.0, 1.0, 0.0));
    }
}
