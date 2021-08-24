//! Light theme for toggle buttons.

use crate::check_box_style_rgb;

pub mod binary_color {
    use crate::check_box_style;
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoFont},
        pixelcolor::BinaryColor,
    };

    check_box_style!(
        CheckBox<BinaryColor, FONT_6X10> {
            Inactive {
                label: On,
                border: On,
                background: Off,
                check_mark: On,
            },
            Idle {
                label: On,
                border: On,
                background: Off,
                check_mark: On,
            },
            Hovered {
                label: On,
                border: On,
                background: Off,
                check_mark: On,
            },
            Pressed {
                label: On,
                border: On,
                background: Off,
                check_mark: On,
            }
        }
    );
}

check_box_style_rgb!(
    CheckBox<FONT_6X10> {
        Inactive {
            label: CSS_GRAY,
            border: CSS_GRAY,
            background: CSS_DARK_GRAY,
            check_mark: CSS_STEEL_BLUE,
        },
        Idle {
            label: BLACK,
            border: BLACK,
            background: WHITE,
            check_mark: CSS_DODGER_BLUE,
        },
        Hovered {
            label: BLACK,
            border: BLACK,
            background: CSS_LIGHT_GRAY,
            check_mark: CSS_DODGER_BLUE,
        },
        Pressed {
            label: BLACK,
            border: BLACK,
            background: CSS_DARK_GRAY,
            check_mark: CSS_DODGER_BLUE,
        }
    }
);
