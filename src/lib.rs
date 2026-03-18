//! # bevy_kana
//!
//! Simplified, named wrappers for Bevy.
//!
//! `bevy_kana` provides zero-cost newtype wrappers around Bevy math primitives
//! that prevent accidental mixing at compile time. All types are
//! `#[repr(transparent)]` and `Deref` to their inner type for ergonomic field
//! and method access.
//!
//! ## Type categories
//!
//! **Semantic types** wrap an inner type with no invariant — they exist to
//! prevent mixing values that share the same underlying type but carry
//! different meaning:
//!
//! - [`Position`] — a point in 3D space (`Vec3`)
//! - [`Displacement`] — a delta or offset (`Vec3`)
//! - [`Velocity`] — rate of position change (`Vec3`)
//! - [`ScreenPosition`] — pixel-space coordinates (`Vec2`)
//! - [`Orientation`] — a rotation (`Quat`)
//!
//! **Invariant types** enforce a property at construction. Arithmetic that
//! would break the invariant returns the underlying type (type demotion):
//!
//! - [`Normal`] — a unit-length surface normal (`Vec3`)

#[macro_use]
mod macros;
mod displacement;
mod normal;
mod orientation;
mod position;
pub mod prelude;
mod screen_position;
mod velocity;

pub use displacement::Displacement;
pub use normal::Normal;
pub use normal::ZeroLengthError;
pub use orientation::Orientation;
pub use position::Position;
pub use screen_position::ScreenPosition;
pub use velocity::Velocity;
