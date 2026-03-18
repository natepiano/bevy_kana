use bevy::math::Vec2;
use bevy::reflect::Reflect;
semantic_newtype!(
    /// Pixel-space coordinates on screen.
    ///
    /// Wraps `Vec2` to distinguish screen coordinates from other 2D
    /// quantities. Useful for UI layout, cursor tracking, and any
    /// computation in pixel space.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::math::Vec2;
    /// use bevy_kana::ScreenPosition;
    ///
    /// let cursor = ScreenPosition(Vec2::new(640.0, 480.0));
    /// let offset = ScreenPosition(Vec2::new(10.0, -5.0));
    /// let moved = cursor + offset;
    /// assert_eq!(moved.into_inner(), Vec2::new(650.0, 475.0));
    /// ```
    ScreenPosition, Vec2
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_self() {
        let a = ScreenPosition(Vec2::new(100.0, 200.0));
        let b = ScreenPosition(Vec2::new(10.0, 20.0));
        let result = a + b;
        assert_eq!(result.into_inner(), Vec2::new(110.0, 220.0));
    }

    #[test]
    fn deref_provides_vec2_access() {
        let sp = ScreenPosition(Vec2::new(100.0, 200.0));
        assert_eq!(sp.x, 100.0);
        assert_eq!(sp.y, 200.0);
    }

    #[test]
    fn from_into_roundtrip() {
        let v = Vec2::new(100.0, 200.0);
        let sp = ScreenPosition::from(v);
        let back: Vec2 = sp.into();
        assert_eq!(v, back);
    }
}
