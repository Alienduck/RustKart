use chinchilib::rgb::RGBA8;

pub struct Track {
    pub width: usize,
    pub height: usize,
    pub track_width: f32,
}

impl Track {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            track_width: 280.0,
        }
    }

    /// Met à jour la taille utilisée pour le rendu (à appeler quand la fenêtre change de taille)
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn draw(&self, frame: &mut [u8], camera: &crate::camera::Camera) {
        // Dessiner le background dégradé simple
        self.draw_simple_background(frame);

        // Dessiner la piste en perspective
        self.draw_track_from_camera(frame, camera);
    }

    fn draw_simple_background(&self, frame: &mut [u8]) {
        // Aligner l'horizon du background avec le calcul dans la caméra
        let mid_y = (self.height as f32 * 0.6) as usize;

        // Ciel (haut)
        for y in 0..mid_y {
            for x in 0..self.width {
                let idx = (y * self.width + x) * 4;
                if idx + 3 < frame.len() {
                    // Ciel bleu clair
                    frame[idx] = 100;
                    frame[idx + 1] = 180;
                    frame[idx + 2] = 255;
                    frame[idx + 3] = 255;
                }
            }
        }

        // Sol/Herbe (bas)
        for y in mid_y..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) * 4;
                if idx + 3 < frame.len() {
                    // Herbe vert clair
                    frame[idx] = 50;
                    frame[idx + 1] = 180;
                    frame[idx + 2] = 50;
                    frame[idx + 3] = 255;
                }
            }
        }
    }

    fn draw_track_from_camera(&self, frame: &mut [u8], camera: &crate::camera::Camera) {
        const ROAD_COLOR: RGBA8 = RGBA8 {
            r: 80,
            g: 80,
            b: 90,
            a: 255,
        };
        const ROAD_STRIPE: RGBA8 = RGBA8 {
            r: 220,
            g: 220,
            b: 200,
            a: 255,
        };
        const ROAD_EDGE: RGBA8 = RGBA8 {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };

        let road_half_width = self.track_width / 2.0;

        // Dessiner la route segment par segment, du loin au proche
        // pour que les segments proches se dessinent par-dessus
        let mut segments: Vec<(usize, f32, f32, f32, f32)> = Vec::new();

        // Limiter la portée de rendu mais l'étendre pour avoir un horizon plus lointain
        let max_draw = 1500i32;
        for z_offset in (-200..max_draw).step_by(3) {
            let z = camera.z + z_offset as f32;

            // Calculer une courbure simple de la piste (centre oscillant selon z)
            let center_x = self.track_center(z);
            let left_x = center_x - road_half_width;
            let right_x = center_x + road_half_width;

            // Projeter sur l'écran
            if let (
                Some((left_sx, left_sy)),
                Some((right_sx, right_sy)),
                Some((center_sx, _center_sy)),
            ) = (
                camera.world_to_screen(left_x, z, 0.0, self.width, self.height),
                camera.world_to_screen(right_x, z, 0.0, self.width, self.height),
                camera.world_to_screen(center_x, z, 0.0, self.width, self.height),
            ) {
                let y = ((left_sy + right_sy) / 2.0) as i32;

                if y >= 0 && y < self.height as i32 {
                    let y_usize = y as usize;
                    let _left_x_i = (left_sx as i32).max(0) as usize;
                    let _right_x_i = (right_sx as i32).min(self.width as i32 - 1) as usize;

                    segments.push((y_usize, left_sx, right_sx, center_sx, z_offset as f32));
                }
            }
        }

        // Dessiner les segments en remplissant les scanlines manquantes
        // Trier par y (du plus loin vers le plus proche à l'écran)
        segments.sort_by_key(|s| s.0);

        if !segments.is_empty() {
            // Itérer par paires de segments et interpoler pour chaque ligne y entre eux
            for window in segments.windows(2) {
                let (y0, left0, right0, center0, z0) = window[0];
                let (y1, left1, right1, center1, z1) = window[1];

                // s'assurer des bornes
                let y_start = y0.min(self.height - 1);
                let y_end = y1.min(self.height - 1);

                if y_end < y_start {
                    continue;
                }

                for y_scan in y_start..=y_end {
                    let t = if y_end == y_start {
                        0.0
                    } else {
                        (y_scan - y_start) as f32 / (y_end - y_start) as f32
                    };

                    let left_sx = left0 + (left1 - left0) * t;
                    let right_sx = right0 + (right1 - right0) * t;
                    let center_sx = center0 + (center1 - center0) * t;
                    let z_interp = z0 + (z1 - z0) * t;

                    let left_x_i = (left_sx as i32).max(0) as usize;
                    let right_x_i = (right_sx as i32).min(self.width as i32 - 1) as usize;

                    if left_x_i <= right_x_i && y_scan < self.height {
                        // Remplir la route pour cette scanline
                        for screen_x in left_x_i..=right_x_i {
                            let idx = (y_scan * self.width + screen_x) * 4;
                            if idx + 3 < frame.len() {
                                let distance_from_center = (screen_x as f32 - center_sx).abs();
                                let road_width = (right_sx - left_sx).abs().max(1.0);
                                let rel_pos = distance_from_center / (road_width / 2.0 + 1.0);

                                // Bandes blanches selon la profondeur interpolée
                                let stripe_index = (((z_interp / 4.0) as usize) / 12) % 2;
                                let is_stripe = rel_pos < 0.7 && stripe_index == 0;

                                let color = if is_stripe { ROAD_STRIPE } else { ROAD_COLOR };

                                frame[idx] = color.r;
                                frame[idx + 1] = color.g;
                                frame[idx + 2] = color.b;
                                frame[idx + 3] = color.a;
                            }
                        }

                        // Bordures blanches (placer à l'extérieur de la route)
                        if (z_interp as i32) % 20 == 0 {
                            if left_x_i < self.width {
                                let idx = (y_scan * self.width + left_x_i) * 4;
                                if idx + 3 < frame.len() {
                                    frame[idx] = ROAD_EDGE.r;
                                    frame[idx + 1] = ROAD_EDGE.g;
                                    frame[idx + 2] = ROAD_EDGE.b;
                                    frame[idx + 3] = ROAD_EDGE.a;
                                }
                            }
                            if right_x_i < self.width {
                                let idx = (y_scan * self.width + right_x_i) * 4;
                                if idx + 3 < frame.len() {
                                    frame[idx] = ROAD_EDGE.r;
                                    frame[idx + 1] = ROAD_EDGE.g;
                                    frame[idx + 2] = ROAD_EDGE.b;
                                    frame[idx + 3] = ROAD_EDGE.a;
                                }
                            }
                        }
                    }
                }
            }
            // Dessiner le dernier segment seul (le plus proche) pour garantir la ligne la plus basse
            if let Some(last) = segments.last() {
                let (y, left_sx, right_sx, _center_sx, _z_offset) = *last;
                let left_x_i = (left_sx as i32).max(0) as usize;
                let right_x_i = (right_sx as i32).min(self.width as i32 - 1) as usize;
                if left_x_i <= right_x_i && y < self.height {
                    for screen_x in left_x_i..=right_x_i {
                        let idx = (y * self.width + screen_x) * 4;
                        if idx + 3 < frame.len() {
                            frame[idx] = ROAD_COLOR.r;
                            frame[idx + 1] = ROAD_COLOR.g;
                            frame[idx + 2] = ROAD_COLOR.b;
                            frame[idx + 3] = ROAD_COLOR.a;
                        }
                    }
                }
            }
        }

    }

    /// Retourne la position X du centre de la piste pour un z donné (simple sinus pour créer des virages)
    fn track_center(&self, z: f32) -> f32 {
        // fréquence et amplitude - on peut ajuster ces constantes pour changer la forme des virages
        let freq = 0.004; // contrôle la longueur des virages
        let amp = 140.0; // amplitude des virages (en unités monde)
        (z * freq).sin() * amp
    }
}
