//! Input action macros and keybinding utilities for `bevy_enhanced_input`.
//!
//! Provides macros to reduce boilerplate when wiring keyboard actions to
//! commands through intermediate events, and a [`Keybindings`] builder
//! that handles platform-specific modifier keys (Cmd vs Ctrl) and
//! automatic `BlockBy` application.

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

/// Generates a `bevy_enhanced_input` `InputAction` struct.
///
/// # Examples
///
/// ```ignore
/// use bevy_kana::action;
///
/// action!(CameraHome);
/// ```
///
/// Expands to:
///
/// ```ignore
/// #[derive(InputAction)]
/// #[action_output(bool)]
/// pub struct CameraHome;
/// ```
#[macro_export]
macro_rules! action {
    ($(#[$meta:meta])* $action:ident) => {
        $(#[$meta])*
        #[derive(InputAction)]
        #[action_output(bool)]
        pub struct $action;
    };
}

/// Generates a Bevy `Event` struct with `Reflect` support.
///
/// Supports both unit events and events with payload fields.
/// Events generated this way are compatible with the Bevy Remote Protocol's
/// `world.trigger_event`.
///
/// # Examples
///
/// Unit event:
///
/// ```ignore
/// use bevy_kana::event;
///
/// event!(PauseEvent);
/// ```
///
/// Payload event:
///
/// ```ignore
/// use bevy_kana::event;
///
/// event!(ZoomToTarget { entity: Entity });
/// ```
#[macro_export]
macro_rules! event {
    ($(#[$meta:meta])* $event:ident) => {
        $(#[$meta])*
        #[derive(Event, Reflect, Default)]
        #[reflect(Event)]
        pub struct $event;
    };
    ($(#[$meta:meta])* $event:ident { $($field:ident : $ty:ty),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Event, Reflect)]
        #[reflect(Event)]
        pub struct $event {
            $(pub $field: $ty,)+
        }
    };
}

/// Wires an input action to a command function through an intermediate event.
///
/// Registers two observers:
/// 1. `On<Start<Action>>` triggers the event
/// 2. `On<Event>` runs the command via `run_system_cached`
///
/// The intermediate event decouples keyboard input from command execution,
/// allowing the same command to be invoked by a keybinding, programmatically
/// via `commands.trigger(MyEvent)`, or through the Bevy Remote Protocol's
/// `world.trigger_event`.
///
/// Use with [`action!`] and [`event!`] to generate the action and event structs.
///
/// # Examples
///
/// ```ignore
/// use bevy_kana::action;
/// use bevy_kana::bind_action_system;
/// use bevy_kana::event;
///
/// action!(PauseToggle);
/// event!(PauseEvent);
///
/// fn setup(app: &mut App) {
///     bind_action_system!(app, PauseToggle, PauseEvent, pause_command);
/// }
/// ```
#[macro_export]
macro_rules! bind_action_system {
    ($app:expr, $action:ty, $event:ty, $command:path) => {
        $app.add_observer(
            |_: On<bevy_enhanced_input::action::events::Start<$action>>, mut commands: Commands| {
                commands.trigger(<$event>::default());
            },
        )
        .add_observer(|_: On<$event>, mut commands: Commands| {
            commands.run_system_cached($command);
        })
    };
}

/// Non-consuming modifier action for Cmd (macOS) / Ctrl (other platforms).
#[derive(InputAction)]
#[action_output(bool)]
struct PrimaryShortcutsModifier;

/// Non-consuming modifier action for Option (macOS) / Alt (other platforms).
#[derive(InputAction)]
#[action_output(bool)]
struct AltModifier;

/// Non-consuming modifier action for Ctrl on macOS (distinct from Cmd).
#[derive(InputAction)]
#[action_output(bool)]
struct ControlModifier;

/// Modifier-aware keybinding builder with platform-specific Cmd/Ctrl handling.
///
/// Spawns modifier actions and provides methods to bind keys with automatic
/// `BlockBy` application, preventing single-key actions from firing when
/// any modifier is held.
///
/// # Platform behavior
///
/// **macOS:**
/// - `PrimaryShortcutsModifier` = Cmd (Super) — for platform shortcuts
/// - `ControlModifier` = Ctrl — separate physical key, blocks single keys
/// - `AltModifier` = Option — blocks single keys
///
/// **Windows / Linux:**
/// - `PrimaryShortcutsModifier` = Ctrl — platform shortcuts AND single-key blocking
/// - `AltModifier` = Alt — blocks single keys
/// - `ControlModifier` is not spawned (Ctrl is already the primary modifier)
///
/// # Type parameters
///
/// - `C: Component` — the input context component for the action spawner
///
/// # Examples
///
/// ```ignore
/// use bevy_kana::input::Keybindings;
///
/// fn setup_bindings(spawner: &mut ActionSpawner<MyContext>) {
///     let kb = Keybindings::new::<ShiftAction>(spawner, ActionSettings::default());
///     kb.spawn_key::<JumpAction>(spawner, KeyCode::Space);
///     kb.spawn_platform_key::<SaveAction>(spawner, KeyCode::KeyS);
///     kb.spawn_shift_key::<RunAction>(spawner, KeyCode::KeyR);
/// }
/// ```
pub struct Keybindings<C: Component> {
    all_modifiers:       Vec<Entity>,
    non_shift_modifiers: Vec<Entity>,
    settings:            ActionSettings,
    marker:              std::marker::PhantomData<C>,
}

impl<C: Component> Keybindings<C> {
    /// Spawns modifier actions and returns a `Keybindings` ready for use.
    ///
    /// The `S` type parameter is the `InputAction` for the shift modifier.
    /// This allows the caller to query `Action<S>` to check shift state
    /// (e.g., for shift-click selection).
    pub fn new<S: InputAction>(spawner: &mut ActionSpawner<C>, settings: ActionSettings) -> Self {
        let non_consuming_modifier = ActionSettings {
            consume_input: false,
            require_reset: true,
            ..default()
        };
        let primary_modifier_bindings = if cfg!(target_os = "macos") {
            bindings![KeyCode::SuperLeft, KeyCode::SuperRight]
        } else {
            bindings![KeyCode::ControlLeft, KeyCode::ControlRight]
        };

        let shift = spawner
            .spawn((
                Action::<S>::new(),
                non_consuming_modifier,
                bindings![KeyCode::ShiftLeft, KeyCode::ShiftRight],
            ))
            .id();
        let primary = spawner
            .spawn((
                Action::<PrimaryShortcutsModifier>::new(),
                non_consuming_modifier,
                primary_modifier_bindings,
            ))
            .id();
        let alt = spawner
            .spawn((
                Action::<AltModifier>::new(),
                non_consuming_modifier,
                bindings![KeyCode::AltLeft, KeyCode::AltRight],
            ))
            .id();

        let mut all_modifiers = vec![shift, primary, alt];
        let mut non_shift_modifiers = vec![primary, alt];

        // On macOS, Ctrl is a separate physical key from Cmd — block it too.
        if cfg!(target_os = "macos") {
            let ctrl = spawner
                .spawn((
                    Action::<ControlModifier>::new(),
                    non_consuming_modifier,
                    bindings![KeyCode::ControlLeft, KeyCode::ControlRight],
                ))
                .id();
            all_modifiers.push(ctrl);
            non_shift_modifiers.push(ctrl);
        }

        Self {
            all_modifiers,
            non_shift_modifiers,
            settings,
            marker: std::marker::PhantomData,
        }
    }

    /// Spawn an action bound to a single key, blocked by all modifiers.
    pub fn spawn_key<A: InputAction>(&self, spawner: &mut ActionSpawner<C>, key: KeyCode) {
        spawner.spawn((
            Action::<A>::new(),
            self.settings,
            BlockBy::new(self.all_modifiers.clone()),
            bindings![key],
        ));
    }

    /// Spawn an action bound to Shift + key, blocked by non-shift modifiers only.
    pub fn spawn_shift_key<A: InputAction>(&self, spawner: &mut ActionSpawner<C>, key: KeyCode) {
        spawner.spawn((
            Action::<A>::new(),
            self.settings,
            BlockBy::new(self.non_shift_modifiers.clone()),
            bindings![key.with_mod_keys(ModKeys::SHIFT)],
        ));
    }

    /// Spawn an action with arbitrary bindings, blocked by all modifiers.
    pub fn spawn_binding<A: InputAction, B: Bundle>(
        &self,
        spawner: &mut ActionSpawner<C>,
        bindings: B,
    ) {
        spawner.spawn((
            Action::<A>::new(),
            self.settings,
            BlockBy::new(self.all_modifiers.clone()),
            bindings,
        ));
    }

    /// Spawn an action with platform Cmd/Ctrl modifier. No `BlockBy` needed
    /// since the modifier key itself is the disambiguator.
    pub fn spawn_platform_key<A: InputAction>(&self, spawner: &mut ActionSpawner<C>, key: KeyCode) {
        let platform_bindings = if cfg!(target_os = "macos") {
            bindings![key.with_mod_keys(ModKeys::SUPER)]
        } else {
            bindings![key.with_mod_keys(ModKeys::CONTROL)]
        };
        spawner.spawn((Action::<A>::new(), self.settings, platform_bindings));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    event!(TestEvent);
    event!(TestPayloadEvent { value: u32 });

    #[test]
    fn unit_event_defaults() {
        let event = TestEvent;
        assert_eq!(std::mem::size_of_val(&event), 0);
    }

    #[test]
    fn payload_event_fields() {
        let event = TestPayloadEvent { value: 42 };
        assert_eq!(event.value, 42);
    }
}
