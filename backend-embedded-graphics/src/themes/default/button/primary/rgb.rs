use core::marker::PhantomData;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    prelude::WebColors,
};

use crate::themes::default::button::{ButtonStateColors, ButtonStyle};

pub struct PrimaryButtonInactive<C>(PhantomData<C>);
pub struct PrimaryButtonIdle<C>(PhantomData<C>);
pub struct PrimaryButtonHovered<C>(PhantomData<C>);
pub struct PrimaryButtonPressed<C>(PhantomData<C>);

impl<C> ButtonStateColors<C> for PrimaryButtonInactive<C>
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
    C: WebColors,
{
    type Inactive = PrimaryButtonInactive<C>;
    type Idle = PrimaryButtonIdle<C>;
    type Hovered = PrimaryButtonHovered<C>;
    type Pressed = PrimaryButtonPressed<C>;

    const FONT: MonoFont<'static, 'static> = FONT_6X10;
}
