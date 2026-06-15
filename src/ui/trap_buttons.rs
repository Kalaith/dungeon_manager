use crate::data::GameData;
use crate::state::player_state::PlayerState;
use crate::state::InteractionMode;
use crate::ui::sidebar::{BUTTON_SIZE, BUTTON_SPACING, PADDING, RIGHT_MARGIN};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct TrapButton {
    pub label: String,
    pub mode: InteractionMode,
    pub cost: i32,
    pub hotkey: String,
    pub unlocked: bool,
    pub stock: u32,
}

pub fn trap_button_layout(
    panel_y: f32,
    player: &PlayerState,
    game_data: &GameData,
) -> Vec<(TrapButton, Rect)> {
    let mut traps: Vec<&crate::data::traps::TrapData> = game_data.traps.values().collect();
    traps.sort_by(|a, b| {
        trap_sort_key(&a.category)
            .cmp(&trap_sort_key(&b.category))
            .then_with(|| a.cost.cmp(&b.cost))
            .then_with(|| a.name.cmp(&b.name))
    });

    let mut layout = Vec::new();
    let start_x = PADDING;
    let start_y = panel_y + PADDING;
    let mut current_x = start_x;
    let mut current_y = start_y;

    for trap in traps {
        let width = BUTTON_SIZE * 2.5;
        if current_x + width > screen_width() - RIGHT_MARGIN {
            current_x = start_x;
            current_y += BUTTON_SIZE + BUTTON_SPACING;
        }

        layout.push((
            TrapButton {
                label: trap.name.clone(),
                mode: InteractionMode::BuildTrap(trap.id.clone()),
                cost: trap.cost,
                hotkey: trap_hotkey(&trap.id),
                unlocked: player.is_trap_unlocked(&trap.id),
                stock: player.trap_inventory_count(&trap.id),
            },
            Rect::new(current_x, current_y, width, BUTTON_SIZE),
        ));

        current_x += width + BUTTON_SPACING;
    }

    layout
}

pub fn trap_button_at(
    panel_y: f32,
    mouse_pos: (f32, f32),
    player: &PlayerState,
    game_data: &GameData,
) -> Option<InteractionMode> {
    trap_button_layout(panel_y, player, game_data)
        .into_iter()
        .find(|(button, rect)| {
            button.unlocked
                && button.stock > 0
                && mouse_pos.0 >= rect.x
                && mouse_pos.0 <= rect.x + rect.w
                && mouse_pos.1 >= rect.y
                && mouse_pos.1 <= rect.y + rect.h
        })
        .map(|(button, _)| button.mode)
}

fn trap_sort_key(category: &str) -> u8 {
    match category {
        "door" | "defense" => 0,
        "offense" => 1,
        "utility" => 2,
        _ => 3,
    }
}

fn trap_hotkey(trap_id: &str) -> String {
    match trap_id {
        "door" | "wooden_door" => "D",
        "braced_door" => "B",
        "magic_door" => "M",
        "spike_trap" => "S",
        "blowgun_trap" => "G",
        "boulder_trap" => "O",
        "alarm_trap" => "A",
        _ => "",
    }
    .to_string()
}
