//! Light theme for toggle buttons.

use crate::{check_box_style_binary_color, check_box_style_rgb};

check_box_style_binary_color!(
    CheckBox {
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

check_box_style_rgb!(
    CheckBox {
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
