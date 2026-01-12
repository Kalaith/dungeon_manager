//! Spell bar UI - displays available spells and handles casting

use macroquad::prelude::*;
use crate::state::player_state::PlayerState;
use crate::state::tile_state::TilePos;
use crate::state::entities::EntityId;
use crate::data::spells::SpellData;

const SPELL_BUTTON_SIZE: f32 = 60.0;
const SPELL_BUTTON_SPACING: f32 = 8.0;
const PANEL_PADDING: f32 = 10.0;

#[derive(Debug, Clone, PartialEq)]
pub enum SpellTarget {
    None,
    Tile(TilePos),
    Entity(EntityId),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpellAction {
    SelectSpell(String),
    CastSpell(String, SpellTarget),
    CancelCast,
}

pub struct SpellBar {
    available_spells: Vec<String>,
    selected_spell: Option<String>,
    panel_x: f32,
    panel_y: f32,
}

impl SpellBar {
    pub fn new() -> Self {
        Self {
            available_spells: vec![
                "speed_up".to_string(),
                "heal".to_string(),
                "lightning_strike".to_string(),
                "summon_imps".to_string(),
            ],
            selected_spell: None,
            panel_x: 0.0,
            panel_y: 0.0,
        }
    }

    /// Update panel position (call on window resize or init)
    pub fn update_position(&mut self) {
        let num_spells = self.available_spells.len();
        let panel_width =
            (SPELL_BUTTON_SIZE + SPELL_BUTTON_SPACING) * num_spells as f32 + PANEL_PADDING * 2.0;

        self.panel_x = screen_width() / 2.0 - panel_width / 2.0;
        self.panel_y = screen_height() - 100.0;
    }

    /// Handle input and return spell action if any
    pub fn handle_input(
        &mut self,
        player: &PlayerState,
        spells: &std::collections::HashMap<String, SpellData>,
        mouse_tile: Option<TilePos>,
    ) -> Option<SpellAction> {
        let mouse_pos = mouse_position();

        // Check if clicking on spell buttons
        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, spell_id) in self.available_spells.iter().enumerate() {
                let btn_x = self.panel_x + PANEL_PADDING + (SPELL_BUTTON_SIZE + SPELL_BUTTON_SPACING) * i as f32;
                let btn_y = self.panel_y + PANEL_PADDING;

                if mouse_pos.0 >= btn_x
                    && mouse_pos.0 <= btn_x + SPELL_BUTTON_SIZE
                    && mouse_pos.1 >= btn_y
                    && mouse_pos.1 <= btn_y + SPELL_BUTTON_SIZE
                {
                    // Check if spell is unlocked and affordable
                    if !player.is_spell_unlocked(spell_id) {
                        continue;
                    }

                    if let Some(spell_data) = spells.get(spell_id) {
                        if player.mana < spell_data.cost.mana || player.gold < spell_data.cost.gold {
                            continue;
                        }
                    }

                    // Select spell
                    self.selected_spell = Some(spell_id.clone());
                    return Some(SpellAction::SelectSpell(spell_id.clone()));
                }
            }
        }

        // If spell is selected and clicking on game world, try to cast
        if let Some(ref spell_id) = self.selected_spell {
            if is_mouse_button_pressed(MouseButton::Left) && !self.is_mouse_over_panel() {
                // Determine target type based on spell
                let spell_data = spells.get(spell_id);
                let target_type = spell_data.map(|s| s.targeting.target_type.as_str()).unwrap_or("tile");
                
                let target = match target_type {
                    "global" => {
                        // Global spells don't need a target
                        SpellTarget::None
                    }
                    "creature" => {
                        // For creature spells, we'd need entity detection
                        // For now, fall back to tile targeting with entity ID placeholder
                        // In a full implementation, we'd detect clicked entity
                        if let Some(tile) = mouse_tile {
                            // Placeholder: could extend to find entity at tile
                            SpellTarget::Tile(tile)
                        } else {
                            SpellTarget::None
                        }
                    }
                    _ => {
                        // Tile-based targeting
                        if let Some(tile) = mouse_tile {
                            SpellTarget::Tile(tile)
                        } else {
                            SpellTarget::None
                        }
                    }
                };
                
                let action = SpellAction::CastSpell(spell_id.clone(), target);
                self.selected_spell = None; // Clear selection after cast
                return Some(action);
            }

            // Cancel selection with right click or ESC
            if is_mouse_button_pressed(MouseButton::Right) || is_key_pressed(KeyCode::Escape) {
                self.selected_spell = None;
                return Some(SpellAction::CancelCast);
            }
        }

        None
    }
    
    /// Create a spell target for an entity (for creature-targeted spells)
    pub fn create_entity_target(entity_id: EntityId) -> SpellTarget {
        SpellTarget::Entity(entity_id)
    }
    
    /// Create a no-target for global spells
    pub fn create_global_target() -> SpellTarget {
        SpellTarget::None
    }

    /// Draw the spell bar
    pub fn draw(
        &self,
        player: &PlayerState,
        spells: &std::collections::HashMap<String, SpellData>,
    ) {
        let mouse_pos = mouse_position();

        // Draw panel background
        let num_spells = self.available_spells.len();
        let panel_width =
            (SPELL_BUTTON_SIZE + SPELL_BUTTON_SPACING) * num_spells as f32 + PANEL_PADDING * 2.0;
        let panel_height = SPELL_BUTTON_SIZE + PANEL_PADDING * 2.0 + 20.0;

        draw_rectangle(
            self.panel_x,
            self.panel_y,
            panel_width,
            panel_height,
            Color::new(0.1, 0.1, 0.15, 0.95),
        );

        draw_rectangle_lines(
            self.panel_x,
            self.panel_y,
            panel_width,
            panel_height,
            2.0,
            Color::new(0.3, 0.3, 0.4, 1.0),
        );

        // Draw spell buttons
        for (i, spell_id) in self.available_spells.iter().enumerate() {
            let btn_x =
                self.panel_x + PANEL_PADDING + (SPELL_BUTTON_SIZE + SPELL_BUTTON_SPACING) * i as f32;
            let btn_y = self.panel_y + PANEL_PADDING;

            let spell_data = spells.get(spell_id);

            // Check if spell is unlocked
            let is_unlocked = player.is_spell_unlocked(spell_id);

            // Check if on cooldown
            let on_cooldown = player.spell_cooldowns.contains_key(spell_id);
            let cooldown_remaining = player.spell_cooldowns.get(spell_id).copied().unwrap_or(0.0);

            // Check if affordable
            let can_afford = if let Some(spell) = spell_data {
                player.mana >= spell.cost.mana && player.gold >= spell.cost.gold
            } else {
                false
            };

            // Check if selected
            let is_selected = self
                .selected_spell
                .as_ref()
                .map(|s| s == spell_id)
                .unwrap_or(false);

            // Check if hovered
            let is_hovered = mouse_pos.0 >= btn_x
                && mouse_pos.0 <= btn_x + SPELL_BUTTON_SIZE
                && mouse_pos.1 >= btn_y
                && mouse_pos.1 <= btn_y + SPELL_BUTTON_SIZE;

            // Determine button color
            let button_color = if !is_unlocked {
                Color::new(0.2, 0.2, 0.2, 0.5) // Locked - dark gray
            } else if on_cooldown {
                Color::new(0.3, 0.2, 0.2, 0.8) // On cooldown - dark red
            } else if !can_afford {
                Color::new(0.3, 0.2, 0.2, 0.8) // Can't afford - dark red
            } else if is_selected {
                Color::new(0.3, 0.7, 0.3, 1.0) // Selected - green
            } else if is_hovered {
                Color::new(0.4, 0.4, 0.5, 1.0) // Hovered - light gray
            } else {
                Color::new(0.25, 0.25, 0.3, 1.0) // Default - dark gray
            };

            // Draw button background
            draw_rectangle(btn_x, btn_y, SPELL_BUTTON_SIZE, SPELL_BUTTON_SIZE, button_color);

            // Draw border
            let border_color = if is_selected {
                Color::new(0.5, 1.0, 0.5, 1.0) // Bright green
            } else if is_hovered && can_afford && is_unlocked && !on_cooldown {
                Color::new(0.6, 0.6, 0.7, 1.0) // Light border
            } else {
                Color::new(0.4, 0.4, 0.5, 1.0) // Normal border
            };
            draw_rectangle_lines(btn_x, btn_y, SPELL_BUTTON_SIZE, SPELL_BUTTON_SIZE, 2.0, border_color);

            // Draw spell name (abbreviated)
            let spell_name = spell_data.map(|s| s.name.as_str()).unwrap_or(spell_id);
            let abbreviated = if spell_name.len() > 8 {
                &spell_name[0..6]
            } else {
                spell_name
            };

            let text_color = if is_unlocked && can_afford && !on_cooldown {
                WHITE
            } else {
                Color::new(0.5, 0.5, 0.5, 1.0)
            };

            draw_text(
                abbreviated,
                btn_x + 5.0,
                btn_y + 20.0,
                14.0,
                text_color,
            );

            // Draw mana cost
            if let Some(spell) = spell_data {
                let cost_text = format!("{}M", spell.cost.mana);
                let cost_color = if player.mana >= spell.cost.mana {
                    Color::new(0.4, 0.6, 1.0, 1.0) // Blue
                } else {
                    Color::new(0.8, 0.3, 0.3, 1.0) // Red
                };

                draw_text(
                    &cost_text,
                    btn_x + 5.0,
                    btn_y + SPELL_BUTTON_SIZE - 8.0,
                    12.0,
                    cost_color,
                );
            }

            // Draw cooldown overlay
            if on_cooldown {
                let cooldown_height = (cooldown_remaining / 10.0) * SPELL_BUTTON_SIZE;
                draw_rectangle(
                    btn_x,
                    btn_y + SPELL_BUTTON_SIZE - cooldown_height,
                    SPELL_BUTTON_SIZE,
                    cooldown_height,
                    Color::new(0.0, 0.0, 0.0, 0.6),
                );

                draw_text(
                    &format!("{:.0}", cooldown_remaining),
                    btn_x + 20.0,
                    btn_y + 35.0,
                    16.0,
                    Color::new(1.0, 0.5, 0.5, 1.0),
                );
            }
        }

        // Draw hint if spell is selected
        if let Some(ref spell_id) = self.selected_spell {
            if let Some(spell_data) = spells.get(spell_id) {
                let hint_text = format!(
                    "{}: {} (Click to cast, ESC to cancel)",
                    spell_data.name, spell_data.description
                );

                let text_width = measure_text(&hint_text, None, 14, 1.0).width;
                draw_rectangle(
                    screen_width() / 2.0 - text_width / 2.0 - 10.0,
                    self.panel_y - 40.0,
                    text_width + 20.0,
                    30.0,
                    Color::new(0.1, 0.1, 0.15, 0.95),
                );

                draw_text(
                    &hint_text,
                    screen_width() / 2.0 - text_width / 2.0,
                    self.panel_y - 20.0,
                    14.0,
                    Color::new(0.9, 0.9, 1.0, 1.0),
                );
            }
        }
    }

    /// Check if mouse is over the spell bar panel
    pub fn is_mouse_over_panel(&self) -> bool {
        let mouse_pos = mouse_position();
        let num_spells = self.available_spells.len();
        let panel_width =
            (SPELL_BUTTON_SIZE + SPELL_BUTTON_SPACING) * num_spells as f32 + PANEL_PADDING * 2.0;
        let panel_height = SPELL_BUTTON_SIZE + PANEL_PADDING * 2.0 + 20.0;

        mouse_pos.0 >= self.panel_x
            && mouse_pos.0 <= self.panel_x + panel_width
            && mouse_pos.1 >= self.panel_y
            && mouse_pos.1 <= self.panel_y + panel_height
    }

    /// Get currently selected spell
    pub fn get_selected_spell(&self) -> Option<&String> {
        self.selected_spell.as_ref()
    }

    /// Clear spell selection
    pub fn clear_selection(&mut self) {
        self.selected_spell = None;
    }
}
