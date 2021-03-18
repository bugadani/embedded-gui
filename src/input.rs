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
    Ctrl,
    Alt,
    Shift,
    Del,
    Tab,
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

#[derive(Copy, Clone, Debug)]
pub enum InputEvent {
    KeyDown(Key, Modifier, u32),
    KeyUp(Key, Modifier),
    PointerHover(Position),
    PointerDown(Position),
    PointerMove(Position),
    PointerUp(Position),
    HorizontalScroll(i32),
    VerticalScroll(i32),
}

pub enum SelectionModifier {
    None,
    GrabSelection(Position),
    TempSelection(Position),
}

impl InputEvent {
    pub fn selection_modifier(self) -> SelectionModifier {
        match self {
            InputEvent::PointerDown(pos) => SelectionModifier::GrabSelection(pos),
            InputEvent::PointerHover(pos) => SelectionModifier::TempSelection(pos),
            _ => SelectionModifier::None,
        }
    }
}
