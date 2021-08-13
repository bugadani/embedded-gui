use crate::Position;

#[derive(Copy, Clone, Debug)]
pub enum Key {
    // FIXME there has to be a more generic way
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    Enter,
    Backspace,
    Space,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    ArrowLeft,
    Del,
    Tab,
    Comma,
    Period,
    Minus,
}

#[derive(Copy, Clone, Debug)]
pub enum Modifier {
    None,
    Shift,
    Alt,
    AltShift,
    Ctrl,
    CtrlAlt,
    CtrlShift,
}

pub trait ToStr {
    fn to_str(&self) -> Option<&str>;
}

impl ToStr for (Key, Modifier) {
    fn to_str(&self) -> Option<&str> {
        let options = match self.0 {
            Key::A => Some(("a", "A", "A", "ä")),
            Key::B => Some(("b", "B", "B", "{")),
            Key::C => Some(("c", "C", "C", "&")),
            Key::D => Some(("d", "D", "D", "Đ")),
            Key::E => Some(("e", "E", "E", "Ä")),
            Key::F => Some(("f", "F", "F", "[")),
            Key::G => Some(("g", "G", "G", "]")),
            Key::H => Some(("h", "H", "H", "")),
            Key::I => Some(("i", "I", "I", "Í")),
            Key::J => Some(("j", "J", "J", "í")),
            Key::K => Some(("k", "K", "K", "ł")),
            Key::L => Some(("l", "L", "L", "Ł")),
            Key::M => Some(("m", "M", "M", "<")),
            Key::N => Some(("n", "N", "N", "}")),
            Key::O => Some(("o", "O", "O", "")),
            Key::P => Some(("p", "P", "P", "")),
            Key::Q => Some(("q", "Q", "Q", "\\")),
            Key::R => Some(("r", "R", "R", "")),
            Key::S => Some(("s", "S", "S", "đ")),
            Key::T => Some(("t", "T", "T", "")),
            Key::U => Some(("u", "U", "U", "€")),
            Key::V => Some(("v", "V", "V", "@")),
            Key::W => Some(("w", "W", "W", "|")),
            Key::X => Some(("x", "X", "X", "#")),
            Key::Y => Some(("y", "Y", "Y", ">")),
            Key::Z => Some(("z", "Z", "Z", "")),
            Key::N0 => Some(("0", "§", "0", "")),
            Key::N1 => Some(("1", "'", "1", "~")),
            Key::N2 => Some(("2", "\"", "2", "ˇ")),
            Key::N3 => Some(("3", "+", "3", "^")),
            Key::N4 => Some(("4", "!", "4", "˘")),
            Key::N5 => Some(("5", "%", "5", "°")),
            Key::N6 => Some(("6", "/", "6", "˛")),
            Key::N7 => Some(("7", "=", "7", "`")),
            Key::N8 => Some(("8", "(", "8", "˙")),
            Key::N9 => Some(("9", ")", "9", "´")),
            Key::Space => Some((" ", " ", " ", " ")),
            Key::Comma => Some((",", "?", ",", " ")),
            Key::Period => Some((".", ":", ".", ">")),
            Key::Minus => Some(("-", "_", "-", "*")),
            Key::Enter => Some(("\n", "\n", "\n", "\n")),
            Key::Tab => Some(("\t", "\t", "\t", "\t")),
            _ => None,
        };

        options.and_then(|choices| match self.1 {
            Modifier::None => Some(choices.0),
            Modifier::Shift => Some(choices.1),
            Modifier::Alt | Modifier::CtrlAlt => Some(choices.3),
            Modifier::AltShift | Modifier::Ctrl | Modifier::CtrlShift => None,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum InputEvent {
    Cancel,
    KeyEvent(KeyEvent),
    PointerEvent(Position, PointerEvent),
    ScrollEvent(ScrollEvent),
}

#[derive(Copy, Clone, Debug)]
pub enum KeyEvent {
    KeyDown(Key, Modifier, u32),
    KeyUp(Key, Modifier),
}

#[derive(Copy, Clone, Debug)]
pub enum PointerEvent {
    Hover,
    Down,
    Drag,
    Up,
}

#[derive(Copy, Clone, Debug)]
pub enum ScrollEvent {
    HorizontalScroll(i32),
    VerticalScroll(i32),
}

pub enum SelectionModifier {
    None,
    GrabSelection(Position),
    TempSelection(Position),
}
