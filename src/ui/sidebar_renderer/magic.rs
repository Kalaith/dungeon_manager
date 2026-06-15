use crate::data::spells::SpellData;
use crate::state::player_state::PlayerState;
use crate::ui::sidebar::{Sidebar, BUTTON_SIZE, BUTTON_SPACING, PADDING, RIGHT_MARGIN};
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;
use std::collections::HashMap;

pub(super) fn draw_magic_content(
    sidebar: &Sidebar,
    player: &PlayerState,
    spells: &HashMap<String, SpellData>,
    graphics: Option<&crate::ui::resources::GraphicsCache>,
) {
    let start_x = PADDING;
    let start_y = sidebar.panel_y + PADDING;

    let mut sorted_spells: Vec<&String> = spells
        .keys()
        .filter(|id| player.is_spell_unlocked(id))
        .collect();

    sorted_spells.sort_by(|a, b| {
        let cost_a = spells.get(*a).map(|s| s.cost.mana).unwrap_or(0);
        let cost_b = spells.get(*b).map(|s| s.cost.mana).unwrap_or(0);
        match cost_a.cmp(&cost_b) {
            std::cmp::Ordering::Equal => a.cmp(b),
            other => other,
        }
    });

    let mut current_x = start_x;
    let mut current_y = start_y;

    for spell_id in sorted_spells.iter() {
        let width = BUTTON_SIZE;

        if current_x + width > screen_width() - RIGHT_MARGIN {
            current_x = start_x;
            current_y += BUTTON_SIZE + BUTTON_SPACING;
        }

        let btn_x = current_x;
        let btn_y = current_y;

        let is_selected = sidebar
            .selected_spell
            .as_ref()
            .map(|s| s == *spell_id)
            .unwrap_or(false);

        let color = if is_selected {
            Color::new(0.2, 0.6, 0.8, 1.0)
        } else {
            Color::new(0.15, 0.15, 0.2, 1.0)
        };

        draw_rectangle(btn_x, btn_y, BUTTON_SIZE, BUTTON_SIZE, color);
        draw_rectangle_lines(btn_x, btn_y, BUTTON_SIZE, BUTTON_SIZE, 2.0, WHITE);

        let mut icon_drawn = false;
        if let Some(cache) = graphics {
            if let Some(data) = spells.get(*spell_id) {
                let icon_path = &data.visual.icon;
                if !icon_path.is_empty() {
                    if let Some(tex) = cache.ui_textures.get(icon_path) {
                        draw_texture_ex(
                            tex,
                            btn_x + 4.0,
                            btn_y + 4.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(BUTTON_SIZE - 8.0, BUTTON_SIZE - 8.0)),
                                ..Default::default()
                            },
                        );
                        icon_drawn = true;
                    }
                }
            }
        }

        if !icon_drawn {
            let abbrev = &spell_id[0..1].to_uppercase();
            draw_ui_text(abbrev, btn_x + 15.0, btn_y + 30.0, 24.0, WHITE);
        }

        if let Some(data) = spells.get(*spell_id) {
            draw_ui_text(
                &format!("{}M", data.cost.mana),
                btn_x,
                btn_y + BUTTON_SIZE + 12.0,
                12.0,
                BLUE,
            );

            if let Some(remaining) = player.spell_cooldowns.get(*spell_id) {
                let max_cooldown = data.cooldown;
                if max_cooldown > 0.0 {
                    let ratio = remaining / max_cooldown;
                    let height = BUTTON_SIZE * ratio;
                    let y_pos = btn_y + (BUTTON_SIZE - height);
                    draw_rectangle(
                        btn_x,
                        y_pos,
                        BUTTON_SIZE,
                        height,
                        Color::new(1.0, 0.0, 0.0, 0.5),
                    );
                }
            }
        }

        current_x += width + BUTTON_SPACING;
    }
}
