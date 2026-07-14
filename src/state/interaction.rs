use crate::state::game_state::GameState;

pub enum GamePhase {
    Loading,
    MainMenu,
    Settings,
    /// Campaign map / mission-select: the player picks which unlocked mission
    /// to play (needed so the M7 branch is actually choosable). Holds the
    /// campaign progress until a mission is chosen.
    MissionSelect(crate::data::campaign::CampaignProgress),
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
    Sell,    // Sell room or cancel task
    Inspect, // Inspect minion details
    SetAttackMarker,
    SetDefendMarker,
    SaveGame,
    Summon(String, crate::state::entities::EntityCategory, u32), // id, category, level
}
