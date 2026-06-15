use crate::data::campaign::CampaignUnlocks;
use crate::data::GameData;
use crate::state::game_state::GameState;

impl GameState {
    pub(in crate::state) fn apply_scenario_setup(
        &mut self,
        game_data: &GameData,
        scenario_id: &str,
    ) {
        let Some(scenario) = game_data.scenarios.get(scenario_id) else {
            return;
        };

        if let Some(player) = scenario
            .players
            .iter()
            .find(|player| player.owner.is_player_controlled())
        {
            if player.start_gold > 0 {
                self.player.gold = player.start_gold.min(self.player.max_gold);
            }
            if player.start_mana > 0 {
                self.player.mana = player.start_mana.min(self.player.max_mana);
            }
            if let Some(max_creatures) = player.max_creatures {
                self.player.max_creatures = max_creatures as usize;
            }
        }

        if !scenario.availability.rooms.is_empty() {
            self.player.unlocked_rooms.clear();
            self.player.unlock_room("dungeon_heart".to_string());
            for room in scenario
                .availability
                .rooms
                .iter()
                .filter_map(|(id, rule)| rule.unlocked.then_some(id))
            {
                self.player.unlock_room(room.clone());
            }
        }

        if !scenario.availability.creatures.is_empty() {
            self.player.unlocked_creatures.clear();
            self.player.unlock_creature("imp".to_string());
            for creature in scenario
                .availability
                .creatures
                .iter()
                .filter_map(|(id, rule)| rule.unlocked.then_some(id))
            {
                self.player.unlock_creature(creature.clone());
            }
        }

        if !scenario.availability.spells.is_empty() {
            self.player.unlocked_spells.clear();
            for spell in scenario
                .availability
                .spells
                .iter()
                .filter_map(|(id, rule)| rule.unlocked.then_some(id))
            {
                self.player.unlock_spell(spell.clone());
            }
        }

        if !scenario.availability.traps.is_empty() {
            self.player.unlocked_traps.clear();
            for trap in scenario
                .availability
                .traps
                .iter()
                .filter_map(|(id, rule)| rule.unlocked.then_some(id))
            {
                self.player.unlock_trap(trap.clone());
            }
        }

        for tech_id in &scenario.rules.starting_research {
            if let Some(tech) = game_data.technologies.get(tech_id) {
                self.player.complete_research(tech);
            }
        }
    }

    pub(in crate::state) fn apply_campaign_unlocks(&mut self, unlocks: &CampaignUnlocks) {
        for room in &unlocks.rooms {
            self.player.unlock_room(room.clone());
        }
        for spell in &unlocks.spells {
            self.player.unlock_spell(spell.clone());
        }
        for trap in &unlocks.traps {
            self.player.unlock_trap(trap.clone());
        }
        for creature in &unlocks.creatures {
            self.player.unlock_creature(creature.clone());
        }
    }
}
