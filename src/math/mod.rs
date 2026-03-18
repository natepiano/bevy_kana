//! Zero-cost newtype wrappers around Bevy math primitives.
//!
//! All types are `#[repr(transparent)]` and `Deref` to their inner type
//! for ergonomic field and method access.

#[macro_use]
mod macros;
mod displacement;
mod normal;
mod orientation;
mod position;
mod screen_position;
mod velocity;

pub use displacement::Displacement;
pub use normal::Normal;
pub use normal::ZeroLengthError;
pub use orientation::Orientation;
pub use position::Position;
pub use screen_position::ScreenPosition;
pub use velocity::Velocity;
