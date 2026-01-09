use std::f32::consts::PI;

pub struct Camera {
    pub x: f32,
    pub z: f32,
    pub direction: f32,
    pub height: f32, // Hauteur de caméra = 60 pixels du sol
}

impl Camera {
    pub fn new(x: f32, z: f32) -> Self {
        Self {
            x,
            z,
            direction: 0.0,
            height: 60.0,
        }
    }

    pub fn follow_player(&mut self, player_x: f32, player_z: f32, player_direction: f32) {
        // Distance de la caméra derrière le joueur
        let camera_distance = 35.0;
        let offset_x = -player_direction.sin() * camera_distance;
        let offset_z = -player_direction.cos() * camera_distance;

        // Lissage
        self.x = self.x * 0.85 + (player_x + offset_x) * 0.15;
        self.z = self.z * 0.85 + (player_z + offset_z) * 0.15;
        self.direction = player_direction;
    }

    /// Projection perspective pour Mario Kart-like
    /// Dessine de la profondeur proche à loin
    pub fn world_to_screen(
        &self,
        world_x: f32,
        world_z: f32,
        world_y: f32,
        screen_w: usize,
        screen_h: usize,
    ) -> Option<(f32, f32)> {
        // Coordonnées relatives à la caméra
        let rel_x = world_x - self.x;
        let rel_z = world_z - self.z;
        let rel_y = world_y - 0.0; // Y du sol est 0

        // Rotation par rapport à la direction de la caméra
        let cos_d = self.direction.cos();
        let sin_d = self.direction.sin();

        let rotated_x = rel_x * cos_d - rel_z * sin_d;
        let rotated_z = rel_x * sin_d + rel_z * cos_d;

        // Culling : ne pas dessiner derrière la caméra
        if rotated_z < 0.5 {
            return None;
        }

        // Perspective : proportionnel à la distance
        // Plus on est loin, plus petit et vers le centre
        let depth_factor = 300.0 / rotated_z;

        let screen_x = (screen_w as f32 / 2.0) + (rotated_x * depth_factor);
        
        // Y projection : plus haut = plus loin en arrière
        // La hauteur de la caméra est 60, le sol est à 0
        let camera_y = self.height;
        let screen_y = (screen_h as f32 * 0.6) - ((camera_y - rel_y) * depth_factor * 0.5);

        // Vérifier que le pixel est visible
        if screen_x < 0.0 || screen_x >= screen_w as f32 {
            return None;
        }
        if screen_y < 0.0 || screen_y >= screen_h as f32 {
            return None;
        }

        Some((screen_x, screen_y))
    }

    pub fn distance_to(&self, x: f32, z: f32) -> f32 {
        ((x - self.x).powi(2) + (z - self.z).powi(2)).sqrt()
    }
}
