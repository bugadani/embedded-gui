use core::marker::PhantomData;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    pixelcolor::{Rgb555, Rgb565, Rgb888, RgbColor, WebColors},
};

use crate::themes::{
    default::{
        ButtonStateColors, ButtonStyle, CheckBoxStateColors, CheckBoxVisualStyle, DefaultTheme,
        RadioButtonStateColors, RadioButtonVisualStyle,
    },
    Theme,
};

// region: Primary button

pub struct PrimaryButtonDisabled<C>(PhantomData<C>);
pub struct PrimaryButtonIdle<C>(PhantomData<C>);
pub struct PrimaryButtonHovered<C>(PhantomData<C>);
pub struct PrimaryButtonPressed<C>(PhantomData<C>);

impl<C> ButtonStateColors<C> for PrimaryButtonDisabled<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::CSS_LIGHT_GRAY;
    const BORDER_COLOR: C = C::CSS_DIM_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_DIM_GRAY;
}

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
    type Disabled = PrimaryButtonDisabled<C>;
    type Idle = PrimaryButtonIdle<C>;
    type Hovered = PrimaryButtonHovered<C>;
    type Pressed = PrimaryButtonPressed<C>;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
}

// endregion

// region: Secondary button

pub struct SecondaryButtonDisabled<C>(PhantomData<C>);
pub struct SecondaryButtonIdle<C>(PhantomData<C>);
pub struct SecondaryButtonHovered<C>(PhantomData<C>);
pub struct SecondaryButtonPressed<C>(PhantomData<C>);

impl<C> ButtonStateColors<C> for SecondaryButtonDisabled<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::CSS_LIGHT_GRAY;
    const BORDER_COLOR: C = C::CSS_DIM_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_DIM_GRAY;
}

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
    type Disabled = SecondaryButtonDisabled<C>;
    type Idle = SecondaryButtonIdle<C>;
    type Hovered = SecondaryButtonHovered<C>;
    type Pressed = SecondaryButtonPressed<C>;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
}

// endregion

// region: CheckBox
pub struct CheckBoxDisabled;
pub struct CheckBoxIdle;
pub struct CheckBoxHovered;
pub struct CheckBoxPressed;

impl<C> CheckBoxStateColors<C> for CheckBoxDisabled
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::CSS_GRAY;
    const BORDER_COLOR: C = C::CSS_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_DARK_GRAY;
    const CHECK_MARK_COLOR: C = C::CSS_STEEL_BLUE;
}

impl<C> CheckBoxStateColors<C> for CheckBoxIdle
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::BLACK;
    const BORDER_COLOR: C = C::BLACK;
    const BACKGROUND_COLOR: C = C::WHITE;
    const CHECK_MARK_COLOR: C = C::CSS_DODGER_BLUE;
}

impl<C> CheckBoxStateColors<C> for CheckBoxHovered
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::BLACK;
    const BORDER_COLOR: C = C::BLACK;
    const BACKGROUND_COLOR: C = C::CSS_LIGHT_GRAY;
    const CHECK_MARK_COLOR: C = C::CSS_DODGER_BLUE;
}

impl<C> CheckBoxStateColors<C> for CheckBoxPressed
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::BLACK;
    const BORDER_COLOR: C = C::BLACK;
    const BACKGROUND_COLOR: C = C::CSS_DARK_GRAY;
    const CHECK_MARK_COLOR: C = C::CSS_DODGER_BLUE;
}

pub struct CheckBoxStyle<C>(PhantomData<C>);
impl<C> CheckBoxVisualStyle<C> for CheckBoxStyle<C>
where
    C: WebColors,
{
    type Disabled = CheckBoxDisabled;
    type Idle = CheckBoxIdle;
    type Hovered = CheckBoxHovered;
    type Pressed = CheckBoxPressed;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
}
// endregion

// region: RadioButton
pub struct RadioButtonDisabled;
pub struct RadioButtonIdle;
pub struct RadioButtonHovered;
pub struct RadioButtonPressed;

impl<C> RadioButtonStateColors<C> for RadioButtonDisabled
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::CSS_GRAY;
    const BORDER_COLOR: C = C::CSS_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_DARK_GRAY;
    const CHECK_MARK_COLOR: C = C::CSS_STEEL_BLUE;
}

impl<C> RadioButtonStateColors<C> for RadioButtonIdle
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::BLACK;
    const BORDER_COLOR: C = C::BLACK;
    const BACKGROUND_COLOR: C = C::WHITE;
    const CHECK_MARK_COLOR: C = C::CSS_DODGER_BLUE;
}

impl<C> RadioButtonStateColors<C> for RadioButtonHovered
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::BLACK;
    const BORDER_COLOR: C = C::BLACK;
    const BACKGROUND_COLOR: C = C::CSS_LIGHT_GRAY;
    const CHECK_MARK_COLOR: C = C::CSS_DODGER_BLUE;
}

impl<C> RadioButtonStateColors<C> for RadioButtonPressed
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::BLACK;
    const BORDER_COLOR: C = C::BLACK;
    const BACKGROUND_COLOR: C = C::CSS_DARK_GRAY;
    const CHECK_MARK_COLOR: C = C::CSS_DODGER_BLUE;
}

pub struct RadioButtonStyle<C>(PhantomData<C>);
impl<C> RadioButtonVisualStyle<C> for RadioButtonStyle<C>
where
    C: WebColors,
{
    type Disabled = RadioButtonDisabled;
    type Idle = RadioButtonIdle;
    type Hovered = RadioButtonHovered;
    type Pressed = RadioButtonPressed;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
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

    type CheckBox = CheckBoxStyle<Self>;
    type RadioButton = RadioButtonStyle<Self>;
}

impl Theme for Rgb555 {
    const TEXT_COLOR: Self = Self::WHITE;
    const BORDER_COLOR: Self = Self::WHITE;
    const BACKGROUND_COLOR: Self = Self::BLACK;
}

impl DefaultTheme for Rgb555 {
    type PrimaryButton = PrimaryButtonStyle<Self>;
    type SecondaryButton = SecondaryButtonStyle<Self>;

    type CheckBox = CheckBoxStyle<Self>;
    type RadioButton = RadioButtonStyle<Self>;
}

impl Theme for Rgb565 {
    const TEXT_COLOR: Self = Self::WHITE;
    const BORDER_COLOR: Self = Self::WHITE;
    const BACKGROUND_COLOR: Self = Self::BLACK;
}

impl DefaultTheme for Rgb565 {
    type PrimaryButton = PrimaryButtonStyle<Self>;
    type SecondaryButton = SecondaryButtonStyle<Self>;

    type CheckBox = CheckBoxStyle<Self>;
    type RadioButton = RadioButtonStyle<Self>;
}
