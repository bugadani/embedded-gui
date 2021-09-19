//! Light theme for buttons.

use crate::{slider_style_binary_color, slider_style_rgb};

slider_style_binary_color!(
    Slider {
        direction: Horizontal,
        thickness: 7,
        width: 5,
        states: {
            Inactive, Idle: {
                fill: Off,
                border: On,
                background: On,
                border_thickness: 1,
                background_thickness: 1,
            },
            Hovered, Dragged: {
                fill: On,
                border: Off,
                background: On,
                border_thickness: 1,
                background_thickness: 1,
            }
        }
    }
);

slider_style_rgb!(
    Slider {
        direction: Horizontal,
        thickness: 7,
        width: 5,
        states: {
            Inactive, Idle: {
                fill: CSS_SLATE_GRAY,
                border: None,
                background: CSS_GRAY,
                border_thickness: 0,
                background_thickness: 1,
            },
            Hovered: {
                fill: CSS_LIGHT_SLATE_GRAY,
                border: None,
                background: CSS_GRAY,
                border_thickness: 0,
                background_thickness: 1,
            },
            Dragged: {
                fill: CSS_STEEL_BLUE,
                border: None,
                background: CSS_GRAY,
                border_thickness: 0,
                background_thickness: 1,
            }
        }
    }
);
