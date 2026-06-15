use crate::state::entities::EntityId;
use crate::ui::sidebar::{efficiency_color, Sidebar, BUTTON_SIZE, BUTTON_SPACING, PADDING};
use crate::InteractionMode;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub(super) fn draw_minions_content(
    sidebar: &Sidebar,
    current_mode: &InteractionMode,
    held_entity: Option<EntityId>,
    selected_entity: Option<EntityId>,
    selected_room: Option<usize>,
    entities: &crate::state::entities::EntityManager,
    rooms: &[crate::engine::room_validator::Room],
) {
    let start_x = PADDING;
    let start_y = sidebar.panel_y + PADDING;

    let pickup_drop_color = if interaction_modes_match(current_mode, &InteractionMode::Pickup)
        || interaction_modes_match(current_mode, &InteractionMode::Drop)
    {
        match current_mode {
            InteractionMode::Pickup => Color::new(0.2, 0.6, 0.3, 1.0),
            InteractionMode::Drop => Color::new(0.6, 0.4, 0.2, 1.0),
            _ => Color::new(0.25, 0.25, 0.3, 1.0),
        }
    } else {
        Color::new(0.25, 0.25, 0.3, 1.0)
    };

    let pickup_drop_label = if held_entity.is_some() {
        "Drop Minion"
    } else {
        "Pickup Minion"
    };

    draw_rectangle(
        start_x,
        start_y,
        BUTTON_SIZE * 2.5,
        BUTTON_SIZE,
        pickup_drop_color,
    );
    draw_rectangle_lines(start_x, start_y, BUTTON_SIZE * 2.5, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text(
        pickup_drop_label,
        start_x + 10.0,
        start_y + 30.0,
        16.0,
        WHITE,
    );

    let inspect_x = start_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING;
    let inspect_color = if interaction_modes_match(current_mode, &InteractionMode::Inspect) {
        Color::new(0.2, 0.6, 0.8, 1.0)
    } else {
        Color::new(0.25, 0.25, 0.3, 1.0)
    };

    draw_rectangle(
        inspect_x,
        start_y,
        BUTTON_SIZE * 2.5,
        BUTTON_SIZE,
        inspect_color,
    );
    draw_rectangle_lines(
        inspect_x,
        start_y,
        BUTTON_SIZE * 2.5,
        BUTTON_SIZE,
        2.0,
        WHITE,
    );
    draw_ui_text("Inspect", inspect_x + 10.0, start_y + 30.0, 16.0, WHITE);

    let marker_y = start_y + BUTTON_SIZE + BUTTON_SPACING;

    let attack_color = if interaction_modes_match(current_mode, &InteractionMode::SetAttackMarker) {
        Color::new(0.7, 0.2, 0.2, 1.0)
    } else {
        Color::new(0.4, 0.2, 0.2, 1.0)
    };
    draw_rectangle(
        start_x,
        marker_y,
        BUTTON_SIZE * 2.5,
        BUTTON_SIZE,
        attack_color,
    );
    draw_rectangle_lines(
        start_x,
        marker_y,
        BUTTON_SIZE * 2.5,
        BUTTON_SIZE,
        2.0,
        WHITE,
    );
    draw_ui_text("Set Attack", start_x + 10.0, marker_y + 30.0, 16.0, WHITE);

    let defend_x = start_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING;
    let defend_color = if interaction_modes_match(current_mode, &InteractionMode::SetDefendMarker) {
        Color::new(0.2, 0.2, 0.7, 1.0)
    } else {
        Color::new(0.2, 0.2, 0.4, 1.0)
    };
    draw_rectangle(
        defend_x,
        marker_y,
        BUTTON_SIZE * 2.5,
        BUTTON_SIZE,
        defend_color,
    );
    draw_rectangle_lines(
        defend_x,
        marker_y,
        BUTTON_SIZE * 2.5,
        BUTTON_SIZE,
        2.0,
        WHITE,
    );
    draw_ui_text("Set Defend", defend_x + 10.0, marker_y + 30.0, 16.0, WHITE);

    draw_ui_text(
        "Selection Controls",
        start_x,
        start_y + (BUTTON_SIZE * 2.0) + 40.0,
        18.0,
        LIGHTGRAY,
    );

    let details_x = inspect_x + BUTTON_SIZE * 2.5 + BUTTON_SPACING * 2.0;
    draw_selection_details(
        details_x,
        start_y,
        selected_entity,
        selected_room,
        entities,
        rooms,
    );
}

fn draw_selection_details(
    details_x: f32,
    start_y: f32,
    selected_entity: Option<EntityId>,
    selected_room: Option<usize>,
    entities: &crate::state::entities::EntityManager,
    rooms: &[crate::engine::room_validator::Room],
) {
    if let Some(id) = selected_entity {
        draw_entity_details(details_x, start_y, id, entities);
    } else if let Some(room_id) = selected_room {
        draw_room_details(details_x, start_y, room_id, rooms);
    } else {
        draw_ui_text(
            "Select a unit or room to view details",
            details_x,
            start_y + 30.0,
            18.0,
            GRAY,
        );
    }
}

fn draw_entity_details(
    details_x: f32,
    start_y: f32,
    id: EntityId,
    entities: &crate::state::entities::EntityManager,
) {
    let entity = match entities.get(id) {
        Some(entity) => entity,
        None => return,
    };

    if let Some(creature) = entity.as_creature() {
        draw_ui_text(
            &format!(
                "Selected: {} (Lvl {})",
                creature.creature_id, creature.level
            ),
            details_x,
            start_y + 20.0,
            20.0,
            WHITE,
        );
        draw_ui_text(
            &format!(
                "HP: {:.0}/{:.0} | Mood: {:.0}%",
                creature.health, creature.max_health, creature.mood
            ),
            details_x,
            start_y + 45.0,
            16.0,
            WHITE,
        );
        draw_ui_text(
            &format!(
                "Rest: {:.0}% | Food: {:.0}%",
                creature.get_need("sleep"),
                creature.get_need("food")
            ),
            details_x,
            start_y + 65.0,
            16.0,
            WHITE,
        );
        draw_ui_text(
            &format!("Job: {:?}", creature.current_task),
            details_x,
            start_y + 85.0,
            16.0,
            LIGHTGRAY,
        );
        return;
    }

    if let Some(hero) = entity.as_hero() {
        draw_ui_text(
            &format!("Hero: {} (Lvl {})", hero.hero_id, hero.level),
            details_x,
            start_y + 20.0,
            20.0,
            WHITE,
        );

        let hp_pct = hero.health / hero.max_health;
        let bar_width = 200.0;
        draw_rectangle(details_x, start_y + 30.0, bar_width, 10.0, RED);
        draw_rectangle(details_x, start_y + 30.0, bar_width * hp_pct, 10.0, GREEN);
        draw_ui_text(
            &format!("{:.0}/{:.0} HP", hero.health, hero.max_health),
            details_x + 5.0,
            start_y + 39.0,
            10.0,
            WHITE,
        );

        let role = if hero.is_defender {
            "Defender"
        } else {
            "Attacker"
        };
        let wave_info = if hero.wave_assigned > 0 {
            format!(" (Wave {})", hero.wave_assigned)
        } else {
            String::new()
        };
        draw_ui_text(
            &format!("Role: {}{}", role, wave_info),
            details_x,
            start_y + 55.0,
            16.0,
            WHITE,
        );

        let status = if hero.is_digging {
            "Digging"
        } else if hero.current_path.is_some() {
            "Moving"
        } else {
            "Idle"
        };
        draw_ui_text(
            &format!("Status: {} | Goal: {:?}", status, hero.current_goal),
            details_x,
            start_y + 75.0,
            14.0,
            LIGHTGRAY,
        );
        draw_ui_text(
            &format!("Kills: {} | Gold: {}", hero.kills, hero.gold_stolen),
            details_x,
            start_y + 95.0,
            14.0,
            GOLD,
        );

        if hero.is_fleeing {
            draw_ui_text("FLEEING!", details_x + 150.0, start_y + 55.0, 16.0, RED);
        }
    }
}

fn draw_room_details(
    details_x: f32,
    start_y: f32,
    room_id: usize,
    rooms: &[crate::engine::room_validator::Room],
) {
    let room = match rooms.iter().find(|room| room.id == room_id) {
        Some(room) => room,
        None => return,
    };

    draw_ui_text(
        &format!("Room: {} (ID: {})", room.room_type, room.id),
        details_x,
        start_y + 20.0,
        20.0,
        WHITE,
    );
    draw_ui_text(
        &format!("Size: {} tiles", room.tiles.len()),
        details_x,
        start_y + 45.0,
        16.0,
        WHITE,
    );

    let efficiency = efficiency_color(room.efficiency);
    draw_ui_text(
        &format!("Efficiency: {:.0}%", room.efficiency * 100.0),
        details_x,
        start_y + 65.0,
        16.0,
        efficiency,
    );
    draw_ui_text(
        "Walls/doors needed for 100%",
        details_x,
        start_y + 85.0,
        12.0,
        GRAY,
    );
}

fn interaction_modes_match(m1: &InteractionMode, m2: &InteractionMode) -> bool {
    m1 == m2
}
