use embedded_graphics::{
    mono_font::{latin1, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::PixelColor,
};
use embedded_gui::{geometry::BoundingBox, widgets::label::Label};

use crate::{themes::Theme, widgets::label::LabelStyle};

pub trait LabelConstructor<S, C> {
    fn new(text: S) -> Label<S, LabelStyle<MonoTextStyle<C, latin1::Font6x10>>>
    where
        C: PixelColor,
        S: AsRef<str>;
}

impl<C, S> LabelConstructor<S, C> for Label<S, LabelStyle<MonoTextStyle<C, latin1::Font6x10>>>
where
    S: AsRef<str>,
    C: PixelColor + Theme,
{
    fn new(text: S) -> Self {
        Label {
            parent_index: 0,
            text,
            label_properties: LabelStyle {
                renderer: MonoTextStyleBuilder::new()
                    .font(latin1::Font6x10)
                    .text_color(<C as Theme>::TEXT_COLOR)
                    .build(),
            },
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
        }
    }
}
