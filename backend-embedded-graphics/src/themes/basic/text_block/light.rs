//! Light theme for labels.

use crate::{text_block_style_binary_color, text_block_style_rgb};

text_block_style_binary_color!(TextBlock {
    text: On,
    background: None,
    font: ascii::FONT_6X10,
});

text_block_style_rgb!(TextBlock {
    text: BLACK,
    background: None,
    font: ascii::FONT_6X10,
});
