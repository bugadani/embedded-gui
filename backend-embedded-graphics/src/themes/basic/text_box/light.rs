//! Light theme for labels.

use crate::{text_box_style_binary_color, text_box_style_rgb};

text_box_style_binary_color!(TextBox {
    text: On,
    background: None,
    font: ascii::FONT_6X10,
});

text_box_style_rgb!(TextBox {
    text: BLACK,
    background: None,
    font: ascii::FONT_6X10,
});
