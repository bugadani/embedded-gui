use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    pixelcolor::BinaryColor,
};

use crate::themes::default::{radio_button::RadioButtonStateColors, RadioButtonVisualStyle};

pub struct RadioButtonInactive;
pub struct RadioButtonIdle;
pub struct RadioButtonHovered;
pub struct RadioButtonPressed;

impl RadioButtonStateColors<BinaryColor> for RadioButtonInactive {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
    const CHECK_MARK_COLOR: BinaryColor = BinaryColor::On;
}

impl RadioButtonStateColors<BinaryColor> for RadioButtonIdle {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
    const CHECK_MARK_COLOR: BinaryColor = BinaryColor::On;
}

impl RadioButtonStateColors<BinaryColor> for RadioButtonHovered {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
    const CHECK_MARK_COLOR: BinaryColor = BinaryColor::On;
}

impl RadioButtonStateColors<BinaryColor> for RadioButtonPressed {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
    const CHECK_MARK_COLOR: BinaryColor = BinaryColor::On;
}

pub struct RadioButtonStyle;
impl RadioButtonVisualStyle<BinaryColor> for RadioButtonStyle {
    type Inactive = RadioButtonInactive;
    type Idle = RadioButtonIdle;
    type Hovered = RadioButtonHovered;
    type Pressed = RadioButtonPressed;

    const FONT: MonoFont<'static> = FONT_6X10;
}
