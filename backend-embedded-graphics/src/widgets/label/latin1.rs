use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{latin1, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::PixelColor,
};
use embedded_gui::{widgets::label::Label, BoundingBox, WidgetState};

use crate::{themes::Theme, widgets::label::LabelStyle};

pub trait LabelConstructor<S, C, D> {
    fn new(text: S) -> Label<S, LabelStyle<MonoTextStyle<D::Color, latin1::Font6x10>>>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
        S: AsRef<str>;
}

impl<C, D, S> LabelConstructor<S, C, D>
    for Label<S, LabelStyle<MonoTextStyle<D::Color, latin1::Font6x10>>>
where
    S: AsRef<str>,
    C: PixelColor + Theme,
    D: DrawTarget<Color = C>,
{
    fn new(text: S) -> Self {
        Label {
            parent_index: 0,
            text,
            label_properties: LabelStyle {
                renderer: MonoTextStyleBuilder::new()
                    .font(latin1::Font6x10)
                    .text_color(<D::Color as Theme>::TEXT_COLOR)
                    .build(),
            },
            bounds: BoundingBox::default(),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }
}
