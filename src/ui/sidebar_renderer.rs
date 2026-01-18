use macroquad::prelude::*;
use crate::InteractionMode;
use crate::state::player_state::PlayerState;
use crate::state::entities::EntityId;
use crate::data::spells::SpellData;
use std::collections::HashMap;
use crate::ui::sidebar::{Sidebar, SidebarTab, PANEL_HEIGHT, TAB_HEIGHT, BUTTON_SIZE, BUTTON_SPACING, PADDING, RIGHT_MARGIN, efficiency_color};

pub fn draw_sidebar(
    sidebar: &Sidebar,
    current_mode: &InteractionMode,
    player: &PlayerState,
    game_data: &crate::data::GameData,
    held_entity: Option<EntityId>,
    selected_entity: Option<EntityId>,
    selected_room: Option<usize>,
    entities: &crate::state::entities::EntityManager,
    rooms: &[crate::engine::room_validator::Room]
) {
    if !sidebar.is_expanded {
        // Draw just tabs
        draw_tabs(sidebar);
        return;
    }

    // Draw Panel Background
    draw_rectangle(0.0, sidebar.panel_y, screen_width() - RIGHT_MARGIN, PANEL_HEIGHT, Color::new(0.1, 0.1, 0.12, 0.95));
    draw_line(0.0, sidebar.panel_y, screen_width() - RIGHT_MARGIN, sidebar.panel_y, 2.0, Color::new(0.3, 0.3, 0.35, 1.0));

    draw_tabs(sidebar);

    // Draw Content based on Tab
    match sidebar.current_tab {
        SidebarTab::Build => draw_build_content(sidebar, current_mode, player, game_data),
        SidebarTab::Magic => draw_magic_content(sidebar, player, &game_data.spells),
        SidebarTab::Minions => draw_minions_content(sidebar, current_mode, held_entity, selected_entity, selected_room, entities, rooms),
        SidebarTab::Traps => draw_traps_content(sidebar, current_mode, player, game_data),
        SidebarTab::Research => draw_research_content(sidebar, player, &game_data.technologies),
        SidebarTab::Utils => draw_utils_content(sidebar, current_mode),
    }
    
    // Draw selected spell hint if any
    if let Some(spell_id) = &sidebar.selected_spell {
        draw_text(
            &format!("Casting: {} (Left Click to Cast, Right Click to Cancel)", spell_id), 
            20.0, 
            sidebar.panel_y - 40.0, 
            20.0, 
            WHITE
        );
    }
}

fn draw_tabs(sidebar: &Sidebar) {
    let tabs = vec![
        (SidebarTab::Build, "Build"),
        (SidebarTab::Magic, "Magic"),
        (SidebarTab::Minions, "Inspect"),
        (SidebarTab::Traps, "Traps"),
        (SidebarTab::Research, "Tech"),
        (SidebarTab::Utils, "Utils"),
    ];

    let tab_width = 100.0;
    let start_x = 20.0;

    for (i, (tab, label)) in tabs.iter().enumerate() {
        let x = start_x + i as f32 * tab_width;
        let y = sidebar.panel_y - TAB_HEIGHT;
        
        let is_active = sidebar.current_tab == *tab && sidebar.is_expanded;
        
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
    let toggle_x = screen_width() - RIGHT_MARGIN - 40.0;
    let toggle_y = sidebar.panel_y - TAB_HEIGHT;
    draw_rectangle(toggle_x, toggle_y, 40.0, TAB_HEIGHT, Color::new(0.2, 0.2, 0.2, 0.9));
    draw_text(
        if sidebar.is_expanded { "v" } else { "^" },
        toggle_x + 15.0,
        toggle_y + 20.0,
        16.0,
        WHITE
    );
}

fn draw_utils_content(sidebar: &Sidebar, current_mode: &InteractionMode) {
     let start_x = PADDING;
     let start_y = sidebar.panel_y + PADDING;

     // Save Game Button
     let save_btn_width = 150.0;
     let save_btn_height = BUTTON_SIZE;
     let save_x = start_x;
     let save_y = start_y;
     
     let is_save_selected = interaction_modes_match(current_mode, &InteractionMode::SaveGame);
     
     let save_color = if is_save_selected {
         Color::new(0.2, 0.6, 0.3, 1.0)
     } else {
         Color::new(0.25, 0.25, 0.3, 1.0)
     };

     draw_rectangle(save_x, save_y, save_btn_width, save_btn_height, save_color);
     draw_rectangle_lines(save_x, save_y, save_btn_width, save_btn_height, 2.0, WHITE);
     draw_text("Save Game", save_x + 30.0, save_y + 30.0, 16.0, WHITE);
}

fn interaction_modes_match(m1: &InteractionMode, m2: &InteractionMode) -> bool {
    m1 == m2
}
fn draw_build_content(sidebar: &Sidebar, current_mode: &InteractionMode, player: &PlayerState, game_data: &crate::data::GameData) {
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
        // Skip dungeon heart
        if room.id == "dungeon_heart" {
            continue;
        }

        // Assign hotkeys for common rooms to maintain familiarity
        let hotkey = match room.id.as_str() {
            "lair" => "2",
            "hatchery" => "3",
            "treasury" => "4",
            "training_hall" => "T",
            "library" => "L",
            "workshop" => "W",
            "guard_post" => "G",
            "prison" => "P",
            _ => "",
        }.to_string();

        layout.push((
            room.name.clone(),
            InteractionMode::BuildRoom(room.id.clone()),
            room.build.cost_per_tile,
            hotkey
        ));
    }

    // Add utility items
    let spawner_cost = game_data.tiles.get("monster_spawner").and_then(|t| t.cost).unwrap_or(50);
    layout.push(("Spawner".to_string(), InteractionMode::PlaceSpawner, spawner_cost, "5".to_string()));
    layout.push(("Sell/Cancel".to_string(), InteractionMode::Sell, 0, "X".to_string()));

    let start_x = PADDING;
    let start_y = sidebar.panel_y + PADDING;
    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut tooltip_to_draw: Option<((f32, f32), Vec<String>)> = None;

    for (label, mode, cost, hotkey) in layout {
        let width = BUTTON_SIZE * 2.5;
        if current_x + width > screen_width() - RIGHT_MARGIN {
            current_x = start_x;
            current_y += BUTTON_SIZE + BUTTON_SPACING;
        }

        let is_selected = interaction_modes_match(current_mode, &mode);
        let can_afford = player.gold >= cost;
        
        // Check lock status
        let is_locked = if let InteractionMode::BuildRoom(room_id) = &mode {
            !player.is_room_unlocked(room_id)
        } else {
            false
        };

        let color = if is_locked {
             Color::new(0.1, 0.1, 0.1, 0.8) // Dark Gray for Locked
        } else if is_selected {
            Color::new(0.2, 0.6, 0.3, 1.0)
        } else if !can_afford {
            Color::new(0.3, 0.1, 0.1, 1.0)
        } else {
            Color::new(0.25, 0.25, 0.3, 1.0)
        };

        draw_rectangle(current_x, current_y, width, BUTTON_SIZE, color);
        draw_rectangle_lines(current_x, current_y, width, BUTTON_SIZE, 2.0, Color::new(0.4, 0.4, 0.5, 1.0));

        draw_text(&label, current_x + 5.0, current_y + 18.0, 16.0, if is_locked { GRAY } else { WHITE });
        if is_locked {
             draw_text("LOCKED", current_x + 5.0, current_y + 40.0, 14.0, RED);
        } else {
            if cost > 0 {
                draw_text(&format!("{}g", cost), current_x + 5.0, current_y + 40.0, 14.0, GOLD);
            }
            if !hotkey.is_empty() {
                draw_text(&hotkey, current_x + width - 15.0, current_y + 15.0, 12.0, GRAY);
            }
        }

        // Tooltip logic for locked rooms
        let mouse = mouse_position();
        let is_hovered = mouse.0 >= current_x && mouse.0 <= current_x + width &&
                         mouse.1 >= current_y && mouse.1 <= current_y + BUTTON_SIZE;

        if is_hovered && is_locked {
             if let InteractionMode::BuildRoom(room_id) = &mode {
                 let mut lines = Vec::new();
                 lines.push("LOCKED".to_string());
                 
                 if let Some(room_data) = game_data.rooms.get(room_id) {
                     // Research Requirements
                     for tech_id in &room_data.requirements.research {
                         let tech_name = game_data.technologies.get(tech_id)
                             .map(|t| t.name.clone())
                             .unwrap_or(tech_id.clone());
                         lines.push(format!("Requires: {}", tech_name));
                     }
                     
                     // Room requirements
                     for req_room_id in &room_data.requirements.global_rooms_required {
                          let room_name = game_data.rooms.get(req_room_id)
                              .map(|r| r.name.clone())
                              .unwrap_or(req_room_id.clone());
                          lines.push(format!("Requires: {}", room_name));
                     }
                 }
                 
                 tooltip_to_draw = Some((mouse, lines));
             }
        }

        current_x += width + BUTTON_SPACING;
    }

    if let Some((pos, lines)) = tooltip_to_draw {
        draw_tooltip(pos, lines);
    }
}

fn draw_tooltip(pos: (f32, f32), lines: Vec<String>) {
    if lines.is_empty() { return; }
    
    let font_size = 18.0;
    let padding = 8.0;
    let mut max_width = 0.0f32;
    
    for line in &lines {
        let dims = measure_text(line, None, font_size as u16, 1.0);
        if dims.width > max_width {
            max_width = dims.width;
        }
    }
    
    let box_width = max_width + padding * 2.0;
    let box_height = (font_size + 4.0) * lines.len() as f32 + padding * 2.0;
    
    // Offset slightly
    let mut draw_x = pos.0 + 15.0;
    let mut draw_y = pos.1 + 15.0;

    // Prevent going off screen
    if draw_x + box_width > screen_width() {
        draw_x = pos.0 - box_width - 5.0;
    }
    if draw_y + box_height > screen_height() {
        draw_y = pos.1 - box_height - 5.0;
    }
    
    // Draw background
    draw_rectangle(draw_x, draw_y, box_width, box_height, Color::new(0.1, 0.1, 0.1, 0.95));
    draw_rectangle_lines(draw_x, draw_y, box_width, box_height, 1.0, WHITE);
    
    for (i, line) in lines.iter().enumerate() {
        let color = if i == 0 { RED } else { WHITE }; // Header is red
        draw_text(
            line,
            draw_x + padding,
            draw_y + padding + (i as f32 * (font_size + 4.0)) + font_size - 4.0,
            font_size,
            color,
        );
    }
}

fn draw_magic_content(sidebar: &Sidebar, player: &PlayerState, spells: &HashMap<String, SpellData>) {
    let start_x = PADDING;
    let start_y = sidebar.panel_y + PADDING;
    
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
        
        let btn_x = current_x;
        let btn_y = current_y;
        
        let is_selected = sidebar.selected_spell.as_ref().map(|s| s == *spell_id).unwrap_or(false);
        
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
                     let y_pos = btn_y + (BUTTON_SIZE - h);
                     
                     draw_rectangle(btn_x, y_pos, BUTTON_SIZE, h, Color::new(1.0, 0.0, 0.0, 0.5));
                 }
             }
        }
        
        current_x += width + BUTTON_SPACING;
    }
}

fn draw_minions_content(sidebar: &Sidebar, current_mode: &InteractionMode, held_entity: Option<EntityId>, selected_entity: Option<EntityId>, selected_room: Option<usize>, entities: &crate::state::entities::EntityManager, rooms: &[crate::engine::room_validator::Room]) {
    let start_x = PADDING;
    let start_y = sidebar.panel_y + PADDING;
    
    // Pickup/Drop Button
    let pd_color = if interaction_modes_match(current_mode, &InteractionMode::Pickup) || 
                      interaction_modes_match(current_mode, &InteractionMode::Drop) {
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
    let inspect_color = if interaction_modes_match(current_mode, &InteractionMode::Inspect) {
        Color::new(0.2, 0.6, 0.8, 1.0)
    } else {
            Color::new(0.25, 0.25, 0.3, 1.0)
    };
    
    draw_rectangle(inspect_x, start_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, inspect_color);
    draw_rectangle_lines(inspect_x, start_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, 2.0, WHITE);
    draw_text("Inspect", inspect_x + 10.0, start_y + 30.0, 16.0, WHITE);

    // Marker Buttons (New Line)
    let marker_y = start_y + BUTTON_SIZE + BUTTON_SPACING;

    // Attack Marker
    let attack_color = if interaction_modes_match(current_mode, &InteractionMode::SetAttackMarker) {
        Color::new(0.7, 0.2, 0.2, 1.0)
    } else {
            Color::new(0.4, 0.2, 0.2, 1.0)
    };
    draw_rectangle(start_x, marker_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, attack_color);
    draw_rectangle_lines(start_x, marker_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, 2.0, WHITE);
    draw_text("Set Attack", start_x + 10.0, marker_y + 30.0, 16.0, WHITE);

    // Defend Marker
    let defend_x = start_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING;
    let defend_color = if interaction_modes_match(current_mode, &InteractionMode::SetDefendMarker) {
        Color::new(0.2, 0.2, 0.7, 1.0)
    } else {
            Color::new(0.2, 0.2, 0.4, 1.0)
    };
    draw_rectangle(defend_x, marker_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, defend_color);
    draw_rectangle_lines(defend_x, marker_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, 2.0, WHITE);
    draw_text("Set Defend", defend_x + 10.0, marker_y + 30.0, 16.0, WHITE);

    // Minion Count info
    draw_text("Selection Controls", start_x, start_y + (BUTTON_SIZE * 2.0) + 40.0, 18.0, LIGHTGRAY);
    
    // Selected Minion Details
    let details_x = inspect_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING * 2.0;
    draw_selection_details(details_x, start_y, selected_entity, selected_room, entities, rooms);
}

fn draw_selection_details(details_x: f32, start_y: f32, selected_entity: Option<EntityId>, selected_room: Option<usize>, entities: &crate::state::entities::EntityManager, rooms: &[crate::engine::room_validator::Room]) {
    if let Some(id) = selected_entity {
        draw_entity_details(details_x, start_y, id, entities);
    } else if let Some(room_id) = selected_room {
        draw_room_details(details_x, start_y, room_id, rooms);
    } else {
        draw_text("Select a unit or room to view details", details_x, start_y + 30.0, 18.0, GRAY);
    }
}

fn draw_entity_details(details_x: f32, start_y: f32, id: EntityId, entities: &crate::state::entities::EntityManager) {
    let entity = match entities.get(id) {
        Some(e) => e,
        None => return,
    };

    if let Some(creature) = entity.as_creature() {
        draw_text(&format!("Selected: {} (Lvl {})", creature.creature_id, creature.level), details_x, start_y + 20.0, 20.0, WHITE);
        draw_text(&format!("HP: {:.0}/{:.0} | Mood: {:.0}%", creature.health, creature.max_health, creature.mood), details_x, start_y + 45.0, 16.0, WHITE);
        draw_text(&format!("Rest: {:.0}% | Food: {:.0}%", creature.get_need("sleep"), creature.get_need("food")), details_x, start_y + 65.0, 16.0, WHITE);
        draw_text(&format!("Job: {:?}", creature.current_task), details_x, start_y + 85.0, 16.0, LIGHTGRAY);
        return;
    }

    if let Some(hero) = entity.as_hero() {
        draw_text(&format!("Hero: {} (Lvl {})", hero.hero_id, hero.level), details_x, start_y + 20.0, 20.0, WHITE);

        let hp_pct = hero.health / hero.max_health;
        let bar_w = 200.0;
        draw_rectangle(details_x, start_y + 30.0, bar_w, 10.0, RED);
        draw_rectangle(details_x, start_y + 30.0, bar_w * hp_pct, 10.0, GREEN);
        draw_text(&format!("{:.0}/{:.0} HP", hero.health, hero.max_health), details_x + 5.0, start_y + 39.0, 10.0, WHITE);

        let role = if hero.is_defender { "Defender" } else { "Attacker" };
        let wave_info = if hero.wave_assigned > 0 { format!(" (Wave {})", hero.wave_assigned) } else { String::new() };
        draw_text(&format!("Role: {}{}", role, wave_info), details_x, start_y + 55.0, 16.0, WHITE);

        let status = if hero.is_digging { "Digging" } else if hero.current_path.is_some() { "Moving" } else { "Idle" };
        draw_text(&format!("Status: {} | Goal: {:?}", status, hero.current_goal), details_x, start_y + 75.0, 14.0, LIGHTGRAY);
        draw_text(&format!("Kills: {} | Gold: {}", hero.kills, hero.gold_stolen), details_x, start_y + 95.0, 14.0, GOLD);

        if hero.is_fleeing {
            draw_text("FLEEING!", details_x + 150.0, start_y + 55.0, 16.0, RED);
        }
    }
}

fn draw_room_details(details_x: f32, start_y: f32, room_id: usize, rooms: &[crate::engine::room_validator::Room]) {
    let room = match rooms.iter().find(|r| r.id == room_id) {
        Some(r) => r,
        None => return,
    };

    draw_text(&format!("Room: {} (ID: {})", room.room_type, room.id), details_x, start_y + 20.0, 20.0, WHITE);
    draw_text(&format!("Size: {} tiles", room.tiles.len()), details_x, start_y + 45.0, 16.0, WHITE);

    let eff_color = efficiency_color(room.efficiency);
    draw_text(&format!("Efficiency: {:.0}%", room.efficiency * 100.0), details_x, start_y + 65.0, 16.0, eff_color);
    draw_text("Walls/doors needed for 100%", details_x, start_y + 85.0, 12.0, GRAY);
}

fn draw_traps_content(sidebar: &Sidebar, current_mode: &InteractionMode, player: &PlayerState, game_data: &crate::data::GameData) {
    let door_cost = game_data.traps.get("door").map(|t| t.cost).unwrap_or(50);
    let spike_cost = game_data.traps.get("spike_trap").map(|t| t.cost).unwrap_or(100);
    let layout = vec![
        ("Door", InteractionMode::BuildTrap("door".to_string()), door_cost, "D"),
        ("Spike Trap", InteractionMode::BuildTrap("spike_trap".to_string()), spike_cost, "S"),
    ];

    let start_x = PADDING;
    let start_y = sidebar.panel_y + PADDING;
    let mut current_x = start_x;
    let mut current_y = start_y;

    for (label, mode, cost, hotkey) in layout {
        let width = BUTTON_SIZE * 2.5;
        if current_x + width > screen_width() - RIGHT_MARGIN {
            current_x = start_x;
            current_y += BUTTON_SIZE + BUTTON_SPACING;
        }

        let is_selected = interaction_modes_match(current_mode, &mode);
        let can_afford = player.materials >= cost;

        let color = if is_selected {
            Color::new(0.2, 0.6, 0.3, 1.0)
        } else if !can_afford {
            Color::new(0.3, 0.1, 0.1, 1.0)
        } else {
            Color::new(0.25, 0.25, 0.3, 1.0)
        };

        draw_rectangle(current_x, current_y, width, BUTTON_SIZE, color);
        draw_rectangle_lines(current_x, current_y, width, BUTTON_SIZE, 2.0, Color::new(0.4, 0.4, 0.5, 1.0));

        draw_text(label, current_x + 5.0, current_y + 18.0, 16.0, WHITE);
        if cost > 0 {
            // Use M for materials
            draw_text(&format!("{} Mats", cost), current_x + 5.0, current_y + 40.0, 14.0, WHITE);
        }
        draw_text(hotkey, current_x + width - 15.0, current_y + 15.0, 12.0, GRAY);

        current_x += width + BUTTON_SPACING;
    }
    
    // Show material count
    draw_text(
        &format!("Materials: {} / {}", player.materials, player.max_materials), 
        screen_width() - RIGHT_MARGIN - 250.0, 
        sidebar.panel_y + 30.0, 
        20.0, 
        WHITE
    );
}

fn draw_research_content(sidebar: &Sidebar, player: &PlayerState, technologies: &HashMap<String, crate::data::TechData>) {
    let start_x = PADDING;
    let start_y = sidebar.panel_y + PADDING;

    let mut sorted_techs: Vec<&crate::data::TechData> = technologies.values().collect();
    // Sort by cost
    sorted_techs.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());

    let mut current_x = start_x;
    let mut current_y = start_y;

    for tech in sorted_techs {
        let width = BUTTON_SIZE * 3.5;
        
        if current_x + width > screen_width() - RIGHT_MARGIN {
            current_x = start_x;
            current_y += BUTTON_SIZE * 1.2 + BUTTON_SPACING;
        }

        let is_completed = player.is_tech_completed(&tech.id);
        let is_active = player.active_research.as_ref().map(|id| id == &tech.id).unwrap_or(false);
        
        // Check prerequisites
        let prereqs_met = tech.prerequisites.iter().all(|req| player.is_tech_completed(req));

        // Determine color/state
        let (bg_color, text_color) = if is_completed {
            (Color::new(0.2, 0.4, 0.2, 1.0), GRAY) // Dark Green
        } else if is_active {
            (Color::new(0.2, 0.6, 0.8, 1.0), WHITE) // Blue
        } else if prereqs_met {
            (Color::new(0.3, 0.3, 0.35, 1.0), WHITE) // Available
        } else {
            (Color::new(0.15, 0.15, 0.15, 0.5), DARKGRAY) // Locked
        };

        // Draw Button
        draw_rectangle(current_x, current_y, width, BUTTON_SIZE * 1.2, bg_color);
        draw_rectangle_lines(current_x, current_y, width, BUTTON_SIZE * 1.2, 2.0, Color::new(0.4, 0.4, 0.5, 1.0));

        // Tech Name
        draw_text(&tech.name, current_x + 5.0, current_y + 15.0, 16.0, text_color);
        
        // Cost / Progress
            if is_completed {
            draw_text("Completed", current_x + 5.0, current_y + 35.0, 14.0, text_color);
        } else if is_active {
            let progress = player.research_progress;
            let pct = (progress / tech.cost).clamp(0.0, 1.0);
            
            // Draw progress bar
            let bar_w = width - 10.0;
            draw_rectangle(current_x + 5.0, current_y + 35.0, bar_w, 10.0, BLACK);
            draw_rectangle(current_x + 5.0, current_y + 35.0, bar_w * pct, 10.0, GREEN);
            
            draw_text(&format!("{:.0}/{:.0}", progress, tech.cost), current_x + 5.0, current_y + 30.0, 12.0, WHITE);
        } else if prereqs_met {
            draw_text(&format!("Cost: {:.0}", tech.cost), current_x + 5.0, current_y + 35.0, 14.0, GOLD);
        } else {
            draw_text("Locked", current_x + 5.0, current_y + 35.0, 14.0, RED);
        }

        // Move to next position
        current_x += width + BUTTON_SPACING;
    }
    
    // Show Research Points generation rate (implied from libraries)
    // Hard to calculate here without iterating all rooms/creatures, but let's show status text
    if let Some(active) = &player.active_research {
        if let Some(tech) = technologies.get(active) {
            draw_text(
                &format!("Researching: {} ({:.1}%)", tech.name, (player.research_progress / tech.cost) * 100.0),
                screen_width() - RIGHT_MARGIN - 300.0,
                sidebar.panel_y + 30.0,
                18.0,
                LIGHTGRAY
            );
        }
    } else {
            draw_text(
            "No Active Research",
            screen_width() - RIGHT_MARGIN - 300.0,
            sidebar.panel_y + 30.0,
            18.0,
            GRAY
        );
    }
}

// Original function replaced by updated version above with SaveGame support
