//! Notification rendering module
//!
//! Handles drawing in-game notifications for events.

use macroquad::prelude::*;
use crate::state::game_state::GameState;
use crate::state::notifications::NotificationType;

/// Draw active notifications on screen
pub fn draw_notifications(state: &GameState) {
    let notifications = state.notifications.get_notifications();
    if notifications.is_empty() {
        return;
    }

    let notification_width = 300.0;
    let notification_height = 40.0;
    let padding = 10.0;
    let start_x = screen_width() - notification_width - 20.0;
    let start_y = crate::ui::core::HUD_HEIGHT + 20.0;

    for (i, notification) in notifications.iter().enumerate() {
        let y = start_y + (notification_height + padding) * i as f32;
        let opacity = notification.opacity();

        // Background color based on type
        let bg_color = match notification.notification_type {
            NotificationType::Success => Color::new(0.1, 0.5, 0.1, 0.9 * opacity),
            NotificationType::Info => Color::new(0.2, 0.3, 0.5, 0.9 * opacity),
            NotificationType::Warning => Color::new(0.6, 0.5, 0.1, 0.9 * opacity),
            NotificationType::Danger => Color::new(0.6, 0.1, 0.1, 0.9 * opacity),
        };

        // Border color
        let border_color = match notification.notification_type {
            NotificationType::Success => Color::new(0.2, 0.8, 0.2, opacity),
            NotificationType::Info => Color::new(0.3, 0.5, 0.8, opacity),
            NotificationType::Warning => Color::new(0.9, 0.7, 0.1, opacity),
            NotificationType::Danger => Color::new(0.9, 0.2, 0.2, opacity),
        };

        // Draw background
        draw_rectangle(start_x, y, notification_width, notification_height, bg_color);
        draw_rectangle_lines(start_x, y, notification_width, notification_height, 2.0, border_color);

        // Draw text
        let text_color = Color::new(1.0, 1.0, 1.0, opacity);
        draw_text(
            &notification.message,
            start_x + 10.0,
            y + 26.0,
            18.0,
            text_color,
        );
    }
}
