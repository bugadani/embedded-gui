use core::marker::PhantomData;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    pixelcolor::WebColors,
};

use crate::themes::default::check_box::{CheckBoxStateColors, CheckBoxVisualStyle};

pub struct CheckBoxInactive;
pub struct CheckBoxIdle;
pub struct CheckBoxHovered;
pub struct CheckBoxPressed;

impl<C> CheckBoxStateColors<C> for CheckBoxInactive
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
    type Inactive = CheckBoxInactive;
    type Idle = CheckBoxIdle;
    type Hovered = CheckBoxHovered;
    type Pressed = CheckBoxPressed;

    const FONT: MonoFont<'static> = FONT_6X10;
}
