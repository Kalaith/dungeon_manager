use crate::ui::sidebar::{Sidebar, BUTTON_SIZE, BUTTON_SPACING, PADDING};
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub(super) fn draw_cheats_content(sidebar: &Sidebar) {
    let start_x = PADDING;
    let start_y = sidebar.panel_y + PADDING;

    let row1_y = start_y;

    let cat_text = format!("{:?}", sidebar.cheat_state.category);
    let cat_btn_width = 120.0;
    draw_rectangle(
        start_x,
        row1_y,
        cat_btn_width,
        BUTTON_SIZE,
        Color::new(0.2, 0.4, 0.6, 1.0),
    );
    draw_rectangle_lines(start_x, row1_y, cat_btn_width, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text(&cat_text, start_x + 10.0, row1_y + 30.0, 16.0, WHITE);

    let lvl_minus_x = start_x + cat_btn_width + BUTTON_SPACING;
    draw_rectangle(
        lvl_minus_x,
        row1_y,
        BUTTON_SIZE,
        BUTTON_SIZE,
        Color::new(0.4, 0.2, 0.2, 1.0),
    );
    draw_rectangle_lines(lvl_minus_x, row1_y, BUTTON_SIZE, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text("-", lvl_minus_x + 15.0, row1_y + 30.0, 20.0, WHITE);

    let lvl_text = format!("Lvl {}", sidebar.cheat_state.level);
    let lvl_text_x = lvl_minus_x + BUTTON_SIZE + BUTTON_SPACING;
    draw_ui_text(&lvl_text, lvl_text_x, row1_y + 30.0, 16.0, WHITE);

    let lvl_plus_x = lvl_text_x + 50.0 + BUTTON_SPACING;
    draw_rectangle(
        lvl_plus_x,
        row1_y,
        BUTTON_SIZE,
        BUTTON_SIZE,
        Color::new(0.2, 0.4, 0.2, 1.0),
    );
    draw_rectangle_lines(lvl_plus_x, row1_y, BUTTON_SIZE, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text("+", lvl_plus_x + 15.0, row1_y + 30.0, 20.0, WHITE);

    let id_x = lvl_plus_x + BUTTON_SIZE + BUTTON_SPACING;
    let id_text = &sidebar.cheat_state.entity_id;
    let id_btn_width = 200.0;
    draw_rectangle(
        id_x,
        row1_y,
        id_btn_width,
        BUTTON_SIZE,
        Color::new(0.3, 0.3, 0.35, 1.0),
    );
    draw_rectangle_lines(id_x, row1_y, id_btn_width, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text(id_text, id_x + 10.0, row1_y + 30.0, 16.0, WHITE);

    let spawn_x = id_x + id_btn_width + BUTTON_SPACING;
    let spawn_btn_width = 140.0;
    draw_rectangle(
        spawn_x,
        row1_y,
        spawn_btn_width,
        BUTTON_SIZE,
        Color::new(0.6, 0.2, 0.6, 1.0),
    );
    draw_rectangle_lines(spawn_x, row1_y, spawn_btn_width, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text("Prepare Spawn", spawn_x + 10.0, row1_y + 30.0, 16.0, WHITE);

    let row2_y = row1_y + BUTTON_SIZE + BUTTON_SPACING;

    let gold_btn_width = 120.0;
    draw_rectangle(start_x, row2_y, gold_btn_width, BUTTON_SIZE, GOLD);
    draw_rectangle_lines(start_x, row2_y, gold_btn_width, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text("Gold +100", start_x + 10.0, row2_y + 30.0, 16.0, BLACK);

    let fow_x = start_x + gold_btn_width + BUTTON_SPACING;
    let fow_width = 150.0;
    draw_rectangle(
        fow_x,
        row2_y,
        fow_width,
        BUTTON_SIZE,
        Color::new(0.3, 0.3, 0.4, 1.0),
    );
    draw_rectangle_lines(fow_x, row2_y, fow_width, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text("Toggle Fog", fow_x + 10.0, row2_y + 30.0, 16.0, WHITE);

    let dig_x = fow_x + fow_width + BUTTON_SPACING;
    let dig_width = 130.0;
    let dig_active = sidebar.cheat_state.instant_dig_active;
    let dig_color = if dig_active { GREEN } else { RED };
    draw_rectangle(dig_x, row2_y, dig_width, BUTTON_SIZE, dig_color);
    draw_rectangle_lines(dig_x, row2_y, dig_width, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text("Instant Dig", dig_x + 10.0, row2_y + 30.0, 16.0, WHITE);

    let heart_x = dig_x + dig_width + BUTTON_SPACING;
    let heart_width = 140.0;
    let heart_active = sidebar.cheat_state.immortal_heart_active;
    let heart_color = if heart_active { GREEN } else { RED };
    draw_rectangle(heart_x, row2_y, heart_width, BUTTON_SIZE, heart_color);
    draw_rectangle_lines(heart_x, row2_y, heart_width, BUTTON_SIZE, 2.0, WHITE);
    draw_ui_text("God Mode", heart_x + 10.0, row2_y + 30.0, 16.0, WHITE);
}
