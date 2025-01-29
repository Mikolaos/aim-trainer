// Import klawiszy z biblioteki ggez
use ggez::event::KeyCode;

// Funkcja konwertująca kod klawisza na znak
pub fn keycode_to_char(keycode: KeyCode) -> Option<char> {
    match keycode {
        // Mapowanie kodów klawiszy na odpowiadające im małe litery
        KeyCode::A => Some('a'),
        KeyCode::B => Some('b'),
        KeyCode::C => Some('c'),
        KeyCode::D => Some('d'),
        KeyCode::E => Some('e'),
        KeyCode::F => Some('f'),
        KeyCode::G => Some('g'),
        KeyCode::H => Some('h'),
        KeyCode::I => Some('i'),
        KeyCode::J => Some('j'),
        KeyCode::K => Some('k'),
        KeyCode::L => Some('l'),
        KeyCode::M => Some('m'),
        KeyCode::N => Some('n'),
        KeyCode::O => Some('o'),
        KeyCode::P => Some('p'),
        KeyCode::Q => Some('q'),
        KeyCode::R => Some('r'),
        KeyCode::S => Some('s'),
        KeyCode::T => Some('t'),
        KeyCode::U => Some('u'),
        KeyCode::V => Some('v'),
        KeyCode::W => Some('w'),
        KeyCode::X => Some('x'),
        KeyCode::Y => Some('y'),
        KeyCode::Z => Some('z'),
        KeyCode::Space => Some(' '), // Mapa klawisza spacji na znak ' '
        _ => None, // Zwraca None, jeśli klawisz nie jest obsługiwany
    }
}