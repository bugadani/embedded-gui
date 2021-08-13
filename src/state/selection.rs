//! Focused state.

crate::state_group! {
    [SelectionStateGroup: 0x8000_0000] = {
        Unselected = 0,
        Selected = 0x8000_0000,
    }
}
