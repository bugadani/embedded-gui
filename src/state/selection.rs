//! Focused state.

use super::{State, StateGroup};

pub struct SelectionStateGroup;
impl StateGroup for SelectionStateGroup {
    const MASK: u32 = 0x8000_000;
}

pub struct Selected;
impl State for Selected {
    type Group = SelectionStateGroup;

    const VALUE: u32 = 0x8000_000;
}

pub struct Unselected;
impl State for Unselected {
    type Group = SelectionStateGroup;

    const VALUE: u32 = 0x0000_000;
}
