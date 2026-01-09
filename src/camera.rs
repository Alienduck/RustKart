pub struct Camera {
    pub x: f32,
    pub z: f32,
    pub direction: f32,
    pub height: f32, // Hauteur de caméra
    pub speed: f32,  // vitesse actuelle du joueur, utilisée pour ajuster l'effet de profondeur
}

impl Camera {
    pub fn new(x: f32, z: f32) -> Self {
        Self {
            x,
            z,
            direction: 0.0,
            height: 60.0,
            speed: 0.0,
        }
    }

    pub fn follow_player(
        &mut self,
        player_x: f32,
        player_z: f32,
        player_direction: f32,
        player_speed: f32,
    ) {
        // Adapter la distance et la hauteur de la caméra selon la vitesse pour donner une sensation de vitesse
        let base_distance = (35.0 + player_speed * 1.2).clamp(20.0, 120.0);
        let base_height = (50.0 - player_speed * 0.5).clamp(18.0, 80.0);

        let offset_x = -player_direction.sin() * base_distance;
        let offset_z = -player_direction.cos() * base_distance;

        // Lissage position et hauteur
        self.x = self.x * 0.85 + (player_x + offset_x) * 0.15;
        self.z = self.z * 0.85 + (player_z + offset_z) * 0.15;
        self.height = self.height * 0.9 + base_height * 0.1;
        self.direction = player_direction;
        self.speed = player_speed;
    }

    /// Projection perspective pour Mario Kart-like
    /// Vue à la première personne légèrement élevée (caméra derrière et au-dessus du kart)
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
        // utiliser hauteur relative positive = camera.height - world_y pour que les objets proches apparaissent BAS
        let rel_y = self.height - world_y; // Hauteur relative à la caméra

        // Rotation par rapport à la direction de la caméra
        let cos_d = self.direction.cos();
        let sin_d = self.direction.sin();

        let rotated_x = rel_x * cos_d - rel_z * sin_d;
        let rotated_z = rel_x * sin_d + rel_z * cos_d;

        // Culling : ne pas dessiner derrière la caméra
        if rotated_z < 0.5 {
            return None;
        }

        // Perspective simple : la distance et la vitesse déterminent l'échelle
        let speed_factor = 1.0 + (self.speed * 0.03);
        // Empêcher les valeurs extrêmes pour les objets très proches en plafonnant le denominateur
        let denom = rotated_z.max(5.0);
        let depth_factor = (400.0 * speed_factor) / denom;

        // Position X : centré à l'écran, échelle par profondeur
        let screen_x = (screen_w as f32 / 2.0) + (rotated_x * depth_factor);

        // Position Y :
        // - Les objets proches (petit rotated_z => depth_factor grand) apparaissent BAS (y élevé)
        // Horizon ajusté plus bas pour une meilleure perspective
        let horizon_y = screen_h as f32 * 0.6;
        let vertical_scale = 0.8;
        let screen_y = horizon_y + (rel_y * depth_factor * vertical_scale);

        // Clamp des coordonnées pour éviter de rejeter les sprites partiellement hors-écran
        let screen_x = screen_x.clamp(0.0, screen_w as f32 - 1.0);
        let screen_y = screen_y.clamp(0.0, screen_h as f32 - 1.0);

        Some((screen_x, screen_y))
    }

    pub fn distance_to(&self, x: f32, z: f32) -> f32 {
        ((x - self.x).powi(2) + (z - self.z).powi(2)).sqrt()
    }
}
