use embedded_graphics::{
    mono_font::{latin1, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::PixelColor,
};
use embedded_gui::{geometry::BoundingBox, widgets::textbox::TextBox};

use crate::{themes::Theme, widgets::textbox::TextBoxStyle};

pub trait TextBoxConstructor<'a, 'b, 'c, S, C>
where
    S: AsRef<str>,
    C: PixelColor,
{
    fn new(text: S) -> TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>>>;
}

impl<'a, 'b, 'c, C, S> TextBoxConstructor<'a, 'b, 'c, S, C>
    for TextBox<S, TextBoxStyle<MonoTextStyle<'a, 'b, 'c, C>>>
where
    S: AsRef<str>,
    C: PixelColor + Theme,
{
    fn new(text: S) -> Self {
        TextBox {
            parent_index: 0,
            text,
            label_properties: TextBoxStyle {
                renderer: MonoTextStyleBuilder::new()
                    .font(&latin1::FONT_6X10)
                    .text_color(<C as Theme>::TEXT_COLOR)
                    .build(),
            },
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
        }
    }
}
