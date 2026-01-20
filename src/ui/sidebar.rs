//! Sidebar UI - Combined build menu, spell bar, and minion management
//! 
//! Unifies all player controls into a single bottom panel with tabs.

use macroquad::prelude::*;
use crate::InteractionMode;
use crate::state::player_state::PlayerState;
use crate::state::entities::EntityId;
use crate::data::spells::SpellData;
use std::collections::HashMap;

pub const PANEL_HEIGHT: f32 = 180.0;
pub const TAB_HEIGHT: f32 = 30.0;
pub const BUTTON_SIZE: f32 = 48.0;
pub const BUTTON_SPACING: f32 = 10.0;
pub const PADDING: f32 = 10.0;
pub const RIGHT_MARGIN: f32 = 170.0;

/// Get color based on efficiency value
pub fn efficiency_color(efficiency: f32) -> Color {
    if efficiency >= 0.9 { GREEN } else if efficiency >= 0.5 { YELLOW } else { RED }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarTab {
    Build,
    Magic,
    Minions,
    Traps,
    Research,
    Utils, // Save/Load, Settings, etc.
}

pub struct Sidebar {
    pub current_tab: SidebarTab,
    pub is_expanded: bool,
    pub panel_y: f32, // Animated Y position
    
    // UI State
    pub selected_spell: Option<String>,
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

    pub fn switch_to_tab(&mut self, tab: SidebarTab) {
        self.current_tab = tab;
        self.is_expanded = true;
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
        game_data: &crate::data::GameData,
        _current_mode: &InteractionMode,
        held_entity: Option<EntityId>,
        action_queue: &mut crate::ui::actions::ActionQueue,
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
                } else if mouse_pos.0 >= start_x + tab_width * 4.0 && mouse_pos.0 < start_x + tab_width * 5.0 {
                    self.current_tab = SidebarTab::Research;
                    self.is_expanded = true;
                } else if mouse_pos.0 >= start_x + tab_width * 5.0 && mouse_pos.0 < start_x + tab_width * 6.0 {
                    self.current_tab = SidebarTab::Utils;
                    self.is_expanded = true;
                } else if mouse_pos.0 >= screen_width() - RIGHT_MARGIN - 40.0 && mouse_pos.0 <= screen_width() - RIGHT_MARGIN {
                    // Toggle expand/collapse
                    self.is_expanded = !self.is_expanded;
                }
            }
        }
        
        if !self.is_expanded {
            return None;
        }

        // Handle Content Clicks
        if mouse_pos.1 >= self.panel_y && mouse_pos.0 < screen_width() - RIGHT_MARGIN {
            if is_mouse_button_pressed(MouseButton::Left) {
                match self.current_tab {
                    SidebarTab::Build => {
                        return self.handle_build_tab_click(mouse_pos, player, game_data);
                    }
                    SidebarTab::Magic => {
                        return self.handle_magic_tab_click(mouse_pos, player, &game_data.spells);
                    }
                    SidebarTab::Minions => {
                        return self.handle_minions_tab_click(mouse_pos, held_entity);
                    }
                    SidebarTab::Traps => {
                        return self.handle_traps_tab_click(mouse_pos, player, game_data);
                    }
                    SidebarTab::Research => {
                        self.handle_research_tab_click(mouse_pos, player, &game_data.technologies, action_queue);
                        return None;
                    }
                    SidebarTab::Utils => {
                        self.handle_utils_tab_click(mouse_pos, action_queue);
                        return None;
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

    fn handle_build_tab_click(&mut self, mouse_pos: (f32, f32), player: &PlayerState, game_data: &crate::data::GameData) -> Option<InteractionMode> {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        
        // Start with Dig
        let mut layout = vec![
            ("Dig".to_string(), InteractionMode::Dig, 0, "1".to_string()),
        ];
        
        // Add rooms dynamically from data
        let mut rooms: Vec<&crate::data::rooms::RoomData> = game_data.rooms.values().collect();
        // Sort by cost, then by name for stability
        rooms.sort_by(|a, b| {
            match a.build.cost_per_tile.cmp(&b.build.cost_per_tile) {
                std::cmp::Ordering::Equal => a.name.cmp(&b.name),
                other => other,
            }
        });

        for room in rooms {
            // Check if room is dungeon_heart - skip it
            if room.id == "dungeon_heart" {
                continue;
            }

            layout.push((
                room.name.clone(),
                InteractionMode::BuildRoom(room.id.clone()),
                room.build.cost_per_tile,
                String::new() // Hotkey doesn't matter for click handler
            ));
        }

        // Add utility items
        let spawner_cost = game_data.tiles.get("monster_spawner").and_then(|t| t.cost).unwrap_or(50);
        layout.push(("Spawner".to_string(), InteractionMode::PlaceSpawner, spawner_cost, "5".to_string()));
        layout.push(("Sell/Cancel".to_string(), InteractionMode::Sell, 0, "X".to_string()));

        let mut current_x = start_x;
        let mut current_y = start_y;

        for (label, mode, cost, _hotkey) in layout {
            // Suppress unused label warning
            let _ = label;
            
            let width = BUTTON_SIZE * 2.5;
            if current_x + width > screen_width() - RIGHT_MARGIN {
                current_x = start_x;
                current_y += BUTTON_SIZE + BUTTON_SPACING;
            }
            
            if mouse_pos.0 >= current_x && mouse_pos.0 <= current_x + width
               && mouse_pos.1 >= current_y && mouse_pos.1 <= current_y + BUTTON_SIZE {
                
                // Check if unlocked
                let is_locked = if let InteractionMode::BuildRoom(room_id) = &mode {
                    !player.is_room_unlocked(room_id)
                } else {
                    false
                };

                if !is_locked && player.gold >= cost {
                    self.selected_spell = None; // Deselect spell if switching to build
                    return Some(mode);
                }
            }
            
            current_x += width + BUTTON_SPACING;
        }

        None
    }
    
    fn get_room_cost(&self, room_id: &str, game_data: &crate::data::GameData) -> i32 {
        game_data.rooms.get(room_id)
            .map(|r| r.build.cost_per_tile)
            .unwrap_or_else(|| panic!("Room type '{}' defined in UI but missing in rooms.json", room_id))
    }
    
    fn handle_magic_tab_click(&mut self, mouse_pos: (f32, f32), player: &PlayerState, spells: &HashMap<String, SpellData>) -> Option<InteractionMode> {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;
        
        // Dynamic sorted list of unlocked spells
        let mut sorted_spells: Vec<&String> = spells.keys()
            .filter(|id| player.is_spell_unlocked(id))
            .collect();
            
        // Sort by mana cost, then name
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

        for (_i, spell_id) in sorted_spells.iter().enumerate() {
            let width = BUTTON_SIZE;
            
            if current_x + width > screen_width() - RIGHT_MARGIN {
                current_x = start_x;
                current_y += BUTTON_SIZE + BUTTON_SPACING;
            }
            
            if mouse_pos.0 >= current_x && mouse_pos.0 <= current_x + width 
               && mouse_pos.1 >= current_y && mouse_pos.1 <= current_y + BUTTON_SIZE {
                   
                   // Check cost and cooldown
                   if let Some(data) = spells.get(*spell_id) {
                       if player.gold >= data.cost.gold && player.mana >= data.cost.mana {
                           // Set selected spell
                           self.selected_spell = Some(spell_id.to_string());
                           return Some(InteractionMode::None);
                       }
                   }
            }
            
            current_x += width + BUTTON_SPACING;
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

        // Marker Buttons (New Line)
        let marker_y = start_y + BUTTON_SIZE + BUTTON_SPACING;
        
        // Attack Marker
        if mouse_pos.0 >= start_x && mouse_pos.0 <= start_x + BUTTON_SIZE * 2.5
           && mouse_pos.1 >= marker_y && mouse_pos.1 <= marker_y + BUTTON_SIZE {
               return Some(InteractionMode::SetAttackMarker);
        }

        // Defend Marker
        let defend_x = start_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING;
        if mouse_pos.0 >= defend_x && mouse_pos.0 <= defend_x + BUTTON_SIZE * 2.5
           && mouse_pos.1 >= marker_y && mouse_pos.1 <= marker_y + BUTTON_SIZE {
               return Some(InteractionMode::SetDefendMarker);
        }

        None
    }

    fn handle_traps_tab_click(&mut self, mouse_pos: (f32, f32), player: &PlayerState, game_data: &crate::data::GameData) -> Option<InteractionMode> {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;

        // Trap items: Label, Mode, Material Cost, Hotkey (optional)
        let door_cost = game_data.traps.get("door").map(|t| t.cost).unwrap_or(50);
        let spike_cost = game_data.traps.get("spike_trap").map(|t| t.cost).unwrap_or(100);
        let buttons = vec![
            ("Door", InteractionMode::BuildTrap("door".to_string()), door_cost, "D"),
            ("Spike Trap", InteractionMode::BuildTrap("spike_trap".to_string()), spike_cost, "S"),
        ];

        let mut current_x = start_x;
        let mut current_y = start_y;

        for (_label, mode, cost, _hotkey) in buttons {
            let width = BUTTON_SIZE * 2.5;
            if current_x + width > screen_width() - RIGHT_MARGIN {
                current_x = start_x;
                current_y += BUTTON_SIZE + BUTTON_SPACING;
            }

            if mouse_pos.0 >= current_x && mouse_pos.0 <= current_x + width
               && mouse_pos.1 >= current_y && mouse_pos.1 <= current_y + BUTTON_SIZE {
                
                if player.materials >= cost {
                    self.selected_spell = None; 
                    return Some(mode);
                }
            }
            
            current_x += width + BUTTON_SPACING;
        }

        None
    }

    pub fn draw(&self, current_mode: &InteractionMode, player: &PlayerState, game_data: &crate::data::GameData, held_entity: Option<EntityId>, selected_entity: Option<EntityId>, selected_room: Option<usize>, entities: &crate::state::entities::EntityManager, rooms: &[crate::engine::room_validator::Room], graphics: Option<&crate::ui::resources::GraphicsCache>) {
        crate::ui::sidebar_renderer::draw_sidebar(
            self,
            current_mode,
            player,
            game_data,
            held_entity,
            selected_entity,
            selected_room,
            entities,
            rooms,
            graphics
        );
    }
    
    pub fn handle_research_tab_click(
        &mut self,
        mouse_pos: (f32, f32),
        player: &PlayerState,
        technologies: &HashMap<String, crate::data::TechData>,
        action_queue: &mut crate::ui::actions::ActionQueue,
    ) {
        let start_x = PADDING;
        let start_y = self.panel_y + PADDING;

        let mut sorted_techs: Vec<&crate::data::TechData> = technologies.values().collect();
        sorted_techs.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());

        let mut current_x = start_x;
        let mut current_y = start_y;

        for tech in sorted_techs {
            let width = BUTTON_SIZE * 3.5;
            if current_x + width > screen_width() - RIGHT_MARGIN {
                current_x = start_x;
                current_y += BUTTON_SIZE * 1.2 + BUTTON_SPACING;
            }

            // Hitbox
            if mouse_pos.0 >= current_x && mouse_pos.0 <= current_x + width
               && mouse_pos.1 >= current_y && mouse_pos.1 <= current_y + BUTTON_SIZE * 1.2 {
                
                let is_completed = player.is_tech_completed(&tech.id);
                let is_active = player.active_research.as_ref().map(|id| id == &tech.id).unwrap_or(false);
                let prereqs_met = tech.prerequisites.iter().all(|req| player.is_tech_completed(req));

                if !is_completed && !is_active && prereqs_met {
                    // Start research!
                    action_queue.push(crate::ui::actions::UiAction::StartResearch(tech.id.clone()));
                }
            }

             // Move to next position (must match draw logic)
            current_x += width + BUTTON_SPACING;
        }
    }

    pub fn handle_utils_tab_click(
        &mut self,
        mouse_pos: (f32, f32),
        action_queue: &mut crate::ui::actions::ActionQueue,
    ) {
         let start_x = PADDING;
         let start_y = self.panel_y + PADDING;

         // Save Game Button
         let btn_width = 150.0;
         let btn_height = BUTTON_SIZE;
         let spacing = 20.0;
         
         let save_x = start_x;
         let save_y = start_y;
         
         if mouse_pos.0 >= save_x && mouse_pos.0 <= save_x + btn_width
            && mouse_pos.1 >= save_y && mouse_pos.1 <= save_y + btn_height {
                action_queue.push(crate::ui::actions::UiAction::SaveGame);
         }
         
         // Load Game Button
         let load_x = save_x + btn_width + spacing;
         let load_y = start_y;
         
         if mouse_pos.0 >= load_x && mouse_pos.0 <= load_x + btn_width
            && mouse_pos.1 >= load_y && mouse_pos.1 <= load_y + btn_height {
                action_queue.push(crate::ui::actions::UiAction::LoadGame);
         }
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
            (InteractionMode::SaveGame, InteractionMode::SaveGame) => true,
            // Mode doesn't really matter for Load as it overrides everything immediately
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
