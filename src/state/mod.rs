//! Visual state container.
pub mod selection;

pub trait StateGroup {
    const MASK: u32;
}

pub trait State {
    type Group: StateGroup;

    const VALUE: u32;
}

#[macro_export]
macro_rules! state_group {
    ($([$group:ident: $mask:literal] = {
        $($state:ident = $value:literal),+ $(,)?
    })+) => {
        $(
            pub struct $group;
            impl StateGroup for $group {
                const MASK: u32 = $mask;
            }

            $(
                pub struct $state;
                impl State for $state {
                    type Group = $group;

                    const VALUE: u32 = $value;
                }
            )+
        )+
    };
}

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
