//! Notification system for game events
//! Displays toast-style messages to the player

use serde::{Deserialize, Serialize};

/// Type of notification for styling purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    /// Positive event (research complete, victory)
    Success,
    /// Neutral information (pay day, creature spawned)
    Info,
    /// Warning (low resources, creature unhappy)
    Warning,
    /// Negative event (creature died, trap triggered)
    Danger,
}

/// A single notification message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub message: String,
    pub notification_type: NotificationType,
    /// Time remaining before this notification disappears (seconds)
    pub time_remaining: f32,
    /// Total duration for fade calculations
    pub total_duration: f32,
}

impl Notification {
    pub fn new(message: String, notification_type: NotificationType, duration: f32) -> Self {
        Self {
            message,
            notification_type,
            time_remaining: duration,
            total_duration: duration,
        }
    }

    /// Get opacity for fade-out effect (1.0 = fully visible, 0.0 = invisible)
    pub fn opacity(&self) -> f32 {
        let fade_start = 1.0; // Start fading at 1 second remaining
        if self.time_remaining > fade_start {
            1.0
        } else {
            self.time_remaining / fade_start
        }
    }
}

/// Default notification duration in seconds
const DEFAULT_DURATION: f32 = 4.0;
/// Maximum number of notifications to display at once
const MAX_NOTIFICATIONS: usize = 5;

/// Manages the notification queue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationManager {
    notifications: Vec<Notification>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }

    /// Add a notification with default duration
    pub fn push(&mut self, message: impl Into<String>, notification_type: NotificationType) {
        self.push_with_duration(message, notification_type, DEFAULT_DURATION);
    }

    /// Add a notification with custom duration
    pub fn push_with_duration(&mut self, message: impl Into<String>, notification_type: NotificationType, duration: f32) {
        let notification = Notification::new(message.into(), notification_type, duration);
        self.notifications.push(notification);

        // Trim oldest if over limit
        while self.notifications.len() > MAX_NOTIFICATIONS {
            self.notifications.remove(0);
        }
    }

    /// Convenience methods for different notification types
    pub fn success(&mut self, message: impl Into<String>) {
        self.push(message, NotificationType::Success);
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.push(message, NotificationType::Info);
    }

    pub fn warning(&mut self, message: impl Into<String>) {
        self.push(message, NotificationType::Warning);
    }

    pub fn danger(&mut self, message: impl Into<String>) {
        self.push(message, NotificationType::Danger);
    }

    /// Update all notifications (call every frame)
    pub fn update(&mut self, dt: f32) {
        for notification in &mut self.notifications {
            notification.time_remaining -= dt;
        }

        // Remove expired notifications
        self.notifications.retain(|n| n.time_remaining > 0.0);
    }

    /// Get all active notifications for rendering
    pub fn get_notifications(&self) -> &[Notification] {
        &self.notifications
    }

    /// Check if there are any notifications
    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    /// Clear all notifications
    pub fn clear(&mut self) {
        self.notifications.clear();
    }
}
