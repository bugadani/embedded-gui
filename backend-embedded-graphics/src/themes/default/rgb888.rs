use embedded_graphics::{
    mono_font::ascii::Font6x10,
    pixelcolor::{Rgb888, RgbColor, WebColors},
};

use crate::themes::{
    default::{ButtonStateColors, ButtonStyle, DefaultTheme},
    Theme,
};

// region: Primary button

pub struct PrimaryButtonIdle;
pub struct PrimaryButtonHovered;
pub struct PrimaryButtonPressed;

impl ButtonStateColors<Rgb888> for PrimaryButtonIdle {
    const LABEL_COLOR: Rgb888 = Rgb888::WHITE;
    const BORDER_COLOR: Rgb888 = Rgb888::new(0, 120, 215);
    const BACKGROUND_COLOR: Rgb888 = Rgb888::new(0, 120, 215);
}

impl ButtonStateColors<Rgb888> for PrimaryButtonHovered {
    const LABEL_COLOR: Rgb888 = Rgb888::WHITE;
    const BORDER_COLOR: Rgb888 = Rgb888::CSS_DODGER_BLUE;
    const BACKGROUND_COLOR: Rgb888 = Rgb888::CSS_DODGER_BLUE;
}

impl ButtonStateColors<Rgb888> for PrimaryButtonPressed {
    const LABEL_COLOR: Rgb888 = Rgb888::WHITE;
    const BORDER_COLOR: Rgb888 = Rgb888::CSS_STEEL_BLUE;
    const BACKGROUND_COLOR: Rgb888 = Rgb888::CSS_STEEL_BLUE;
}

pub struct PrimaryButtonStyle;
impl ButtonStyle<Rgb888> for PrimaryButtonStyle {
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

impl ButtonStateColors<Rgb888> for SecondaryButtonIdle {
    const LABEL_COLOR: Rgb888 = Rgb888::WHITE;
    const BORDER_COLOR: Rgb888 = Rgb888::CSS_SLATE_GRAY;
    const BACKGROUND_COLOR: Rgb888 = Rgb888::CSS_SLATE_GRAY;
}

impl ButtonStateColors<Rgb888> for SecondaryButtonHovered {
    const LABEL_COLOR: Rgb888 = Rgb888::WHITE;
    const BORDER_COLOR: Rgb888 = Rgb888::CSS_LIGHT_SLATE_GRAY;
    const BACKGROUND_COLOR: Rgb888 = Rgb888::CSS_LIGHT_SLATE_GRAY;
}

impl ButtonStateColors<Rgb888> for SecondaryButtonPressed {
    const LABEL_COLOR: Rgb888 = Rgb888::WHITE;
    const BORDER_COLOR: Rgb888 = Rgb888::CSS_STEEL_BLUE;
    const BACKGROUND_COLOR: Rgb888 = Rgb888::CSS_STEEL_BLUE;
}

pub struct SecondaryButtonStyle;
impl ButtonStyle<Rgb888> for SecondaryButtonStyle {
    type Font = Font6x10;

    type Idle = SecondaryButtonIdle;
    type Hovered = SecondaryButtonHovered;
    type Pressed = SecondaryButtonPressed;

    fn font() -> Self::Font {
        Font6x10
    }
}

// endregion

impl Theme for Rgb888 {
    const TEXT_COLOR: Rgb888 = Rgb888::WHITE;
    const BORDER_COLOR: Rgb888 = Rgb888::WHITE;
    const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
}

impl DefaultTheme for Rgb888 {
    type PrimaryButton = PrimaryButtonStyle;
    type SecondaryButton = SecondaryButtonStyle;
}
