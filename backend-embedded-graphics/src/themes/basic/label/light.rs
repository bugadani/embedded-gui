//! Light theme for labels.

use crate::{label_style_binary_color, label_style_rgb};

label_style_binary_color!(
    Label {
        text: On,
        background: None,
        font: ascii::FONT_6X10,
    },
    Title {
        text: On,
        background: None,
        font: ascii::FONT_9X15,
    }
);

label_style_rgb!(
    Label {
        text: BLACK,
        background: None,
        font: ascii::FONT_6X10,
    },
    Title {
        text: BLACK,
        background: None,
        font: ascii::FONT_9X15,
    }
);