//! Visual state container.

#[derive(Copy, Clone, Default)]
pub struct WidgetState(u32);

impl WidgetState {
    const SELECTION_STATE_BITS: u32 = 0x8000_0000;
    const SELECTION_STATE_BIT_POS: u32 = 31;

    pub fn selected(self) -> bool {
        (self.0 & Self::SELECTION_STATE_BITS) != 0
    }

    pub fn change_selection(&mut self, selected: bool) -> bool {
        self.change_state_bit(Self::SELECTION_STATE_BIT_POS, selected)
    }

    pub fn state(self) -> u32 {
        self.0 & !Self::SELECTION_STATE_BITS
    }

    pub fn change_state(&mut self, state: u32) -> bool {
        let old = self.0;
        self.0 = state & !Self::SELECTION_STATE_BITS;

        old != self.0
    }

    pub fn change_state_bit(&mut self, state_bit: u32, value: bool) -> bool {
        let old = self.0;
        debug_assert!(state_bit < 32);

        if value {
            self.0 |= 1 << state_bit;
        } else {
            self.0 &= !(1 << state_bit);
        }

        old != self.0
    }
}
