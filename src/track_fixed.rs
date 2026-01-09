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
            track_width: 250.0,
        }
    }

    pub fn draw(&self, frame: &mut [u8], camera: &crate::camera::Camera) {
        // Remplir l'écran avec ciel et herbe
        self.draw_background(frame);

        // Dessiner la route en perspective depuis la caméra
        self.draw_road_perspective(frame, camera);
    }

    fn draw_background(&self, frame: &mut [u8]) {
        let horizon_y = (self.height as f32 * 0.45) as usize;

        // Ciel (bleu)
        for y in 0..horizon_y {
            for x in 0..self.width {
                let idx = (y * self.width + x) * 4;
                if idx + 3 < frame.len() {
                    frame[idx] = 100;     // Bleu
                    frame[idx + 1] = 180;
                    frame[idx + 2] = 255;
                    frame[idx + 3] = 255;
                }
            }
        }

        // Herbe (vert)
        for y in horizon_y..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) * 4;
                if idx + 3 < frame.len() {
                    frame[idx] = 40;      // Vert foncé
                    frame[idx + 1] = 120;
                    frame[idx + 2] = 40;
                    frame[idx + 3] = 255;
                }
            }
        }
    }

    fn draw_road_perspective(&self, frame: &mut [u8], camera: &crate::camera::Camera) {
        const ROAD_COLOR: RGBA8 = RGBA8 { r: 60, g: 60, b: 60, a: 255 };
        const ROAD_LINE: RGBA8 = RGBA8 { r: 255, g: 255, b: 255, a: 255 };

        // Dessiner la route depuis près à loin
        for z_offset in (-200..1000).step_by(4) {
            let z = camera.z + z_offset as f32;

            let road_half_width = self.track_width / 2.0;

            // Points de la route à cette profondeur Z
            let left_point = (-road_half_width, z);
            let center_point = (0.0, z);
            let right_point = (road_half_width, z);

            // Projeter ces points sur l'écran
            let left_screen = camera.world_to_screen(left_point.0, left_point.1, 0.0, self.width, self.height);
            let center_screen = camera.world_to_screen(center_point.0, center_point.1, 0.0, self.width, self.height);
            let right_screen = camera.world_to_screen(right_point.0, right_point.1, 0.0, self.width, self.height);

            // Tracer une ligne horizontale si tous les points sont visibles
            if let (Some((left_x, left_y)), Some((center_x, center_y)), Some((right_x, right_y))) =
                (left_screen, center_screen, right_screen)
            {
                // Utiliser la position Y du centre pour tracer la ligne
                let screen_y = center_y as i32;

                if screen_y >= 0 && screen_y < self.height as i32 {
                    let y_usize = screen_y as usize;

                    // Tracer une ligne horizontale de gauche à droite
                    let left_x_i = (left_x.max(0.0) as i32).min(self.width as i32 - 1) as usize;
                    let right_x_i = (right_x.max(0.0) as i32).min(self.width as i32 - 1) as usize;

                    if left_x_i <= right_x_i {
                        let is_stripe = (z_offset / 4) % 15 < 8;

                        for screen_x in left_x_i..=right_x_i {
                            let idx = (y_usize * self.width + screen_x) * 4;
                            if idx + 3 < frame.len() {
                                let color = if is_stripe { ROAD_LINE } else { ROAD_COLOR };
                                frame[idx] = color.r;
                                frame[idx + 1] = color.g;
                                frame[idx + 2] = color.b;
                                frame[idx + 3] = color.a;
                            }
                        }
                    }
                }
            }
        }
    }
}
