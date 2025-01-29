use crate::game_state::GameState; // Import stanu gry
use crate::game_mode::GameMode; // Import trybu gry
use ggez::{
    graphics::{self, Rect}, // Obsługa grafiki i prostokątów
    Context, GameResult,    // Kontekst gry i wynik funkcji
};
use rand::Rng; // Generator liczb losowych
use std::{
    fs::{File, OpenOptions}, // Obsługa plików
    io::{BufRead, BufReader, Write}, // Operacje na strumieniach wejścia/wyjścia
    time::{Duration, Instant}, // Obsługa czasu i trwania
};

// Struktura AimTrainer - reprezentuje główną logikę gry
pub struct AimTrainer {
    pub target: Rect, // Prostokąt reprezentujący cel
    pub score: u32, // Wynik gracza
    pub missed: u32, // Liczba nietrafionych strzałów
    pub target_visible: bool, // Widoczność celu
    pub last_target_time: Instant, // Czas ostatniego pojawienia się celu
    pub game_start_time: Instant, // Czas rozpoczęcia gry
    pub countdown_start_time: Option<Instant>, // Czas rozpoczęcia odliczania
    pub game_duration: Duration, // Czas trwania gry
    pub game_state: GameState, // Aktualny stan gry
    pub game_mode: Option<GameMode>, // Wybrany tryb gry
    pub player_name: String, // Imię gracza
    pub input_buffer: String,
    pub leaderboard: Vec<(String, u32, u32)>, // Tablica wyników: (imię, wynik, nietrafienia)
    pub target_velocity: Option<(f32, f32)>, // Prędkość celu w trybie Crazy
    pub last_direction_change: Option<Instant>, // Czas ostatniej zmiany kierunku w trybie Crazy
}

impl AimTrainer {
    // Konstruktor gry - inicjalizacja wartości początkowych
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let target = AimTrainer::random_target(ctx); // Generowanie początkowego celu
        Ok(AimTrainer {
            target,
            score: 0,
            missed: 0,
            target_visible: true,
            last_target_time: Instant::now(),
            game_start_time: Instant::now(),
            countdown_start_time: None,
            game_duration: Duration::from_secs(30), // Domyślny czas gry: 30 sekund
            game_state: GameState::AwaitingName, // Oczekiwanie na wprowadzenie imienia
            game_mode: None, // Tryb gry nie został jeszcze wybrany
            player_name: String::new(),
            input_buffer: String::new(), //Buffer do wyświetlania nazwy gracza
            leaderboard: vec![],
            target_velocity: None,
            last_direction_change: None,
        })
    }

    // Funkcja generująca losowy cel na ekranie
    pub(crate) fn random_target(ctx: &Context) -> Rect {
        let mut rng = rand::thread_rng(); // Inicjalizacja generatora losowego
        let (screen_width, screen_height) = graphics::drawable_size(ctx); // Rozmiary ekranu
        let size = 50.0; // Domyślny rozmiar celu
        let x = rng.gen_range(0.0..(screen_width - size)); // Losowa pozycja X
        let y = rng.gen_range(0.0..(screen_height - size)); // Losowa pozycja Y
        Rect::new(x, y, size, size) // Tworzenie nowego prostokąta
    }

    // Zapis wyniku gracza do pliku
    pub fn save_score(&self) {
        let mut file = OpenOptions::new()
            .create(true) // Tworzenie pliku, jeśli nie istnieje
            .append(true) // Dodawanie na końcu pliku
            .open("scores.txt") 
            .expect("Unable to open scores file"); 

        // Zapis wyniku w eleganckim formacie
        writeln!(
            file,
            "Player: {} | Score: {} | Missed: {}",
            self.player_name, self.score, self.missed
        )
            .expect("Unable to write to scores file");
    }

    // Wczytanie tablicy wyników z pliku
    pub fn load_leaderboard(&mut self) {
        let file = File::open("scores.txt").unwrap_or_else(|_| File::create("scores.txt").unwrap()); // Tworzenie pliku, jeśli nie istnieje
        let reader = BufReader::new(file); // Bufor do odczytu pliku
        let mut scores = vec![]; // Tymczasowy wektor wyników

        // Parsowanie każdej linii pliku, o matko bosko!
        for line in reader.lines() {
            if let Ok(entry) = line {
                if let Some((name_part, rest)) = entry.split_once(" | Score: ") {
                    if let Some((score_part, missed_part)) = rest.split_once(" | Missed: ") {
                        if let Ok(score) = score_part.parse::<u32>() {
                            if let Ok(missed) = missed_part.parse::<u32>() {
                                let name = name_part.strip_prefix("Player: ").unwrap_or("").to_string();
                                scores.push((name, score, missed));
                            }
                        }
                    }
                }
            }
        }

        // Sortowanie wyników malejąco po wyniku
        scores.sort_by(|a, b| b.1.cmp(&a.1));
        self.leaderboard = scores.into_iter().take(10).collect(); // Zapis tylko top 10 wyników
    }

    // Restartowanie gry
    pub fn restart_game(&mut self) {
        self.score = 0; // Zerowanie wyników
        self.missed = 0; 
        self.target_visible = true; // Ustawienie celu na widoczny
        self.last_target_time = Instant::now(); // Reset czasu celu
        self.game_start_time = Instant::now(); // Reset czasu gry
        self.countdown_start_time = None; // Brak odliczania
        self.game_state = GameState::Playing; // Stan gry: gra w toku
        self.update_target_size(); // Aktualizacja rozmiaru celu
    }

    // Aktualizacja rozmiaru celu w zależności od trybu gry
    pub fn update_target_size(&mut self) {
        if let Some(mode) = &self.game_mode {
            match mode {
                GameMode::Normal => self.target.w = 50.0, // Normalny rozmiar
                GameMode::Hard => self.target.w = 37.5,  // Mniejszy cel (75%)
                GameMode::Crazy => self.target.w = 50.0, 
            }
            self.target.h = self.target.w; // Ustawienie wysokości równej szerokości
        }
    }
}
