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
macro_rules! semantic_newtype {
    (
        $(#[$meta:meta])*
        $name:ident, $inner:ty
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
        #[repr(transparent)]
        pub struct $name(pub $inner);

        impl $name {
            /// Consumes `self` and returns the inner value.
            ///
            /// Use this when you need to pass the raw type to a Bevy API.
            pub const fn into_inner(self) -> $inner {
                self.0
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
    };
}
