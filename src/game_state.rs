// Definicja stanów gry
#[derive(PartialEq)]
pub enum GameState {
    AwaitingName,    // Oczekiwanie na wprowadzenie nazwy gracza
    ChoosingMode,    // Wybór trybu gry
    Countdown,       // Odliczanie przed rozpoczęciem gry
    Playing,         // Granko
    GameOver,        // Koniec gry
}
