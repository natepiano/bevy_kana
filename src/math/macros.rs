/// Generates a semantic newtype wrapper around a math primitive.
///
/// Semantic types wrap an inner type with no invariant — their purpose is
/// to prevent accidental mixing of values that share the same underlying
/// type but carry different meaning (e.g., `Position` vs `Velocity`).
///
/// All arithmetic operations return `Self`, and `Deref` provides transparent
/// access to the inner type's methods and fields.
///
/// # Generated API
///
/// - `Deref<Target = InnerType>` for transparent field and method access
/// - `From<InnerType>` and `Into<InnerType>` conversions
/// - `into_inner(self) -> InnerType`
/// - `Add`, `Sub`, `Mul<f32>`, `Div<f32>`, `Neg` (all return `Self`)
/// - `AddAssign`, `SubAssign`, `MulAssign<f32>`, `DivAssign<f32>`
/// - `Add<InnerType>`, `Sub<InnerType>` for mixing with raw Bevy values
/// - `distance`, `distance_squared`, `lerp` accepting `impl Into<Self>`
macro_rules! semantic_newtype {
    (
        $(#[$meta:meta])*
        $name:ident, $inner:ty
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
        pub struct $name(pub $inner);

        impl $name {
            /// Creates a new value from components.
            pub const fn new(x: f32, y: f32, z: f32) -> Self {
                Self(<$inner>::new(x, y, z))
            }

            /// Consumes `self` and returns the inner value.
            ///
            /// Use this when you need to pass the raw type to a Bevy API.
            pub const fn into_inner(self) -> $inner {
                self.0
            }

            /// Euclidean distance between two values.
            pub fn distance(self, other: impl Into<Self>) -> f32 {
                self.0.distance(other.into().0)
            }

            /// Squared euclidean distance (avoids a square root).
            pub fn distance_squared(self, other: impl Into<Self>) -> f32 {
                self.0.distance_squared(other.into().0)
            }

            /// Linear interpolation between two values.
            #[must_use]
            pub fn lerp(self, other: impl Into<Self>, t: f32) -> Self {
                Self(self.0.lerp(other.into().0, t))
            }
        }

        impl core::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &$inner {
                &self.0
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl core::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl core::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl core::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl core::ops::SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl core::ops::Mul<f32> for $name {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self {
                Self(self.0 * rhs)
            }
        }

        impl core::ops::MulAssign<f32> for $name {
            fn mul_assign(&mut self, rhs: f32) {
                self.0 *= rhs;
            }
        }

        impl core::ops::Div<f32> for $name {
            type Output = Self;

            fn div(self, rhs: f32) -> Self {
                Self(self.0 / rhs)
            }
        }

        impl core::ops::DivAssign<f32> for $name {
            fn div_assign(&mut self, rhs: f32) {
                self.0 /= rhs;
            }
        }

        impl core::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        // Cross-type arithmetic with raw inner type.
        // Allows natural mixing with raw `Vec3` values from Bevy APIs.

        impl core::ops::Add<$inner> for $name {
            type Output = Self;

            fn add(self, rhs: $inner) -> Self {
                Self(self.0 + rhs)
            }
        }

        impl core::ops::AddAssign<$inner> for $name {
            fn add_assign(&mut self, rhs: $inner) {
                self.0 += rhs;
            }
        }

        impl core::ops::Sub<$inner> for $name {
            type Output = Self;

            fn sub(self, rhs: $inner) -> Self {
                Self(self.0 - rhs)
            }
        }

        impl core::ops::SubAssign<$inner> for $name {
            fn sub_assign(&mut self, rhs: $inner) {
                self.0 -= rhs;
            }
        }
    };
}
