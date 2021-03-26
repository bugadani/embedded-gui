use core::marker::PhantomData;

use embedded_graphics::{
    mono_font::ascii::Font6x10,
    pixelcolor::{Rgb555, Rgb565, Rgb888, RgbColor, WebColors},
};

use crate::themes::{
    default::{ButtonStateColors, ButtonStyle, DefaultTheme},
    Theme,
};

// region: Primary button

pub struct PrimaryButtonIdle<C>(PhantomData<C>);
pub struct PrimaryButtonHovered<C>(PhantomData<C>);
pub struct PrimaryButtonPressed<C>(PhantomData<C>);

impl<C> ButtonStateColors<C> for PrimaryButtonIdle<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_STEEL_BLUE;
    const BACKGROUND_COLOR: C = C::CSS_STEEL_BLUE;
}

impl<C> ButtonStateColors<C> for PrimaryButtonHovered<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_DODGER_BLUE;
    const BACKGROUND_COLOR: C = C::CSS_DODGER_BLUE;
}

impl<C> ButtonStateColors<C> for PrimaryButtonPressed<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_LIGHT_STEEL_BLUE;
    const BACKGROUND_COLOR: C = C::CSS_LIGHT_STEEL_BLUE;
}

pub struct PrimaryButtonStyle<C>(PhantomData<C>);
impl<C> ButtonStyle<C> for PrimaryButtonStyle<C>
where
    C: WebColors + Theme,
{
    type Font = Font6x10;

    type Idle = PrimaryButtonIdle<C>;
    type Hovered = PrimaryButtonHovered<C>;
    type Pressed = PrimaryButtonPressed<C>;

    fn font() -> Self::Font {
        Font6x10
    }
}

// endregion

// region: Secondary button

pub struct SecondaryButtonIdle<C>(PhantomData<C>);
pub struct SecondaryButtonHovered<C>(PhantomData<C>);
pub struct SecondaryButtonPressed<C>(PhantomData<C>);

impl<C> ButtonStateColors<C> for SecondaryButtonIdle<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_SLATE_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_SLATE_GRAY;
}

impl<C> ButtonStateColors<C> for SecondaryButtonHovered<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_LIGHT_SLATE_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_LIGHT_SLATE_GRAY;
}

impl<C> ButtonStateColors<C> for SecondaryButtonPressed<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_STEEL_BLUE;
    const BACKGROUND_COLOR: C = C::CSS_STEEL_BLUE;
}

pub struct SecondaryButtonStyle<C>(PhantomData<C>);
impl<C> ButtonStyle<C> for SecondaryButtonStyle<C>
where
    C: WebColors,
{
    type Font = Font6x10;

    type Idle = SecondaryButtonIdle<C>;
    type Hovered = SecondaryButtonHovered<C>;
    type Pressed = SecondaryButtonPressed<C>;

    fn font() -> Self::Font {
        Font6x10
    }
}

// endregion

impl Theme for Rgb888 {
    const TEXT_COLOR: Self = Self::WHITE;
    const BORDER_COLOR: Self = Self::WHITE;
    const BACKGROUND_COLOR: Self = Self::BLACK;
}

impl DefaultTheme for Rgb888 {
    type PrimaryButton = PrimaryButtonStyle<Self>;
    type SecondaryButton = SecondaryButtonStyle<Self>;
}

impl Theme for Rgb555 {
    const TEXT_COLOR: Self = Self::WHITE;
    const BORDER_COLOR: Self = Self::WHITE;
    const BACKGROUND_COLOR: Self = Self::BLACK;
}

impl DefaultTheme for Rgb555 {
    type PrimaryButton = PrimaryButtonStyle<Self>;
    type SecondaryButton = SecondaryButtonStyle<Self>;
}

impl Theme for Rgb565 {
    const TEXT_COLOR: Self = Self::WHITE;
    const BORDER_COLOR: Self = Self::WHITE;
    const BACKGROUND_COLOR: Self = Self::BLACK;
}

impl DefaultTheme for Rgb565 {
    type PrimaryButton = PrimaryButtonStyle<Self>;
    type SecondaryButton = SecondaryButtonStyle<Self>;
}
