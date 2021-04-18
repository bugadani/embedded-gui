//! Visual state container.

pub trait StateGroup {
    const MASK: u32;
}

pub trait State {
    type Group: StateGroup;

    const VALUE: u32;
}

mod selection;

#[derive(Copy, Clone, Default)]
pub struct WidgetState(u32);

impl WidgetState {
    pub fn has_state<S: State>(self, _state: S) -> bool {
        self.0 & S::Group::MASK == S::VALUE
    }

    pub fn set_state<S: State>(&mut self, _state: S) -> bool {
        let old = self.0;
        self.0 = (self.0 & !S::Group::MASK) | S::VALUE;
        self.0 != old
    }
}
