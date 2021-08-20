//! Light theme for labels.

use crate::label_style_rgb;

pub mod binary_color {
    use crate::label_style;
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoFont},
        pixelcolor::BinaryColor,
    };

    label_style!(Label<BinaryColor> {
        text: On,
        background: None,
        font: FONT_6X10,
    });
}

label_style_rgb!(Label {
    text: BLACK,
    background: None,
    font: FONT_6X10,
});
