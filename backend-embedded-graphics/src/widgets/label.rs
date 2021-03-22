use core::marker::PhantomData;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::Font6x10, MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, PixelColor},
    prelude::Point,
    text::TextRenderer,
};
use embedded_gui::{
    widgets::{
        label::{Label, LabelProperties},
        Container, WidgetDataHolder,
    },
    BoundingBox, MeasuredSize, WidgetRenderer,
};

use crate::EgCanvas;

pub struct LabelStyle<T>
where
    T: TextRenderer,
{
    renderer: T,
}

impl Default for LabelStyle<MonoTextStyle<BinaryColor, Font6x10>> {
    fn default() -> Self {
        Self {
            renderer: MonoTextStyleBuilder::new()
                .font(Font6x10)
                .text_color(BinaryColor::On)
                .build(),
        }
    }
}

impl<C, F> LabelStyle<MonoTextStyle<C, F>>
where
    F: MonoFont,
    C: PixelColor,
{
    /// Customize the text color
    pub fn text_color(&mut self, text_color: C) {
        self.renderer = MonoTextStyleBuilder::from(&self.renderer)
            .text_color(text_color)
            .build();
    }

    /// Customize the font
    pub fn font<F2: MonoFont>(self, font: F2) -> LabelStyle<MonoTextStyle<C, F2>> {
        LabelStyle {
            renderer: MonoTextStyleBuilder::from(&self.renderer)
                .font(font)
                .build(),
        }
    }
}

impl<F, C, D> LabelProperties<EgCanvas<C, D>> for LabelStyle<F>
where
    F: TextRenderer,
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    fn measure_text(&self, text: &str) -> MeasuredSize {
        let metrics = self.renderer.measure_string(text, Point::zero());

        MeasuredSize {
            width: metrics.bounding_box.size.width,
            height: metrics.bounding_box.size.height,
        }
    }
}

pub trait LabelConstructor<S, P, C, D> {
    fn new(text: S) -> Container<Label<S, EgCanvas<C, D>, P>>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
        S: AsRef<str>,
        P: LabelProperties<EgCanvas<C, D>>;
}

impl<F, C, D, S> LabelConstructor<S, LabelStyle<F>, C, D>
    for Label<S, EgCanvas<C, D>, LabelStyle<F>>
where
    S: AsRef<str>,
    F: TextRenderer,
    C: PixelColor,
    LabelStyle<F>: Default,
    D: DrawTarget<Color = C>,
{
    fn new(text: S) -> Container<Self> {
        Container::new(Label {
            text,
            label_properties: LabelStyle::default(),
            bounds: BoundingBox::default(),
            _marker: PhantomData,
        })
    }
}

pub trait LabelStyling<F, C, D, S>: Sized
where
    S: AsRef<str>,
    F: MonoFont,
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    type Color;
    type Font;

    fn text_color(self, color: Self::Color) -> Self;

    fn font<F2: MonoFont>(
        self,
        font: F2,
    ) -> Container<Label<S, EgCanvas<C, D>, LabelStyle<MonoTextStyle<C, F2>>>>;
}

impl<F, C, D, S> LabelStyling<F, C, D, S>
    for Container<Label<S, EgCanvas<C, D>, LabelStyle<MonoTextStyle<C, F>>>>
where
    S: AsRef<str>,
    F: MonoFont,
    C: PixelColor,
    LabelStyle<MonoTextStyle<C, F>>: Default,
    D: DrawTarget<Color = C>,
{
    type Color = C;
    type Font = F;

    fn text_color(mut self, color: Self::Color) -> Self {
        self.widget.label_properties.text_color(color);
        self
    }

    fn font<F2: MonoFont>(
        self,
        font: F2,
    ) -> Container<Label<S, EgCanvas<C, D>, LabelStyle<MonoTextStyle<C, F2>>>> {
        let label_properties = self.widget.label_properties.font(font);

        Container {
            parent_index: self.parent_index,
            widget: Label {
                text: self.widget.text,
                bounds: self.widget.bounds,
                label_properties,
                _marker: PhantomData,
            },
            data_holder: WidgetDataHolder::default(),
            state: self.state,
            on_state_changed: |_, _| (),
        }
    }
}

impl<S, F, C, DT> WidgetRenderer<EgCanvas<C, DT>> for Label<S, EgCanvas<C, DT>, LabelStyle<F>>
where
    S: AsRef<str>,
    F: TextRenderer<Color = C>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.label_properties
            .renderer
            .draw_string(
                self.text.as_ref(),
                Point::new(self.bounds.position.x, self.bounds.position.y),
                &mut canvas.target,
            )
            .map(|_| ())
    }
}
