use core::marker::PhantomData;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::Font6x10, MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, PixelColor},
    prelude::Point,
    text::TextRenderer,
};
use embedded_gui::{
    data::NoData,
    widgets::{
        label::{Label, LabelConstructor, LabelProperties},
        WidgetWrapper,
    },
    BoundingBox, MeasuredSize, WidgetRenderer,
};
use heapless::{ArrayLength, String};

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
    pub fn text_color(mut self, text_color: C) -> Self {
        self.renderer = MonoTextStyleBuilder::from(&self.renderer)
            .text_color(text_color)
            .build();
        self
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

impl<F, C, D> LabelConstructor<&'static str, EgCanvas<C, D>, LabelStyle<F>>
    for Label<&'static str, EgCanvas<C, D>, LabelStyle<F>>
where
    F: TextRenderer,
    C: PixelColor,
    LabelStyle<F>: Default,
    D: DrawTarget<Color = C>,
{
    fn new(text: &'static str) -> WidgetWrapper<Self, NoData> {
        WidgetWrapper::new(Label {
            text,
            label_properties: LabelStyle::default(),
            bounds: BoundingBox::default(),
            _marker: PhantomData,
        })
    }
}

impl<F, C, D, L> LabelConstructor<String<L>, EgCanvas<C, D>, LabelStyle<F>>
    for Label<String<L>, EgCanvas<C, D>, LabelStyle<F>>
where
    L: ArrayLength<u8>,
    F: TextRenderer,
    C: PixelColor,
    LabelStyle<F>: Default,
    D: DrawTarget<Color = C>,
{
    fn new(text: String<L>) -> WidgetWrapper<Self, NoData> {
        WidgetWrapper::new(Label {
            text,
            label_properties: LabelStyle::default(),
            bounds: BoundingBox::default(),
            _marker: PhantomData,
        })
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
