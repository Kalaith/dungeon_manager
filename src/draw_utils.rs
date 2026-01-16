use macroquad::prelude::*;

pub fn draw_billboard(pos: Vec3, size: Vec2, texture: &Texture2D, camera_pos: Vec3) {
    let to_cam = camera_pos - pos;
    // Y-axis billboarding (cylindrical)
    let dist_sq = to_cam.x * to_cam.x + to_cam.z * to_cam.z;
    
    // Robustness: If strictly above (dist_sq ~ 0), default to looking South (Z+) or arbitrary valid vector
    let fwd = if dist_sq < 0.001 {
        vec3(0.0, 0.0, 1.0)
    } else {
        vec3(to_cam.x, 0.0, to_cam.z).normalize()
    };
    
    let right = vec3(0.0, 1.0, 0.0).cross(fwd).normalize();
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
        // Double sided: draw CW and CCW
        indices: vec![
            0, 1, 2, 0, 2, 3, // Front
            0, 2, 1, 0, 3, 2  // Back
        ],
        texture: Some(texture.clone()),
    };
    
    draw_mesh(&mesh);
}
