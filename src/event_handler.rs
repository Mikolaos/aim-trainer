use crate::aim_trainer::AimTrainer;
use crate::game_state::GameState; 
use crate::utils::keycode_to_char; 
use ggez::{event::{EventHandler, KeyCode, MouseButton}, graphics, mint, Context, GameResult}; // Import niezbędnych modułów z ggez
use std::time::{Duration, Instant}; 
use rand::Rng; 
use crate::game_mode::GameMode; // Import trybów gry

// Implementacja obsługi zdarzeń dla struktury AimTrainer
impl EventHandler for AimTrainer {
    // Funkcja aktualizująca stan gry
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.game_state {
            // Obsługa Countdown
            GameState::Countdown => {
                if let Some(start_time) = self.countdown_start_time {
                    if std::time::Instant::now() - start_time > std::time::Duration::from_secs(3) {
                        self.game_state = GameState::Playing; // Przejście do gry
                        self.game_start_time = std::time::Instant::now(); // Ustawienie czasu startu gry
                        self.last_target_time = std::time::Instant::now(); // Ustawienie czasu ostatniego celu

                        // Aktywacja chaotycznego ruchu w trybie Crazy
                        if let Some(GameMode::Crazy) = self.game_mode {
                            self.target_velocity = Some((0.0, 0.0)); // Początkowa prędkość celu
                            self.last_direction_change = Some(Instant::now());
                        }
                    }
                }
            }
            // Obsługa Playing
            GameState::Playing => {
                // Jeśli czas gry minął, zmień stan na GameOver
                if std::time::Instant::now() - self.game_start_time > self.game_duration {
                    self.game_state = GameState::GameOver;
                    self.save_score(); // Zapisz wynik
                    self.load_leaderboard(); // Załaduj tablicę wyników
                }

                // Logika dla trybu Crazy
                if let Some(GameMode::Crazy) = self.game_mode {
                    // Zmiana kierunku co 0.5–1 sekund
                    if let Some(last_change) = self.last_direction_change {
                        if last_change.elapsed() > Duration::from_millis(rand::thread_rng().gen_range(500..1000)) { // Cykl po którym zmienia kierunek i prędkość
                            let mut rng = rand::thread_rng();
                            self.target_velocity = Some((
                                rng.gen_range(-200.0..200.0), // Losowa prędkość X
                                rng.gen_range(-200.0..200.0), // Losowa prędkość Y
                            ));
                            self.last_direction_change = Some(Instant::now());
                        }
                    }

                    // Aktualizacja pozycji celu
                    if let Some(velocity) = self.target_velocity {
                        let (screen_width, screen_height) = graphics::drawable_size(ctx);
                        self.target.x += velocity.0 * ggez::timer::delta(ctx).as_secs_f32(); // Przesunięcie celu w osi X
                        self.target.y += velocity.1 * ggez::timer::delta(ctx).as_secs_f32(); // Przesunięcie celu w osi Y

                        // Odbicie celu od krawędzi ekranu
                        if self.target.x < 0.0 || self.target.x + self.target.w > screen_width {
                            self.target_velocity.as_mut().unwrap().0 *= -1.0; // Zmiana kierunku X
                        }
                        if self.target.y < 0.0 || self.target.y + self.target.h > screen_height {
                            self.target_velocity.as_mut().unwrap().1 *= -1.0; // Zmiana kierunku Y
                        }
                    }
                }

                // Logika pojawiania się nowych celów
                if self.target_visible {
                    let duration = match self.game_mode { // Ustalanie czasu życia celu w zależności od trybu
                        Some(GameMode::Normal) => Duration::from_secs(1),
                        Some(GameMode::Hard) => Duration::from_millis(800),
                        Some(GameMode::Crazy) => Duration::from_millis(1000),
                        None => Duration::from_secs(1),
                    };

                    if self.last_target_time.elapsed() > duration {
                        self.missed += 1; // Zwiększenie licznika nietrafionych celów
                        self.target = AimTrainer::random_target(ctx); // Generowanie nowego celu
                        self.last_target_time = Instant::now(); // Reset czasu ostatniego celu
                        self.update_target_size(); // Aktualizacja rozmiaru celu
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    // Funkcja rysująca elementy gry
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK); // Czyszczenie ekranu na czarno

        match self.game_state {
            // Wyświetlanie ekranu wprowadzania imienia
            GameState::AwaitingName => {
                let game_title = graphics::Text::new("Aim Trainer");
                let prompt = graphics::Text::new("Enter your name: ");
                let input = graphics::Text::new(self.input_buffer.clone());

                graphics::draw(ctx, &game_title, ([200.0, 50.0], graphics::Color::WHITE))?;
                graphics::draw(ctx, &prompt, ([10.0, 150.0], graphics::Color::WHITE))?;
                graphics::draw(ctx, &input, ([200.0, 150.0], graphics::Color::WHITE))?;
            }
            // Wyświetlanie ekranu wyboru trybu
            GameState::ChoosingMode => {
                let modes_text = graphics::Text::new("Choose a mode: [1] Normal [2] Hard [3] Crazy");
                graphics::draw(ctx, &modes_text, ([10.0, 10.0], graphics::Color::WHITE))?;
            }
            // Wyświetlanie odliczania
            GameState::Countdown => {
                if let Some(start_time) = self.countdown_start_time {
                    let elapsed = std::time::Instant::now() - start_time;
                    let countdown = 3 - elapsed.as_secs(); // Obliczanie pozostałego czasu
                    let countdown_text = graphics::Text::new(format!("Starting in: {}", countdown));
                    graphics::draw(ctx, &countdown_text, ([10.0, 10.0], graphics::Color::WHITE))?;
                }
            }
            // Wyświetlanie rozgrywki
            GameState::Playing => {
                if self.target_visible {
                    let target_mesh = graphics::Mesh::new_circle(
                        ctx,
                        graphics::DrawMode::fill(),
                        [self.target.x + self.target.w / 2.0, self.target.y + self.target.h / 2.0], //Środek celu
                        self.target.w / 2.0, // Promień celu
                        0.1,
                        graphics::Color::RED,
                    )?;
                    graphics::draw(ctx, &target_mesh, graphics::DrawParam::default())?;
                }

                let time_left = self.game_duration.as_secs() as i64
                    - (std::time::Instant::now() - self.game_start_time).as_secs() as i64; // Pozostały czas gry
                let text = graphics::Text::new(format!(
                    "Score: {} | Missed: {} |  Time Left: {}s",
                    self.score, self.missed, time_left
                ));
                graphics::draw(ctx, &text, ([10.0, 10.0], graphics::Color::WHITE))?;
            }
            // Wyświetlanie ekranu Game Over
            GameState::GameOver => {
                let result_text = graphics::Text::new(format!(
                    "Game Over!\nPlayer: {}\nScore: {}\nMissed: {}\nPress R to Restart\nPress M to Return to Mode Selection\nPress ESC to Exit",
                    self.player_name, self.score, self.missed
                ));
                graphics::draw(ctx, &result_text, ([10.0, 10.0], graphics::Color::WHITE))?;

                // Wyświetlanie tablicy wyników, niezły wąż
                let leaderboard_y_offset = 150.0;
                let leaderboard_text = self
                    .leaderboard
                    .iter()
                    .enumerate()
                    .map(|(i, (name, score, missed))| format!("{}. {} - Score: {}, Missed: {}", i + 1, name, score, missed))
                    .collect::<Vec<String>>()
                    .join("\n");

                let leaderboard_display = graphics::Text::new(format!("Leaderboard:\n{}", leaderboard_text));
                graphics::draw(ctx, &leaderboard_display, ([10.0, leaderboard_y_offset], graphics::Color::WHITE))?;
            }

        }

        graphics::present(ctx)?; // Wyświetlenie zaktualizowanego obrazu
        Ok(())
    }

    // Obsługa kliknięcia myszką
    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.game_state == GameState::Playing && button == MouseButton::Left {
            let click_position = mint::Point2 { x, y }; // Pozycja kliknięcia myszką
            if self.target.contains(click_position) {
                self.score += 1; // Trafienie - zwiększ wynik
            } else {
                self.missed += 1; // Nietrafienie - zwiększ licznik
                self.target_visible = false; // Cel chwilowo niewidoczny
            }

            // Generowanie nowego celu po kliknięciu
            self.target = AimTrainer::random_target(ctx);
            self.update_target_size(); // Aktualizacja rozmiaru celu
            self.target_visible = true;
            self.last_target_time = Instant::now(); // Reset czasu ostatniego celu
        }
    }

    // Obsługa naciśnięcia klawisza
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: ggez::input::keyboard::KeyMods, _repeat: bool) {
        match self.game_state {
            // Obsługa wprowadzania imienia
            GameState::AwaitingName => match keycode {
                KeyCode::Return => {
                    self.player_name = self.input_buffer.clone(); // Zapisz imię gracza
                    self.input_buffer.clear();
                    self.game_state = GameState::ChoosingMode; // Przejdź do wyboru trybu gry
                }
                KeyCode::Back => {
                    self.input_buffer.pop(); // Usuń ostatni znak
                }
                KeyCode::Escape => {
                    std::process::exit(0); // Zamknij grę
                }
                _ => {
                    if let Some(c) = keycode_to_char(keycode) {
                        self.input_buffer.push(c); // Dodaj znak do bufora
                    }
                }
            },
            // Obsługa wyboru trybu gry
            GameState::ChoosingMode => match keycode {
                KeyCode::Key1 => {
                    self.game_mode = Some(crate::game_mode::GameMode::Normal);
                    self.update_target_size(); // Ustaw rozmiar celu dla trybu Normal
                    self.game_state = GameState::Countdown; // Przejdź do odliczania
                    self.countdown_start_time = Some(std::time::Instant::now());
                }
                KeyCode::Key2 => {
                    self.game_mode = Some(crate::game_mode::GameMode::Hard);
                    self.update_target_size(); // Ustaw rozmiar celu dla trybu Hard
                    self.game_state = GameState::Countdown; // Przejdź do odliczania
                    self.countdown_start_time = Some(std::time::Instant::now());
                }
                KeyCode::Key3 => {
                    self.game_mode = Some(crate::game_mode::GameMode::Crazy);
                    self.update_target_size(); // Ustaw rozmiar celu dla trybu Crazy
                    self.game_state = GameState::Countdown; // Przejdź do odliczania
                    self.countdown_start_time = Some(std::time::Instant::now());
                }
                KeyCode::Escape => {
                    std::process::exit(0); // Zamknij grę
                }
                _ => {}
            },
            // Obsługa ekranu Game Over
            GameState::GameOver => match keycode {
                KeyCode::R => {
                    self.restart_game(); // Restartuj grę
                }
                KeyCode::Escape => {
                    std::process::exit(0); // Zamknij grę
                }
                KeyCode::M => {
                    self.game_state = GameState::ChoosingMode; // Powrót do wyboru trybu gry
                    self.score = 0; // Reset wyniku
                    self.missed = 0; // Reset liczby nietrafień
                    self.game_mode = None; // Reset trybu gry
                    self.input_buffer.clear();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
