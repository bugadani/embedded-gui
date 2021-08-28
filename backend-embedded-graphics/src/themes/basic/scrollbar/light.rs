//! Light theme for scrollbars.

use crate::{scrollbar_style_binary_color, scrollbar_style_rgb};

// Note: macro defines Horizontal and Vertical variants with the same prefix to the name.
scrollbar_style_binary_color!(
    Scrollbar {
        thickness: 6,
        states: {
            Inactive, Idle: {
                background_fill: None,
                background_border: None,
                background_border_thickness: 0,
                fill: Off,
                border: On,
                border_thickness: 1,
            },
            Hovered, Dragged: {
                background_fill: None,
                background_border: None,
                background_border_thickness: 0,
                fill: On,
                border: Off,
                border_thickness: 1,
            }
        }
    }
);

scrollbar_style_rgb!(
    Scrollbar {
        thickness: 6,
        states: {
            Inactive, Idle: {
                background_fill: None,
                background_border: None,
                background_border_thickness: 0,
                fill: CSS_SLATE_GRAY,
                border: None,
                border_thickness: 0,
            },
            Hovered: {
                background_fill: None,
                background_border: None,
                background_border_thickness: 0,
                fill: CSS_LIGHT_SLATE_GRAY,
                border: None,
                border_thickness: 0,
            },
            Dragged: {
                background_fill: None,
                background_border: None,
                background_border_thickness: 0,
                fill: CSS_STEEL_BLUE,
                border: None,
                border_thickness: 0,
            }
        }
    }
);
