//! Light theme for toggle buttons.

use crate::{toggle_button_style_binary_color, toggle_button_style_rgb};

toggle_button_style_binary_color!(
    ToggleButton {
        font: ascii::FONT_6X10,
        states: {
            Unchecked, Checked: {
                Inactive, Idle, Pressed: {
                    label: Off,
                    border: On,
                    background: On,
                },
                Hovered: {
                    label: On,
                    border: On,
                    background: Off,
                }
            }
        }
    }
);

toggle_button_style_rgb!(
    ToggleButton {
        font: ascii::FONT_6X10,
        states: {
            Unchecked, Checked: {
                Inactive: {
                    label: CSS_LIGHT_GRAY,
                    border: CSS_DIM_GRAY,
                    background: CSS_DIM_GRAY,
                },
                Idle: {
                    label: WHITE,
                    border: CSS_STEEL_BLUE,
                    background: CSS_STEEL_BLUE,
                },
                Hovered: {
                    label: WHITE,
                    border: CSS_DODGER_BLUE,
                    background: CSS_DODGER_BLUE,
                },
                Pressed: {
                    label: WHITE,
                    border: CSS_LIGHT_STEEL_BLUE,
                    background: CSS_LIGHT_STEEL_BLUE,
                }
            }
        }
    }
);
