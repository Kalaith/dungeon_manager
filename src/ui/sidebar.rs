//! Sidebar UI - Combined build menu, spell bar, and minion management
//! 
//! Unifies all player controls into a single bottom panel with tabs.

use macroquad::prelude::*;
use crate::InteractionMode;
use crate::state::player_state::PlayerState;
use crate::state::entities::{Entity, EntityId};
use crate::data::spells::SpellData;
use std::collections::HashMap;

const PANEL_HEIGHT: f32 = 180.0;
const TAB_HEIGHT: f32 = 30.0;
const BUTTON_SIZE: f32 = 48.0;
const BUTTON_SPACING: f32 = 10.0;
const PADDING: f32 = 10.0;

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarTab {
    Build,
    Magic,
    Minions,
    Traps,
}

pub struct Sidebar {
    current_tab: SidebarTab,
    is_expanded: bool,
    panel_y: f32, // Animated Y position
    
    // UI State
    selected_spell: Option<String>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            current_tab: SidebarTab::Build,
            is_expanded: true,
            panel_y: screen_height() - PANEL_HEIGHT,
            selected_spell: None,
        }
    }

    pub fn update_layout(&mut self) {
        let target_y = if self.is_expanded {
            screen_height() - PANEL_HEIGHT
        } else {
            screen_height() - TAB_HEIGHT
        };
        
        // Simple lerp for smooth animation could go here, for now snap
        self.panel_y = target_y;
    }

    pub fn handle_input(
        &mut self, 
        player: &PlayerState, 
        spells: &HashMap<String, SpellData>,
        _current_mode: &InteractionMode,
        held_entity: Option<EntityId>
    ) -> Option<InteractionMode> {
        let mouse_pos = mouse_position();
        
        // Handle Tab Switching
        if mouse_pos.1 >= self.panel_y - TAB_HEIGHT && mouse_pos.1 <= self.panel_y {
            if is_mouse_button_pressed(MouseButton::Left) {
                let tab_width = 100.0;
                let start_x = 20.0;
                
                if mouse_pos.0 >= start_x && mouse_pos.0 < start_x + tab_width {
                    self.current_tab = SidebarTab::Build;
                    self.is_expanded = true;
                } else if mouse_pos.0 >= start_x + tab_width && mouse_pos.0 < start_x + tab_width * 2.0 {
                    self.current_tab = SidebarTab::Magic;
                    self.is_expanded = true;
                } else if mouse_pos.0 >= start_x + tab_width * 2.0 && mouse_pos.0 < start_x + tab_width * 3.0 {
                    self.current_tab = SidebarTab::Minions;
                    self.is_expanded = true;
                } else if mouse_pos.0 >= start_x + tab_width * 3.0 && mouse_pos.0 < start_x + tab_width * 4.0 {
                    self.current_tab = SidebarTab::Traps;
                    self.is_expanded = true;
                } else if mouse_pos.0 >= screen_width() - 40.0 {
                    // Toggle expand/collapse
                    self.is_expanded = !self.is_expanded;
                }
            }
        }
        
        if !self.is_expanded {
            return None;
        }

        // Handle Content Clicks
        if mouse_pos.1 >= self.panel_y {
            if is_mouse_button_pressed(MouseButton::Left) {
                match self.current_tab {
                    SidebarTab::Build => {
                        return self.handle_build_tab_click(mouse_pos, player);
                    }
                    SidebarTab::Magic => {
                        return self.handle_magic_tab_click(mouse_pos, player, spells);
                    }
                    SidebarTab::Minions => {
                        return self.handle_minions_tab_click(mouse_pos, held_entity);
                    }
                    SidebarTab::Traps => {
                        return self.handle_traps_tab_click(mouse_pos, player);
                    }
                }
            }
        }
        
        // Handle Cancel / Deselect
        if is_key_pressed(KeyCode::Escape) || is_mouse_button_pressed(MouseButton::Right) {
             self.selected_spell = None;
             return Some(InteractionMode::None);
        }

        None
    }

    fn handle_build_tab_click(&mut self, mouse_pos: (f32, f32), player: &PlayerState) -> Option<InteractionMode> {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        
        let buttons = vec![
            ("Dig", InteractionMode::Dig, 0, "1"),
            ("Lair", InteractionMode::BuildRoom("lair".to_string()), 10, "2"),
            ("Hatchery", InteractionMode::BuildRoom("hatchery".to_string()), 15, "3"),
            ("Treasury", InteractionMode::BuildRoom("treasury".to_string()), 20, "4"),
            ("Training", InteractionMode::BuildRoom("training_room".to_string()), 25, "T"),
            ("Library", InteractionMode::BuildRoom("library".to_string()), 30, "L"),
            ("Workshop", InteractionMode::BuildRoom("workshop".to_string()), 40, "W"),
            ("Spawner", InteractionMode::PlaceSpawner, 50, "5"),
            ("Sell/Cancel", InteractionMode::Sell, 0, "X"),
        ];

        let mut current_x = start_x;
        let mut current_y = start_y;

        for (label, mode, cost, _hotkey) in buttons {
            // Suppress unused label warning
            let _ = label;
            if mouse_pos.0 >= current_x && mouse_pos.0 <= current_x + BUTTON_SIZE * 2.5
               && mouse_pos.1 >= current_y && mouse_pos.1 <= current_y + BUTTON_SIZE {
                
                if player.gold >= cost {
                    self.selected_spell = None; // Deselect spell if switching to build
                    return Some(mode);
                }
            }
            
            current_x += BUTTON_SIZE * 2.5 + BUTTON_SPACING;
            if current_x + BUTTON_SIZE * 2.5 > screen_width() {
                current_x = start_x;
                current_y += BUTTON_SIZE + BUTTON_SPACING;
            }
        }

        None
    }
    
    fn handle_magic_tab_click(&mut self, mouse_pos: (f32, f32), player: &PlayerState, spells: &HashMap<String, SpellData>) -> Option<InteractionMode> {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        
        let mut i = 0;
        // Hardcoded sort order for now, or just iterate unlocked
        let sorted_spells = vec!["heal", "lightning_strike", "summon_imps"]; // Speed removed
        
        for spell_id in sorted_spells {
            if !player.is_spell_unlocked(spell_id) { continue; }
            
            let btn_x = start_x + (BUTTON_SIZE + BUTTON_SPACING) * i as f32;
            let btn_y = start_y;
            
            if mouse_pos.0 >= btn_x && mouse_pos.0 <= btn_x + BUTTON_SIZE 
               && mouse_pos.1 >= btn_y && mouse_pos.1 <= btn_y + BUTTON_SIZE {
                   
                   // Check cost and cooldown
                   if let Some(data) = spells.get(spell_id) {
                       if player.gold >= data.cost.gold && player.mana >= data.cost.mana {
                           // Set selected spell
                           self.selected_spell = Some(spell_id.to_string());
                           // We don't change interaction mode immediately, it's handled via selected_spell state access
                           // But to be consistent with main loop, we might want a "CastMode" or similar, 
                           // but existing spell bar just returned SelectSpell.
                           // Let's keep using InteractionMode::None but main loop checks selected_spell.
                           return Some(InteractionMode::None);
                       }
                   }
            }
            i += 1;
        }
        
        None
    }

    fn handle_minions_tab_click(&mut self, mouse_pos: (f32, f32), held_entity: Option<EntityId>) -> Option<InteractionMode> {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;

        // Pickup / Drop Button
        if mouse_pos.0 >= start_x && mouse_pos.0 <= start_x + BUTTON_SIZE * 2.5
           && mouse_pos.1 >= start_y && mouse_pos.1 <= start_y + BUTTON_SIZE {
               if held_entity.is_some() {
                   return Some(InteractionMode::Drop);
               } else {
                   return Some(InteractionMode::Pickup);
               }
        }
        
        // Inspect Button
        let inspect_x = start_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING;
        if mouse_pos.0 >= inspect_x && mouse_pos.0 <= inspect_x + BUTTON_SIZE * 2.5
           && mouse_pos.1 >= start_y && mouse_pos.1 <= start_y + BUTTON_SIZE {
               return Some(InteractionMode::Inspect);
        }

        None
    }

    fn handle_traps_tab_click(&mut self, mouse_pos: (f32, f32), player: &PlayerState) -> Option<InteractionMode> {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        
        // Trap items: Label, Mode, Material Cost, Hotkey (optional)
        let buttons = vec![
            ("Door", InteractionMode::BuildTrap("door".to_string()), 5, "D"),
            ("Spike Trap", InteractionMode::BuildTrap("spike_trap".to_string()), 10, "S"),
        ];

        let mut current_x = start_x;
        let mut current_y = start_y;

        for (_label, mode, cost, _hotkey) in buttons {
            if mouse_pos.0 >= current_x && mouse_pos.0 <= current_x + BUTTON_SIZE * 2.5
               && mouse_pos.1 >= current_y && mouse_pos.1 <= current_y + BUTTON_SIZE {
                
                if player.materials >= cost {
                    self.selected_spell = None; 
                    return Some(mode);
                }
            }
            
            current_x += BUTTON_SIZE * 2.5 + BUTTON_SPACING;
            if current_x + BUTTON_SIZE * 2.5 > screen_width() {
                current_x = start_x;
                current_y += BUTTON_SIZE + BUTTON_SPACING;
            }
        }

        None
    }

    pub fn draw(&self, current_mode: &InteractionMode, player: &PlayerState, spells: &HashMap<String, SpellData>, held_entity: Option<EntityId>, selected_entity: Option<EntityId>, selected_room: Option<usize>, entities: &crate::state::entities::EntityManager, rooms: &[crate::engine::room_validator::Room]) {
        if !self.is_expanded {
            // Draw just tabs
            self.draw_tabs();
            return;
        }

        // Draw Panel Background
        draw_rectangle(0.0, self.panel_y, screen_width(), PANEL_HEIGHT, Color::new(0.1, 0.1, 0.12, 0.95));
        draw_line(0.0, self.panel_y, screen_width(), self.panel_y, 2.0, Color::new(0.3, 0.3, 0.35, 1.0));

        self.draw_tabs();

        // Draw Content based on Tab
        match self.current_tab {
            SidebarTab::Build => self.draw_build_content(current_mode, player),
            SidebarTab::Magic => self.draw_magic_content(player, spells),
            SidebarTab::Minions => self.draw_minions_content(current_mode, held_entity, selected_entity, selected_room, entities, rooms),
            SidebarTab::Traps => self.draw_traps_content(current_mode, player),
        }
        
        // Draw selected spell hint if any
        if let Some(spell_id) = &self.selected_spell {
            draw_text(
                &format!("Casting: {} (Left Click to Cast, Right Click to Cancel)", spell_id), 
                20.0, 
                self.panel_y - 40.0, 
                20.0, 
                WHITE
            );
        }
    }

    fn draw_tabs(&self) {
        let tabs = vec![
            (SidebarTab::Build, "Build"),
            (SidebarTab::Magic, "Magic"),
            (SidebarTab::Minions, "Minions"),
            (SidebarTab::Traps, "Traps"),
        ];

        let tab_width = 100.0;
        let start_x = 20.0;

        for (i, (tab, label)) in tabs.iter().enumerate() {
            let x = start_x + i as f32 * tab_width;
            let y = self.panel_y - TAB_HEIGHT;
            
            let is_active = self.current_tab == *tab && self.is_expanded;
            
            let color = if is_active {
                Color::new(0.1, 0.1, 0.12, 0.95) // Same as panel
            } else {
                Color::new(0.05, 0.05, 0.08, 0.9) // Darker
            };

            draw_rectangle(x, y, tab_width, TAB_HEIGHT, color);
            draw_rectangle_lines(x, y, tab_width, TAB_HEIGHT, 1.0, Color::new(0.3, 0.3, 0.35, 1.0));

            let text_color = if is_active { WHITE } else { GRAY };
            
            // Center text
            let text_size = measure_text(label, None, 16, 1.0);
            draw_text(
                label, 
                x + (tab_width - text_size.width) / 2.0, 
                y + 20.0, 
                16.0, 
                text_color
            );
        }
        
        // Draw toggle button
        let toggle_x = screen_width() - 40.0;
        let toggle_y = self.panel_y - TAB_HEIGHT;
        draw_rectangle(toggle_x, toggle_y, 40.0, TAB_HEIGHT, Color::new(0.2, 0.2, 0.2, 0.9));
        draw_text(
            if self.is_expanded { "v" } else { "^" },
            toggle_x + 15.0,
            toggle_y + 20.0,
            16.0,
            WHITE
        );
    }

    fn draw_build_content(&self, current_mode: &InteractionMode, player: &PlayerState) {
        let layout = vec![
            ("Dig", InteractionMode::Dig, 0, "1"),
            ("Lair", InteractionMode::BuildRoom("lair".to_string()), 10, "2"),
            ("Hatchery", InteractionMode::BuildRoom("hatchery".to_string()), 15, "3"),
            ("Treasury", InteractionMode::BuildRoom("treasury".to_string()), 20, "4"),
            ("Training", InteractionMode::BuildRoom("training_room".to_string()), 25, "T"),
            ("Library", InteractionMode::BuildRoom("library".to_string()), 30, "L"),
            ("Workshop", InteractionMode::BuildRoom("workshop".to_string()), 40, "W"),
            ("Spawner", InteractionMode::PlaceSpawner, 50, "5"),
            ("Sell/Cancel", InteractionMode::Sell, 0, "X"),
        ];

        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        let mut current_x = start_x;
        let mut current_y = start_y;

        for (label, mode, cost, hotkey) in layout {
            let is_selected = self.interaction_modes_match(current_mode, &mode);
            let can_afford = player.gold >= cost;

            let color = if is_selected {
                Color::new(0.2, 0.6, 0.3, 1.0)
            } else if !can_afford {
                Color::new(0.3, 0.1, 0.1, 1.0)
            } else {
                Color::new(0.25, 0.25, 0.3, 1.0)
            };

            draw_rectangle(current_x, current_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, color);
            draw_rectangle_lines(current_x, current_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, 2.0, Color::new(0.4, 0.4, 0.5, 1.0));

            draw_text(label, current_x + 5.0, current_y + 18.0, 16.0, WHITE);
            if cost > 0 {
                draw_text(&format!("{}g", cost), current_x + 5.0, current_y + 40.0, 14.0, GOLD);
            }
            draw_text(hotkey, current_x + BUTTON_SIZE * 2.5 - 15.0, current_y + 15.0, 12.0, GRAY);

            current_x += BUTTON_SIZE * 2.5 + BUTTON_SPACING;
            if current_x + BUTTON_SIZE * 2.5 > screen_width() {
                current_x = start_x;
                current_y += BUTTON_SIZE + BUTTON_SPACING;
            }
        }
    }
    
    fn draw_magic_content(&self, player: &PlayerState, spells: &HashMap<String, SpellData>) {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        
        let sorted_spells = vec!["heal", "lightning_strike", "summon_imps"];
        
        for (i, spell_id) in sorted_spells.iter().enumerate() {
            if !player.is_spell_unlocked(spell_id) { continue; }
            
            let btn_x = start_x + (BUTTON_SIZE + BUTTON_SPACING) * i as f32;
            let btn_y = start_y;
            
            let is_selected = self.selected_spell.as_ref().map(|s| s == *spell_id).unwrap_or(false);
            
            let color = if is_selected {
                Color::new(0.2, 0.6, 0.8, 1.0)
            } else {
                Color::new(0.2, 0.2, 0.25, 1.0)
            };
            
            draw_rectangle(btn_x, btn_y, BUTTON_SIZE, BUTTON_SIZE, color);
            draw_rectangle_lines(btn_x, btn_y, BUTTON_SIZE, BUTTON_SIZE, 2.0, WHITE);
            
            // Icon placeholder (first letter)
            let abbrev = &spell_id[0..1].to_uppercase();
            draw_text(abbrev, btn_x + 15.0, btn_y + 30.0, 24.0, WHITE);
            
            // Cost
            if let Some(data) = spells.get(*spell_id) {
                 draw_text(&format!("{}M", data.cost.mana), btn_x, btn_y + BUTTON_SIZE + 12.0, 12.0, BLUE);

                 // Draw cooldown overlay
                 if let Some(remaining) = player.spell_cooldowns.get(*spell_id) {
                     let max_cooldown = data.cooldown;
                     if max_cooldown > 0.0 {
                         let ratio = remaining / max_cooldown;
                         let h = BUTTON_SIZE * ratio;
                         
                         // Red overlay growing from bottom (or shrinking to bottom)
                         // "Goes fully red and then goes down" -> Fill decreases
                         // Draw rect from bottom up
                         let y_pos = btn_y + (BUTTON_SIZE - h);
                         
                         draw_rectangle(btn_x, y_pos, BUTTON_SIZE, h, Color::new(1.0, 0.0, 0.0, 0.5));
                     }
                 }
            }
        }
    }

    fn draw_minions_content(&self, current_mode: &InteractionMode, held_entity: Option<EntityId>, selected_entity: Option<EntityId>, selected_room: Option<usize>, entities: &crate::state::entities::EntityManager, rooms: &[crate::engine::room_validator::Room]) {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        
        // Pickup/Drop Button
        let pd_color = if self.interaction_modes_match(current_mode, &InteractionMode::Pickup) || 
                          self.interaction_modes_match(current_mode, &InteractionMode::Drop) {
            match current_mode {
                InteractionMode::Pickup => Color::new(0.2, 0.6, 0.3, 1.0), // Green for Pickup active
                InteractionMode::Drop => Color::new(0.6, 0.4, 0.2, 1.0), // Orange for Drop active
                _ => Color::new(0.25, 0.25, 0.3, 1.0)
            }
        } else {
            Color::new(0.25, 0.25, 0.3, 1.0)
        };
        
        let pd_label = if held_entity.is_some() { "Drop Minion" } else { "Pickup Minion" };
        
        draw_rectangle(start_x, start_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, pd_color);
        draw_rectangle_lines(start_x, start_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, 2.0, WHITE);
        draw_text(pd_label, start_x + 10.0, start_y + 30.0, 16.0, WHITE);
        
        // Inspect Button
        let inspect_x = start_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING;
        let inspect_color = if self.interaction_modes_match(current_mode, &InteractionMode::Inspect) {
            Color::new(0.2, 0.6, 0.8, 1.0)
        } else {
             Color::new(0.25, 0.25, 0.3, 1.0)
        };
        
        draw_rectangle(inspect_x, start_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, inspect_color);
        draw_rectangle_lines(inspect_x, start_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, 2.0, WHITE);
        draw_text("Inspect", inspect_x + 10.0, start_y + 30.0, 16.0, WHITE);

        // Minion Count info
        draw_text("Minion Controls", start_x, start_y + BUTTON_SIZE + 30.0, 18.0, LIGHTGRAY);
        
        // Selected Minion Details
        let details_x = inspect_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING * 2.0;
        if let Some(id) = selected_entity {
            if let Some(entity) = entities.get(id) {
                if let Some(creature) = entity.as_creature() {
                     draw_text(&format!("Selected: {} (Lvl {})", creature.creature_id, creature.level), details_x, start_y + 20.0, 20.0, WHITE);
                     draw_text(&format!("HP: {:.0}/{:.0} | Mood: {:.0}%", creature.health, creature.max_health, creature.mood), details_x, start_y + 45.0, 16.0, WHITE);
                     draw_text(&format!("Rest: {:.0}% | Food: {:.0}%", creature.get_need("sleep"), creature.get_need("food")), details_x, start_y + 65.0, 16.0, WHITE);
                     // eprintln!("DEBUG UI: ID={} Rest={:.2} Food={:.2}", creature.creature_id, creature.get_need("sleep"), creature.get_need("food"));
                     draw_text(&format!("Job: {:?}", creature.current_task), details_x, start_y + 85.0, 16.0, LIGHTGRAY);
                } else if let Some(hero) = entity.as_hero() {
                     draw_text(&format!("Hero: {} (Lvl {})", hero.hero_id, hero.level), details_x, start_y + 20.0, 20.0, WHITE);
                     draw_text(&format!("HP: {:.0}/{:.0}", hero.health, hero.max_health), details_x, start_y + 45.0, 16.0, WHITE);
                }
            }

        } else if let Some(room_id) = selected_room {
             if let Some(room) = rooms.get(room_id) {
                  // Room details
                  draw_text(&format!("Room: {} (ID: {})", room.room_type, room.id), details_x, start_y + 20.0, 20.0, WHITE);
                  draw_text(&format!("Size: {} tiles", room.tiles.len()), details_x, start_y + 45.0, 16.0, WHITE);
                  
                  // Efficiency color
                  let eff_color = if room.efficiency >= 0.9 {
                      GREEN
                  } else if room.efficiency >= 0.5 {
                      YELLOW
                  } else {
                      RED
                  };
                  
                  draw_text(&format!("Efficiency: {:.0}%", room.efficiency * 100.0), details_x, start_y + 65.0, 16.0, eff_color);
                  draw_text("Walls/doors needed for 100%", details_x, start_y + 85.0, 12.0, GRAY);
             }
        } else {
            draw_text("Select a unit or room to view details", details_x, start_y + 30.0, 18.0, GRAY);
        }
    }

    fn draw_traps_content(&self, current_mode: &InteractionMode, player: &PlayerState) {
        let layout = vec![
            ("Door", InteractionMode::BuildTrap("door".to_string()), 5, "D"),
            ("Spike Trap", InteractionMode::BuildTrap("spike_trap".to_string()), 10, "S"),
        ];

        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        let mut current_x = start_x;
        let mut current_y = start_y;

        for (label, mode, cost, hotkey) in layout {
            let is_selected = self.interaction_modes_match(current_mode, &mode);
            let can_afford = player.materials >= cost;

            let color = if is_selected {
                Color::new(0.2, 0.6, 0.3, 1.0)
            } else if !can_afford {
                Color::new(0.3, 0.1, 0.1, 1.0)
            } else {
                Color::new(0.25, 0.25, 0.3, 1.0)
            };

            draw_rectangle(current_x, current_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, color);
            draw_rectangle_lines(current_x, current_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, 2.0, Color::new(0.4, 0.4, 0.5, 1.0));

            draw_text(label, current_x + 5.0, current_y + 18.0, 16.0, WHITE);
            if cost > 0 {
                // Use M for materials
                draw_text(&format!("{} Mats", cost), current_x + 5.0, current_y + 40.0, 14.0, WHITE);
            }
            draw_text(hotkey, current_x + BUTTON_SIZE * 2.5 - 15.0, current_y + 15.0, 12.0, GRAY);

            current_x += BUTTON_SIZE * 2.5 + BUTTON_SPACING;
            if current_x + BUTTON_SIZE * 2.5 > screen_width() {
                current_x = start_x;
                current_y += BUTTON_SIZE + BUTTON_SPACING;
            }
        }
        
        // Show material count
        draw_text(
            &format!("Materials: {} / {}", player.materials, player.max_materials), 
            screen_width() - 250.0, 
            self.panel_y + 30.0, 
            20.0, 
            WHITE
        );
    }
    
    fn interaction_modes_match(&self, m1: &InteractionMode, m2: &InteractionMode) -> bool {
        match (m1, m2) {
            (InteractionMode::None, InteractionMode::None) => true,
            (InteractionMode::Dig, InteractionMode::Dig) => true,
            (InteractionMode::PlaceSpawner, InteractionMode::PlaceSpawner) => true,
            (InteractionMode::Pickup, InteractionMode::Pickup) => true,
            (InteractionMode::Drop, InteractionMode::Drop) => true,
            (InteractionMode::Sell, InteractionMode::Sell) => true,
            (InteractionMode::Inspect, InteractionMode::Inspect) => true,
            (InteractionMode::BuildRoom(a), InteractionMode::BuildRoom(b)) => a == b,
            (InteractionMode::BuildTrap(a), InteractionMode::BuildTrap(b)) => a == b,
            _ => false,
        }
    }
    
    pub fn get_selected_spell(&self) -> Option<&String> {
        self.selected_spell.as_ref()
    }
    
    pub fn is_mouse_over(&self) -> bool {
        let mouse_pos = mouse_position();
        mouse_pos.1 >= self.panel_y - TAB_HEIGHT
    }
    
    pub fn clear_selection(&mut self) {
        self.selected_spell = None;
    }
}
