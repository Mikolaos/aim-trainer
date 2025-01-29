// Definicja trybów gry
#[derive(PartialEq)] 
pub enum GameMode {
    Normal, // Normalny tryb gry
    Hard,   // Trudny tryb gry (mniejszy cel, krótszy czas widoczności)
    Crazy,  // Chaotyczny tryb gry (ruchomy cel)
}