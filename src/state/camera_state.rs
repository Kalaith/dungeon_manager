use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraState {
    pub target: (f32, f32, f32),
    pub distance: f32,
    pub target_distance: f32,
    pub angle: f32,
    pub zoom: f32,
    pub zoom_index: usize,
}

impl CameraState {
    pub const ZOOM_LEVELS: &'static [f32] = &[5.0, 8.0, 12.0, 16.0, 20.0, 25.0, 32.0, 40.0, 50.0];

    pub fn new(width: f32, height: f32) -> Self {
        // Find index of default distance (20.0)
        let default_dist = 20.0;
        let index = Self::ZOOM_LEVELS.iter().position(|&d| d == default_dist).unwrap_or(4);

        Self {
            target: (width / 2.0, 0.0, height / 2.0),
            distance: default_dist,
            target_distance: default_dist,
            angle: 0.0,
            zoom: 1.0,
            zoom_index: index,
        }
    }

    pub fn zoom_in(&mut self) {
        if self.zoom_index > 0 {
            self.zoom_index -= 1;
            self.target_distance = Self::ZOOM_LEVELS[self.zoom_index];
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom_index < Self::ZOOM_LEVELS.len() - 1 {
            self.zoom_index += 1;
            self.target_distance = Self::ZOOM_LEVELS[self.zoom_index];
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Smooth zoom interpolation
        let lerp_factor = 5.0 * dt; // Adjust speed as needed
        self.distance += (self.target_distance - self.distance) * lerp_factor.clamp(0.0, 1.0);
    }

    pub fn get_camera3d(&self) -> Camera3D {
        Camera3D {
            position: vec3(
                self.target.0 + (self.distance * 0.5) * (self.angle + std::f32::consts::FRAC_PI_2).cos(),
                self.distance,
                self.target.2 + (self.distance * 0.5) * (self.angle + std::f32::consts::FRAC_PI_2).sin(),
            ),
            target: vec3(self.target.0, 0.0, self.target.2),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0f32.to_radians(), // Fix: Ensure FOV is in radians for consistency with raycasting
            projection: Projection::Perspective,
            aspect: Some(screen_width() / screen_height()), // Fix: Match partial aspect ratio
            ..Default::default()
        }
    }
}
