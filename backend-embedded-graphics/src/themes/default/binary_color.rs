use embedded_graphics::{mono_font::ascii::Font6x10, pixelcolor::BinaryColor};

use crate::themes::{
    default::{ButtonStateColors, ButtonStyle, DefaultTheme},
    Theme,
};

// region: Primary button

pub struct PrimaryButtonIdle;
pub struct PrimaryButtonHovered;
pub struct PrimaryButtonPressed;

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
    type Font = Font6x10;

    type Idle = PrimaryButtonIdle;
    type Hovered = PrimaryButtonHovered;
    type Pressed = PrimaryButtonPressed;

    fn font() -> Self::Font {
        Font6x10
    }
}

// endregion

// region: Secondary button

pub struct SecondaryButtonIdle;
pub struct SecondaryButtonHovered;
pub struct SecondaryButtonPressed;

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
    type Font = Font6x10;

    type Idle = SecondaryButtonIdle;
    type Hovered = SecondaryButtonHovered;
    type Pressed = SecondaryButtonPressed;

    fn font() -> Self::Font {
        Font6x10
    }
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
