#![no_std]

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
        button::Button,
        label::{Label, LabelConstructor, LabelProperties},
        Widget, WidgetProperties,
    },
    BoundingBox, Canvas, MeasuredSize, WidgetRenderer,
};

pub struct EgCanvas<C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    pub target: D,
}

impl<C, D> EgCanvas<C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    pub fn new(target: D) -> Self {
        Self { target }
    }
}

impl<C: PixelColor, D: DrawTarget<Color = C>> Canvas for EgCanvas<C, D> {
    type Error = <D as DrawTarget>::Error;
}

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

impl<F, C, D> LabelConstructor<EgCanvas<C, D>, LabelStyle<F>>
    for Label<EgCanvas<C, D>, LabelStyle<F>>
where
    F: TextRenderer,
    C: PixelColor,
    LabelStyle<F>: Default,
    D: DrawTarget<Color = C>,
{
    fn new(text: &'static str) -> Self {
        Self {
            text,
            widget_properties: WidgetProperties::default(),
            label_properties: LabelStyle::default(),
            bounds: BoundingBox::default(),
            _marker: PhantomData,
        }
    }
}

impl<F, C, D> WidgetRenderer<EgCanvas<C, D>> for Label<EgCanvas<C, D>, LabelStyle<F>>
where
    F: TextRenderer<Color = C>,
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    fn draw(&self, canvas: &mut EgCanvas<C, D>) -> Result<(), D::Error> {
        self.label_properties
            .renderer
            .draw_string(
                self.text,
                Point::new(
                    self.bounding_box().position.x,
                    self.bounding_box().position.y,
                ),
                &mut canvas.target,
            )
            .map(|_| ())
    }
}

impl<C, D, I> WidgetRenderer<EgCanvas<C, D>> for Button<I>
where
    I: Widget + WidgetRenderer<EgCanvas<C, D>>,
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    fn draw(&self, canvas: &mut EgCanvas<C, D>) -> Result<(), D::Error> {
        self.inner.draw(canvas)
    }
}
