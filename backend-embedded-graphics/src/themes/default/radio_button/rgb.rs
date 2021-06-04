use core::marker::PhantomData;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    pixelcolor::WebColors,
};

use crate::themes::default::{radio_button::RadioButtonStateColors, RadioButtonVisualStyle};

pub struct RadioButtonInactive;
pub struct RadioButtonIdle;
pub struct RadioButtonHovered;
pub struct RadioButtonPressed;

impl<C> RadioButtonStateColors<C> for RadioButtonInactive
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
    type Inactive = RadioButtonInactive;
    type Idle = RadioButtonIdle;
    type Hovered = RadioButtonHovered;
    type Pressed = RadioButtonPressed;

    const FONT: MonoFont<'static> = FONT_6X10;
}
