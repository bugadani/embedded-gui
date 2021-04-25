use core::marker::PhantomData;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    prelude::WebColors,
};

use crate::themes::default::button::{ButtonStateColors, ButtonStyle};

pub struct SecondaryButtonInactive<C>(PhantomData<C>);
pub struct SecondaryButtonIdle<C>(PhantomData<C>);
pub struct SecondaryButtonHovered<C>(PhantomData<C>);
pub struct SecondaryButtonPressed<C>(PhantomData<C>);

impl<C> ButtonStateColors<C> for SecondaryButtonInactive<C>
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
    type Inactive = SecondaryButtonInactive<C>;
    type Idle = SecondaryButtonIdle<C>;
    type Hovered = SecondaryButtonHovered<C>;
    type Pressed = SecondaryButtonPressed<C>;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
}
