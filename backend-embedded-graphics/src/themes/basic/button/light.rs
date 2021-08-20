//! Light theme for buttons.

use crate::button_style_rgb;

pub mod binary_color {
    use crate::button_style;
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoFont},
        pixelcolor::BinaryColor,
    };

    button_style!(
        PrimaryButton<BinaryColor, FONT_6X10> {
            Inactive {
                label: Off,
                border: On,
                background: On,
            },
            Idle {
                label: Off,
                border: On,
                background: On,
            },
            Hovered {
                label: On,
                border: On,
                background: Off,
            },
            Pressed {
                label: Off,
                border: On,
                background: On,
            }
        },
        SecondaryButton<BinaryColor, FONT_6X10> {
            Inactive {
                label: On,
                border: Off,
                background: Off,
            },
            Idle {
                label: On,
                border: Off,
                background: Off,
            },
            Hovered {
                label: On,
                border: On,
                background: Off,
            },
            Pressed {
                label: Off,
                border: Off,
                background: On,
            }
        }
    );
}

button_style_rgb!(
    PrimaryButton<FONT_6X10> {
        Inactive {
            label: CSS_LIGHT_GRAY,
            border: CSS_DIM_GRAY,
            background: CSS_DIM_GRAY,
        },
        Idle {
            label: WHITE,
            border: CSS_STEEL_BLUE,
            background: CSS_STEEL_BLUE,
        },
        Hovered {
            label: WHITE,
            border: CSS_DODGER_BLUE,
            background: CSS_DODGER_BLUE,
        },
        Pressed {
            label: WHITE,
            border: CSS_LIGHT_STEEL_BLUE,
            background: CSS_LIGHT_STEEL_BLUE,
        }
    },
    SecondaryButton<FONT_6X10> {
        Inactive {
            label: CSS_LIGHT_GRAY,
            border: CSS_DIM_GRAY,
            background: CSS_DIM_GRAY,
        },
        Idle {
            label: WHITE,
            border: CSS_SLATE_GRAY,
            background: CSS_SLATE_GRAY,
        },
        Hovered {
            label: WHITE,
            border: CSS_LIGHT_SLATE_GRAY,
            background: CSS_LIGHT_SLATE_GRAY,
        },
        Pressed {
            label: WHITE,
            border: CSS_STEEL_BLUE,
            background: CSS_STEEL_BLUE,
        }
    }
);
