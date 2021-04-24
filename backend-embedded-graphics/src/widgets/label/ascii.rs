use embedded_graphics::{
    mono_font::{ascii, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::PixelColor,
};
use embedded_gui::{geometry::BoundingBox, widgets::label::Label};

use crate::{themes::Theme, widgets::label::LabelStyle};

pub trait LabelConstructor<'a, 'b, 'c, S, C>
where
    S: AsRef<str>,
    C: PixelColor,
{
    fn new(text: S) -> Label<S, LabelStyle<MonoTextStyle<'a, 'b, 'c, C>>>;
}

impl<'a, 'b, 'c, C, S> LabelConstructor<'a, 'b, 'c, S, C>
    for Label<S, LabelStyle<MonoTextStyle<'a, 'b, 'c, C>>>
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
                    .font(&ascii::FONT_6X10)
                    .text_color(<C as Theme>::TEXT_COLOR)
                    .build(),
            },
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
        }
    }
}
