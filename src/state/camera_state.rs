use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct CameraState {
    pub target: (f32, f32, f32),
    pub distance: f32,
    pub angle: f32,
    pub zoom: f32,
}

impl CameraState {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            target: (width / 2.0, 0.0, height / 2.0),
            distance: 20.0,
            angle: 0.0,
            zoom: 1.0,
        }
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
            fovy: 45.0,
            projection: Projection::Perspective,
            ..Default::default()
        }
    }
}
