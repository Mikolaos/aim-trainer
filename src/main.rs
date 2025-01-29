// Import modułów gry
mod aim_trainer;
mod game_state;
mod game_mode;
mod utils;
mod event_handler;

// Import bibliotek ggez do obsługi zdarzeń i gry
use ggez::{event, GameResult};
// Import struktury AimTrainer z modułu
use aim_trainer::AimTrainer;
use std::env;

fn main() -> GameResult {
    
    if cfg!(target_os = "linux") {
        env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
    // Tworzenie kontekstu gry i pętli zdarzeń
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Aim Trainer", "Mikołaj Lorenc") // Nazwa gry i autora
        .window_setup(ggez::conf::WindowSetup::default().title("Aim Trainer - Headshot!")) // Opis u góry wygenerowanego okna
        .build()
        .expect("Could not create ggez context");
    
    let game = AimTrainer::new(&mut ctx)?; // Tworzymy i uruchamiamy gre
    event::run(ctx, event_loop, game)
}
