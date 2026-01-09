mod camera;
mod game;
mod graphics;
mod player;
mod track;

use chinchilib::WinitHandler;
use game::RustKart;

fn main() {
    env_logger::init();

    let game = Box::new(RustKart::new());
    let mut app = WinitHandler::new(game, (1280, 720), 60);
    app.set_always_tick(true);

    match app.run() {
        Ok(_) => println!("RustKart exited successfully"),
        Err(e) => eprintln!("RustKart error: {}", e),
    }
}
