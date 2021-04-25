use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    pixelcolor::BinaryColor,
};

use crate::themes::default::{check_box::CheckBoxStateColors, CheckBoxVisualStyle};

pub struct CheckBoxInactive;
pub struct CheckBoxIdle;
pub struct CheckBoxHovered;
pub struct CheckBoxPressed;

impl CheckBoxStateColors<BinaryColor> for CheckBoxInactive {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
    const CHECK_MARK_COLOR: BinaryColor = BinaryColor::On;
}

impl CheckBoxStateColors<BinaryColor> for CheckBoxIdle {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
    const CHECK_MARK_COLOR: BinaryColor = BinaryColor::On;
}

impl CheckBoxStateColors<BinaryColor> for CheckBoxHovered {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
    const CHECK_MARK_COLOR: BinaryColor = BinaryColor::On;
}

impl CheckBoxStateColors<BinaryColor> for CheckBoxPressed {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
    const CHECK_MARK_COLOR: BinaryColor = BinaryColor::On;
}

pub struct CheckBoxStyle;
impl CheckBoxVisualStyle<BinaryColor> for CheckBoxStyle {
    type Inactive = CheckBoxInactive;
    type Idle = CheckBoxIdle;
    type Hovered = CheckBoxHovered;
    type Pressed = CheckBoxPressed;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
}
