use crate::state::game_state::GameState;

pub enum GamePhase {
    Loading,
    MainMenu,
    Playing(GameState),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InteractionMode {
    None,
    Dig,
    BuildRoom(String), // room_type_id
    BuildTrap(String), // trap_type_id
    PlaceSpawner,
    Pickup,
    Drop,
    Sell, // Sell room or cancel task
    Inspect, // Inspect minion details
    SetAttackMarker,
    SetDefendMarker,
    SaveGame,
}
