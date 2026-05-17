//! Projectile system for visual attack effects
//!
//! Manages short-lived visual projectiles that travel from attacker to defender

use crate::state::entities::EntityId;
use crate::state::tile_state::TilePos;
use serde::{Deserialize, Serialize};

/// Type of projectile based on attack type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectileType {
    /// Melee slash - short range, stays near attacker
    Melee,
    /// Arrow - travels from attacker to defender
    Arrow,
    /// Magic orb - travels from attacker to defender with glow
    Magic,
}

impl ProjectileType {
    /// Get projectile type from attack_type string
    pub fn from_attack_type(attack_type: &str) -> Self {
        match attack_type {
            "ranged" => ProjectileType::Arrow,
            "magic" => ProjectileType::Magic,
            _ => ProjectileType::Melee,
        }
    }

    /// Get the texture key for this projectile type
    pub fn texture_key(&self) -> &'static str {
        match self {
            ProjectileType::Melee => "projectile_melee",
            ProjectileType::Arrow => "projectile_arrow",
            ProjectileType::Magic => "projectile_magic",
        }
    }

    /// Get the duration of this projectile in seconds
    pub fn duration(&self) -> f32 {
        match self {
            ProjectileType::Melee => 0.15, // Very quick slash
            ProjectileType::Arrow => 0.3,  // Fast arrow
            ProjectileType::Magic => 0.4,  // Slower magic orb
        }
    }
}

/// A projectile in flight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projectile {
    /// Starting position (attacker's position)
    pub start_pos: (f32, f32),
    /// Target position (defender's position)
    pub end_pos: (f32, f32),
    /// Progress from 0.0 (start) to 1.0 (end)
    pub progress: f32,
    /// Type of projectile
    pub projectile_type: ProjectileType,
    /// Attacker entity ID (for reference)
    pub attacker_id: EntityId,
    /// Defender entity ID (for reference)
    pub defender_id: EntityId,
    /// Total duration
    pub duration: f32,
    /// Time elapsed
    pub elapsed: f32,
    /// Damage to deal on impact
    pub damage: f32,
}

impl Projectile {
    /// Create a new projectile
    pub fn new(
        start_pos: (f32, f32),
        end_pos: (f32, f32),
        projectile_type: ProjectileType,
        attacker_id: EntityId,
        defender_id: EntityId,
        damage: f32,
    ) -> Self {
        let duration = projectile_type.duration();
        Self {
            start_pos,
            end_pos,
            progress: 0.0,
            projectile_type,
            attacker_id,
            defender_id,
            duration,
            elapsed: 0.0,
            damage,
        }
    }

    /// Get current world position of the projectile
    pub fn current_position(&self) -> (f32, f32) {
        // For melee, projectile stays close to attacker (only travels 30% of distance)
        let travel_ratio = match self.projectile_type {
            ProjectileType::Melee => 0.3,
            _ => 1.0,
        };

        let effective_progress = self.progress * travel_ratio;

        let x = self.start_pos.0 + (self.end_pos.0 - self.start_pos.0) * effective_progress;
        let y = self.start_pos.1 + (self.end_pos.1 - self.start_pos.1) * effective_progress;
        (x, y)
    }

    /// Update projectile, returns true if still active
    pub fn update(&mut self, dt: f32) -> bool {
        self.elapsed += dt;
        self.progress = (self.elapsed / self.duration).min(1.0);
        self.progress < 1.0
    }
}

/// Event generated when projectile hits target
pub struct Impact {
    pub attacker_id: EntityId,
    pub defender_id: EntityId,
    pub damage: f32,
}

/// Manager for all active projectiles
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectileManager {
    projectiles: Vec<Projectile>,
}

impl ProjectileManager {
    /// Create a new projectile manager
    pub fn new() -> Self {
        Self {
            projectiles: Vec::new(),
        }
    }

    /// Spawn a new projectile targeting an entity
    pub fn spawn(
        &mut self,
        start_pos: (f32, f32),
        end_pos: (f32, f32),
        attack_type: &str,
        attacker_id: EntityId,
        defender_id: EntityId,
        damage: f32,
    ) {
        let projectile_type = ProjectileType::from_attack_type(attack_type);
        let projectile = Projectile::new(
            start_pos,
            end_pos,
            projectile_type,
            attacker_id,
            defender_id,
            damage,
        );
        self.projectiles.push(projectile);
    }

    /// Spawn a projectile targeting a position (for structures like dungeon heart)
    pub fn spawn_at_position(
        &mut self,
        start_pos: (f32, f32),
        target_pos: TilePos,
        attack_type: &str,
        attacker_id: EntityId,
        damage: f32,
    ) {
        let projectile_type = ProjectileType::from_attack_type(attack_type);
        let end_pos = (target_pos.x as f32, target_pos.y as f32);
        // Use attacker_id as both since there's no defender entity
        let projectile = Projectile::new(
            start_pos,
            end_pos,
            projectile_type,
            attacker_id,
            attacker_id,
            damage,
        );
        self.projectiles.push(projectile);
    }

    /// Update all projectiles, removing completed ones and returning impacts
    pub fn update(&mut self, dt: f32) -> Vec<Impact> {
        let mut impacts = Vec::new();

        self.projectiles.retain_mut(|p| {
            let still_active = p.update(dt);
            if !still_active {
                impacts.push(Impact {
                    attacker_id: p.attacker_id,
                    defender_id: p.defender_id,
                    damage: p.damage,
                });
            }
            still_active
        });

        impacts
    }

    /// Get all active projectiles for rendering
    pub fn active_projectiles(&self) -> &[Projectile] {
        &self.projectiles
    }
}
