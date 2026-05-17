//! Compatibility re-exports for notification state.
//!
//! Rendering remains game-specific in `ui::notifications`, but queue/update
//! behavior is now provided by `macroquad-toolkit`.

#[allow(unused_imports)]
pub use macroquad_toolkit::notifications::{
    Notification, NotificationManager, NotificationType, DEFAULT_DURATION, MAX_NOTIFICATIONS,
};
