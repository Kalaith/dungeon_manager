//! Notification rendering module
//!
//! Handles drawing in-game notifications for events.

use crate::state::game_state::GameState;
use macroquad::prelude::*;
use macroquad_toolkit::notifications::{draw_notification, NotificationRenderConfig};

/// Draw active notifications on screen
pub fn draw_notifications(state: &GameState) {
    let notifications = state.notifications.get_notifications();
    if notifications.is_empty() {
        return;
    }

    let config = NotificationRenderConfig {
        width: 300.0,
        row_height: 40.0,
        spacing: 10.0,
        font_size: 18.0,
        ..Default::default()
    };
    let start_x = screen_width() - config.width - 20.0;
    let start_y = crate::ui::core::HUD_HEIGHT + 20.0;

    for (i, notification) in notifications.iter().enumerate() {
        let y = start_y + (config.row_height + config.spacing) * i as f32;
        draw_notification(notification, start_x, y, &config);
    }
}
