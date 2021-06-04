use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    pixelcolor::BinaryColor,
};

use crate::themes::default::toggle_button::{
    ButtonStateColors, ToggleButtonStyle as ToggleButtonStyleTrait,
};

pub struct ToggleButtonInactive;
pub struct ToggleButtonIdle;
pub struct ToggleButtonHovered;
pub struct ToggleButtonPressed;

impl ButtonStateColors<BinaryColor> for ToggleButtonInactive {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for ToggleButtonIdle {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for ToggleButtonHovered {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for ToggleButtonPressed {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
}

pub struct ToggleButtonInactiveChecked;
pub struct ToggleButtonIdleChecked;
pub struct ToggleButtonHoveredChecked;
pub struct ToggleButtonPressedChecked;

impl ButtonStateColors<BinaryColor> for ToggleButtonInactiveChecked {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for ToggleButtonIdleChecked {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
}

impl ButtonStateColors<BinaryColor> for ToggleButtonHoveredChecked {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for ToggleButtonPressedChecked {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

pub struct ToggleButtonStyle;
impl ToggleButtonStyleTrait<BinaryColor> for ToggleButtonStyle {
    type Inactive = ToggleButtonInactive;
    type Idle = ToggleButtonIdle;
    type Hovered = ToggleButtonHovered;
    type Pressed = ToggleButtonPressed;

    type InactiveChecked = ToggleButtonInactiveChecked;
    type IdleChecked = ToggleButtonIdleChecked;
    type HoveredChecked = ToggleButtonHoveredChecked;
    type PressedChecked = ToggleButtonPressedChecked;

    const FONT: MonoFont<'static> = FONT_6X10;
}
