//! Light theme for toggle buttons.

use crate::{radio_button_style_binary_color, radio_button_style_rgb};

radio_button_style_binary_color!(
    RadioButton {
        font: ascii::FONT_6X10,
        states: {
            Inactive, Idle, Hovered, Pressed: {
                label: On,
                border: On,
                background: Off,
                check_mark: On,
            }
        }
    }
);

radio_button_style_rgb!(
    RadioButton {
        font: ascii::FONT_6X10,
        states: {
            Inactive: {
                label: CSS_GRAY,
                border: CSS_GRAY,
                background: CSS_DARK_GRAY,
                check_mark: CSS_STEEL_BLUE,
            },
            Idle: {
                label: BLACK,
                border: BLACK,
                background: WHITE,
                check_mark: CSS_DODGER_BLUE,
            },
            Hovered: {
                label: BLACK,
                border: BLACK,
                background: CSS_LIGHT_GRAY,
                check_mark: CSS_DODGER_BLUE,
            },
            Pressed: {
                label: BLACK,
                border: BLACK,
                background: CSS_DARK_GRAY,
                check_mark: CSS_DODGER_BLUE,
            }
        }
    }
);
