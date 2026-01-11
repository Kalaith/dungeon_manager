//! Build menu UI - Mouse-driven building interface

use macroquad::prelude::*;
use crate::InteractionMode;
use crate::state::player_state::PlayerState;

const BUTTON_WIDTH: f32 = 140.0;
const BUTTON_HEIGHT: f32 = 50.0;
const BUTTON_SPACING: f32 = 8.0;
const PANEL_PADDING: f32 = 10.0;

#[derive(Debug, Clone)]
pub struct BuildButton {
    pub label: String,
    pub mode: InteractionMode,
    pub cost: i32,
    pub hotkey: String,
}

impl BuildButton {
    pub fn new(label: &str, mode: InteractionMode, cost: i32, hotkey: &str) -> Self {
        Self {
            label: label.to_string(),
            mode,
            cost,
            hotkey: hotkey.to_string(),
        }
    }
}

pub struct BuildMenu {
    buttons: Vec<BuildButton>,
    panel_x: f32,
    panel_y: f32,
    panel_width: f32,
    panel_height: f32,
}

impl BuildMenu {
    pub fn new() -> Self {
        let buttons = vec![
            BuildButton::new("Dig", InteractionMode::Dig, 0, "1"),
            BuildButton::new("Lair", InteractionMode::BuildRoom("lair".to_string()), 10, "2"),
            BuildButton::new("Hatchery", InteractionMode::BuildRoom("hatchery".to_string()), 15, "3"),
            BuildButton::new("Treasury", InteractionMode::BuildRoom("treasury".to_string()), 20, "4"),
            BuildButton::new("Training Room", InteractionMode::BuildRoom("training_room".to_string()), 25, "T"),
            BuildButton::new("Library", InteractionMode::BuildRoom("library".to_string()), 30, "L"),
            BuildButton::new("Spawner", InteractionMode::PlaceSpawner, 50, "5"),
        ];

        let panel_width = BUTTON_WIDTH + PANEL_PADDING * 2.0;
        let panel_height = (BUTTON_HEIGHT + BUTTON_SPACING) * buttons.len() as f32 + PANEL_PADDING * 2.0;

        Self {
            buttons,
            panel_x: 10.0,
            panel_y: 80.0, // Below HUD
            panel_width,
            panel_height,
        }
    }

    /// Check for button clicks and return the clicked mode if any
    pub fn handle_input(&self, player: &PlayerState) -> Option<InteractionMode> {
        let mouse_pos = mouse_position();

        if !is_mouse_button_pressed(MouseButton::Left) {
            return None;
        }

        for (i, button) in self.buttons.iter().enumerate() {
            let btn_x = self.panel_x + PANEL_PADDING;
            let btn_y = self.panel_y + PANEL_PADDING + 30.0 + (BUTTON_HEIGHT + BUTTON_SPACING) * i as f32;

            // Check if mouse is hovering
            let is_hovered = mouse_pos.0 >= btn_x
                && mouse_pos.0 <= btn_x + BUTTON_WIDTH
                && mouse_pos.1 >= btn_y
                && mouse_pos.1 <= btn_y + BUTTON_HEIGHT;

            // Check if player can afford
            let can_afford = player.gold >= button.cost;

            if is_hovered && can_afford {
                return Some(button.mode.clone());
            }
        }

        None
    }

    /// Draw the build menu
    pub fn draw(&self, current_mode: &InteractionMode, player: &PlayerState) {
        let mouse_pos = mouse_position();

        // Draw panel background
        draw_rectangle(
            self.panel_x,
            self.panel_y,
            self.panel_width,
            self.panel_height,
            Color::new(0.1, 0.1, 0.15, 0.95),
        );

        // Draw panel border
        draw_rectangle_lines(
            self.panel_x,
            self.panel_y,
            self.panel_width,
            self.panel_height,
            2.0,
            Color::new(0.3, 0.3, 0.4, 1.0),
        );

        // Draw title
        draw_text(
            "BUILD MENU",
            self.panel_x + PANEL_PADDING,
            self.panel_y + PANEL_PADDING + 15.0,
            18.0,
            Color::new(0.9, 0.9, 1.0, 1.0),
        );

        // Draw buttons
        for (i, button) in self.buttons.iter().enumerate() {
            let btn_x = self.panel_x + PANEL_PADDING;
            let btn_y = self.panel_y + PANEL_PADDING + 30.0 + (BUTTON_HEIGHT + BUTTON_SPACING) * i as f32;

            // Check if mouse is hovering
            let is_hovered = mouse_pos.0 >= btn_x
                && mouse_pos.0 <= btn_x + BUTTON_WIDTH
                && mouse_pos.1 >= btn_y
                && mouse_pos.1 <= btn_y + BUTTON_HEIGHT;

            // Check if this mode is selected
            let is_selected = self.modes_match(current_mode, &button.mode);

            // Check if player can afford
            let can_afford = player.gold >= button.cost;

            // Determine button color
            let button_color = if is_selected {
                Color::new(0.3, 0.5, 0.8, 1.0) // Blue for selected
            } else if !can_afford {
                Color::new(0.3, 0.2, 0.2, 0.8) // Dark red for unaffordable
            } else if is_hovered {
                Color::new(0.4, 0.4, 0.5, 1.0) // Light gray for hover
            } else {
                Color::new(0.25, 0.25, 0.3, 1.0) // Dark gray default
            };

            // Draw button background
            draw_rectangle(btn_x, btn_y, BUTTON_WIDTH, BUTTON_HEIGHT, button_color);

            // Draw button border
            let border_color = if is_selected {
                Color::new(0.5, 0.7, 1.0, 1.0) // Bright blue border
            } else if is_hovered {
                Color::new(0.6, 0.6, 0.7, 1.0) // Light border
            } else {
                Color::new(0.4, 0.4, 0.5, 1.0) // Normal border
            };
            draw_rectangle_lines(btn_x, btn_y, BUTTON_WIDTH, BUTTON_HEIGHT, 2.0, border_color);

            // Draw button text
            let text_color = if can_afford {
                Color::new(0.9, 0.9, 1.0, 1.0)
            } else {
                Color::new(0.6, 0.4, 0.4, 1.0)
            };

            draw_text(
                &button.label,
                btn_x + 8.0,
                btn_y + 20.0,
                16.0,
                text_color,
            );

            // Draw cost
            let cost_text = if button.cost == 0 {
                "FREE".to_string()
            } else {
                format!("{}g", button.cost)
            };

            let cost_color = if button.cost == 0 {
                Color::new(0.3, 0.9, 0.3, 1.0) // Green for free
            } else if can_afford {
                Color::new(0.9, 0.8, 0.3, 1.0) // Gold color
            } else {
                Color::new(0.8, 0.3, 0.3, 1.0) // Red if can't afford
            };

            draw_text(
                &cost_text,
                btn_x + 8.0,
                btn_y + 38.0,
                14.0,
                cost_color,
            );

            // Draw hotkey
            draw_text(
                &format!("[{}]", button.hotkey),
                btn_x + BUTTON_WIDTH - 35.0,
                btn_y + 38.0,
                12.0,
                Color::new(0.6, 0.6, 0.7, 1.0),
            );
        }

        // Draw "ESC to cancel" at bottom
        draw_text(
            "ESC: Cancel",
            self.panel_x + PANEL_PADDING,
            self.panel_y + self.panel_height - 15.0,
            12.0,
            Color::new(0.7, 0.7, 0.8, 1.0),
        );
    }

    fn modes_match(&self, mode1: &InteractionMode, mode2: &InteractionMode) -> bool {
        match (mode1, mode2) {
            (InteractionMode::None, InteractionMode::None) => true,
            (InteractionMode::Dig, InteractionMode::Dig) => true,
            (InteractionMode::PlaceSpawner, InteractionMode::PlaceSpawner) => true,
            (InteractionMode::BuildRoom(r1), InteractionMode::BuildRoom(r2)) => r1 == r2,
            _ => false,
        }
    }

    /// Check if mouse is over the panel
    pub fn is_mouse_over_panel(&self) -> bool {
        let mouse_pos = mouse_position();
        mouse_pos.0 >= self.panel_x
            && mouse_pos.0 <= self.panel_x + self.panel_width
            && mouse_pos.1 >= self.panel_y
            && mouse_pos.1 <= self.panel_y + self.panel_height
    }
}
