use core::marker::PhantomData;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoFont},
    pixelcolor::WebColors,
};

use crate::themes::default::toggle_button::{
    ButtonStateColors, ToggleButtonStyle as ToggleButtonStyleTrait,
};

pub struct ToggleButtonInactive<C>(PhantomData<C>);
pub struct ToggleButtonIdle<C>(PhantomData<C>);
pub struct ToggleButtonHovered<C>(PhantomData<C>);
pub struct ToggleButtonPressed<C>(PhantomData<C>);

impl<C> ButtonStateColors<C> for ToggleButtonInactive<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::CSS_LIGHT_GRAY;
    const BORDER_COLOR: C = C::CSS_DIM_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_DIM_GRAY;
}

impl<C> ButtonStateColors<C> for ToggleButtonIdle<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_SLATE_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_SLATE_GRAY;
}

impl<C> ButtonStateColors<C> for ToggleButtonHovered<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_LIGHT_SLATE_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_LIGHT_SLATE_GRAY;
}

impl<C> ButtonStateColors<C> for ToggleButtonPressed<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_STEEL_BLUE;
    const BACKGROUND_COLOR: C = C::CSS_STEEL_BLUE;
}

pub struct ToggleButtonInactiveChecked<C>(PhantomData<C>);
pub struct ToggleButtonIdleChecked<C>(PhantomData<C>);
pub struct ToggleButtonHoveredChecked<C>(PhantomData<C>);
pub struct ToggleButtonPressedChecked<C>(PhantomData<C>);

impl<C> ButtonStateColors<C> for ToggleButtonInactiveChecked<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::CSS_LIGHT_GRAY;
    const BORDER_COLOR: C = C::CSS_DIM_GRAY;
    const BACKGROUND_COLOR: C = C::CSS_DIM_GRAY;
}

impl<C> ButtonStateColors<C> for ToggleButtonIdleChecked<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_STEEL_BLUE;
    const BACKGROUND_COLOR: C = C::CSS_STEEL_BLUE;
}

impl<C> ButtonStateColors<C> for ToggleButtonHoveredChecked<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_DODGER_BLUE;
    const BACKGROUND_COLOR: C = C::CSS_DODGER_BLUE;
}

impl<C> ButtonStateColors<C> for ToggleButtonPressedChecked<C>
where
    C: WebColors,
{
    const LABEL_COLOR: C = C::WHITE;
    const BORDER_COLOR: C = C::CSS_LIGHT_STEEL_BLUE;
    const BACKGROUND_COLOR: C = C::CSS_LIGHT_STEEL_BLUE;
}

pub struct ToggleButtonStyle<C>(PhantomData<C>);
impl<C> ToggleButtonStyleTrait<C> for ToggleButtonStyle<C>
where
    C: WebColors,
{
    type Inactive = ToggleButtonInactive<C>;
    type Idle = ToggleButtonIdle<C>;
    type Hovered = ToggleButtonHovered<C>;
    type Pressed = ToggleButtonPressed<C>;

    type InactiveChecked = ToggleButtonInactiveChecked<C>;
    type IdleChecked = ToggleButtonIdleChecked<C>;
    type HoveredChecked = ToggleButtonHoveredChecked<C>;
    type PressedChecked = ToggleButtonPressedChecked<C>;

    const FONT: MonoFont<'static> = FONT_6X10;
}
