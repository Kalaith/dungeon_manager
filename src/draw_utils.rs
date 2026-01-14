use macroquad::prelude::*;

pub fn draw_billboard(pos: Vec3, size: Vec2, texture: &Texture2D, camera_pos: Vec3) {
    let to_cam = camera_pos - pos;
    // Y-axis billboarding (cylindrical)
    let fwd = vec3(to_cam.x, 0.0, to_cam.z).normalize_or_zero();
    let right = vec3(0.0, 1.0, 0.0).cross(fwd).normalize_or_zero();
    let up = vec3(0.0, 1.0, 0.0);

    let half_w = size.x * 0.5;
    let half_h = size.y * 0.5;
    
    // Top Left
    let v1 = pos - right * half_w + up * half_h; 
    // Top Right
    let v2 = pos + right * half_w + up * half_h;
    // Bottom Right
    let v3 = pos + right * half_w - up * half_h; 
    // Bottom Left
    let v4 = pos - right * half_w - up * half_h; 

    let mesh = Mesh {
        vertices: vec![
            Vertex { position: v1, uv: vec2(0.0, 0.0), color: WHITE.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v2, uv: vec2(1.0, 0.0), color: WHITE.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v3, uv: vec2(1.0, 1.0), color: WHITE.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
            Vertex { position: v4, uv: vec2(0.0, 1.0), color: WHITE.into(), normal: vec4(fwd.x, fwd.y, fwd.z, 0.0) },
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: Some(texture.clone()),
    };
    
    draw_mesh(&mesh);
}
