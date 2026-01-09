use chinchilib::rgb::RGBA8;

pub struct Player {
    #[allow(dead_code)]
    pub id: usize,
    pub x: f32,
    pub z: f32,
    pub y: f32, // Hauteur (le kart est légèrement au-dessus du sol)
    pub velocity_x: f32,
    pub velocity_z: f32,
    pub acceleration: f32,
    pub max_speed: f32,
    pub friction: f32,
    pub color: RGBA8,
    pub direction: f32,
    pub steering: f32,
}

impl Player {
    pub fn new(id: usize, x: i32, z: i32) -> Self {
        let color = match id {
            0 => RGBA8 {
                r: 255,
                g: 100,
                b: 0,
                a: 255,
            }, // Orange (Mario)
            1 => RGBA8 {
                r: 0,
                g: 150,
                b: 255,
                a: 255,
            }, // Cyan (Luigi)
            2 => RGBA8 {
                r: 255,
                g: 50,
                b: 200,
                a: 255,
            }, // Pink
            _ => RGBA8 {
                r: 200,
                g: 200,
                b: 0,
                a: 255,
            }, // Yellow
        };

        Self {
            id,
            x: x as f32,
            z: z as f32,
            y: 3.0, // Le kart est 3 unités au-dessus du sol
            velocity_x: 0.0,
            velocity_z: 0.0,
            acceleration: 0.8,
            max_speed: 15.0,
            friction: 0.95,
            color,
            direction: 0.0,
            steering: 0.0,
        }
    }

    pub fn move_left(&mut self) {
        self.steering = -0.1;
    }

    pub fn move_right(&mut self) {
        self.steering = 0.1;
    }

    pub fn accelerate(&mut self) {
        let current_speed = (self.velocity_x.powi(2) + self.velocity_z.powi(2)).sqrt();
        if current_speed < self.max_speed {
            self.velocity_x += self.direction.sin() * self.acceleration;
            self.velocity_z += self.direction.cos() * self.acceleration;
        }
    }

    pub fn brake(&mut self) {
        self.velocity_x *= 0.8;
        self.velocity_z *= 0.8;
    }

    pub fn reverse(&mut self) {
        let current_speed = (self.velocity_x.powi(2) + self.velocity_z.powi(2)).sqrt();
        let max_reverse_speed = self.max_speed * 0.6;

        if current_speed < max_reverse_speed {
            // Accélérer en arrière
            self.velocity_x -= self.direction.sin() * (self.acceleration * 0.7);
            self.velocity_z -= self.direction.cos() * (self.acceleration * 0.7);
        }
    }

    pub fn update(&mut self) {
        self.direction += self.steering * 0.1;
        self.steering *= 0.85;

        self.velocity_x *= self.friction;
        self.velocity_z *= self.friction;

        self.x += self.velocity_x;
        self.z += self.velocity_z;

        const TRACK_HALF_WIDTH: f32 = 140.0;
        const COLLISION_DAMPING: f32 = 0.5;

        if self.x < -TRACK_HALF_WIDTH {
            self.x = -TRACK_HALF_WIDTH;
            self.velocity_x *= -COLLISION_DAMPING;
        }
        if self.x > TRACK_HALF_WIDTH {
            self.x = TRACK_HALF_WIDTH;
            self.velocity_x *= -COLLISION_DAMPING;
        }

        if self.z < 0.0 {
            self.z = 0.0;
            // Permettre le recul au démarrage
            if self.velocity_z > 0.0 {
                self.velocity_z = 0.0;
            }
        }

        if self.z > 10000.0 {
            self.z = 10000.0;
        }
    }

    pub fn draw(
        &self,
        frame: &mut [u8],
        width: usize,
        height: usize,
        camera: &crate::camera::Camera,
    ) {
        if let Some((screen_x, screen_y)) =
            camera.world_to_screen(self.x, self.z, self.y, width, height)
        {
            let distance = camera.distance_to(self.x, self.z);

            // Taille du sprite basée sur la distance
            let base_scale = (100.0 / (distance + 20.0)).min(2.0).max(0.3);
            let kart_width = ((40.0 * base_scale) as usize).max(6);
            let kart_height = ((24.0 * base_scale) as usize).max(4);

            let x_start = screen_x as i32 - (kart_width as i32 / 2);
            let y_start = screen_y as i32 - (kart_height as i32 / 2);

            // Dessiner un sprite simple (carré avec contour)
            self.draw_kart_sprite(
                frame,
                width,
                height,
                x_start,
                y_start,
                kart_width,
                kart_height,
            );
        }
    }

    fn draw_kart_sprite(
        &self,
        frame: &mut [u8],
        width: usize,
        height: usize,
        x_start: i32,
        y_start: i32,
        w: usize,
        h: usize,
    ) {
        // Remplir le kart
        for dy in 0..h {
            for dx in 0..w {
                let px = (x_start + dx as i32) as usize;
                let py = (y_start + dy as i32) as usize;

                if px < width && py < height {
                    let idx = (py * width + px) * 4;
                    if idx + 3 < frame.len() {
                        let is_edge = dy == 0 || dy == h - 1 || dx == 0 || dx == w - 1;

                        if is_edge {
                            // Contour noir
                            frame[idx] = 30;
                            frame[idx + 1] = 30;
                            frame[idx + 2] = 30;
                        } else {
                            // Couleur du joueur
                            frame[idx] = self.color.r;
                            frame[idx + 1] = self.color.g;
                            frame[idx + 2] = self.color.b;
                        }
                        frame[idx + 3] = 255;
                    }
                }
            }
        }

        // Dessiner les yeux/phares (petit point blanc devant)
        if h > 4 {
            let eye_y = y_start + 2;
            let left_eye_x = x_start + (w / 4) as i32;
            let right_eye_x = x_start + (3 * w / 4) as i32;

            for eye_x in [left_eye_x, right_eye_x] {
                if eye_x >= 0 && eye_x < width as i32 && eye_y >= 0 && eye_y < height as i32 {
                    let idx = (eye_y as usize * width + eye_x as usize) * 4;
                    if idx + 3 < frame.len() {
                        frame[idx] = 255;
                        frame[idx + 1] = 255;
                        frame[idx + 2] = 255;
                        frame[idx + 3] = 255;
                    }
                }
            }
        }
    }
}
