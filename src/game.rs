use crate::camera::Camera;
use crate::player::Player;
use crate::track::Track;
use chinchilib::pixels::Pixels;
use chinchilib::{DoneStatus, GfxApp, Key};
use std::collections::HashSet;

pub struct RustKart {
    players: Vec<Player>,
    track: Track,
    camera: Camera,
    running: bool,
}

impl RustKart {
    pub fn new() -> Self {
        let players = vec![Player::new(0, -50, 100), Player::new(1, 50, 100)];
        let track = Track::new(1280, 720);
        let camera = Camera::new(0.0, 100.0);

        Self {
            players,
            track,
            camera,
            running: true,
        }
    }

    fn handle_input(&mut self, pressed_keys: &HashSet<Key>) {
        // Récupérer le joueur principal (P1 contrôlé par le joueur)
        if let Some(player) = self.players.first_mut() {
            // Contrôles AZERTY
            // Q et D pour tourner
            if pressed_keys.contains(&Key::KeyQ) {
                player.move_left();
            }
            if pressed_keys.contains(&Key::KeyD) {
                player.move_right();
            }
            // Z pour accélérer
            if pressed_keys.contains(&Key::KeyZ) {
                player.accelerate();
            }
            // S pour freiner
            if pressed_keys.contains(&Key::KeyS) {
                player.brake();
            }
            // A pour marche arrière
            if pressed_keys.contains(&Key::KeyA) {
                player.reverse();
            }
        }

        // P2 avec les touches directrices
        if let Some(player) = self.players.get_mut(1) {
            if pressed_keys.contains(&Key::Left) {
                player.move_left();
            }
            if pressed_keys.contains(&Key::Right) {
                player.move_right();
            }
            if pressed_keys.contains(&Key::Up) {
                player.accelerate();
            }
            if pressed_keys.contains(&Key::Down) {
                player.brake();
            }
            // X pour marche arrière
            if pressed_keys.contains(&Key::KeyX) {
                player.reverse();
            }
        }
    }
}

impl GfxApp for RustKart {
    fn on_tick(&mut self, pressed_keys: &HashSet<Key>) -> bool {
        self.handle_input(pressed_keys);

        // Update player positions and physics
        for player in &mut self.players {
            player.update();
        }

        // La caméra suit le joueur principal et prend en compte la vitesse pour l'effet de perspective
        if let Some(player) = self.players.first() {
            let speed = (player.velocity_x.powi(2) + player.velocity_z.powi(2)).sqrt();
            self.camera
                .follow_player(player.x, player.z, player.direction, speed);
        }

        true
    }

    fn draw(&mut self, pixels: &mut Pixels, width: usize) {
        let frame = pixels.frame_mut();

        // Calculer la hauteur réelle à partir de la taille du framebuffer
        let height = (frame.len() / 4) / width;

        // Mettre à jour la taille de la piste si nécessaire (support du redimensionnement)
        self.track.resize(width, height);

        // Draw track with camera perspective
        self.track.draw(frame, &self.camera);

        // Sort players by distance from camera (draw far ones first)
        let mut sorted_players: Vec<_> = self.players.iter().collect();
        sorted_players.sort_by(|a, b| {
            let dist_a = self.camera.distance_to(a.x, a.z);
            let dist_b = self.camera.distance_to(b.x, b.z);
            dist_b
                .partial_cmp(&dist_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Draw players from farthest to nearest
        for player in sorted_players {
            player.draw(frame, width, height, &self.camera);
        }
    }

    fn done(&self) -> DoneStatus {
        if self.running {
            DoneStatus::NotDone
        } else {
            DoneStatus::Exit
        }
    }
}
