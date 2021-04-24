use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    pixelcolor::BinaryColor,
};

use crate::themes::{
    default::{ButtonStateColors, ButtonStyle, DefaultTheme},
    Theme,
};

// region: Primary button

pub struct PrimaryButtonDisabled;
pub struct PrimaryButtonIdle;
pub struct PrimaryButtonHovered;
pub struct PrimaryButtonPressed;

impl ButtonStateColors<BinaryColor> for PrimaryButtonDisabled {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
}

impl ButtonStateColors<BinaryColor> for PrimaryButtonIdle {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
}

impl ButtonStateColors<BinaryColor> for PrimaryButtonHovered {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for PrimaryButtonPressed {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
}

pub struct PrimaryButtonStyle;
impl ButtonStyle<BinaryColor> for PrimaryButtonStyle {
    type Disabled = PrimaryButtonDisabled;
    type Idle = PrimaryButtonIdle;
    type Hovered = PrimaryButtonHovered;
    type Pressed = PrimaryButtonPressed;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
}

// endregion

// region: Secondary button

pub struct SecondaryButtonDisabled;
pub struct SecondaryButtonIdle;
pub struct SecondaryButtonHovered;
pub struct SecondaryButtonPressed;

impl ButtonStateColors<BinaryColor> for SecondaryButtonDisabled {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for SecondaryButtonIdle {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for SecondaryButtonHovered {
    const LABEL_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl ButtonStateColors<BinaryColor> for SecondaryButtonPressed {
    const LABEL_COLOR: BinaryColor = BinaryColor::Off;
    const BORDER_COLOR: BinaryColor = BinaryColor::Off;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::On;
}

pub struct SecondaryButtonStyle;
impl ButtonStyle<BinaryColor> for SecondaryButtonStyle {
    type Disabled = SecondaryButtonDisabled;
    type Idle = SecondaryButtonIdle;
    type Hovered = SecondaryButtonHovered;
    type Pressed = SecondaryButtonPressed;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
}

// endregion

impl Theme for BinaryColor {
    const TEXT_COLOR: BinaryColor = BinaryColor::On;
    const BORDER_COLOR: BinaryColor = BinaryColor::On;
    const BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
}

impl DefaultTheme for BinaryColor {
    type PrimaryButton = PrimaryButtonStyle;
    type SecondaryButton = SecondaryButtonStyle;
}
