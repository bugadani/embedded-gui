#![no_std]

use core::marker::PhantomData;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::Font6x10, MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, PixelColor},
    prelude::{Point, Primitive, Size},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::TextRenderer,
    Drawable,
};
use embedded_gui::{
    data::{NoData, WidgetData},
    widgets::{
        border::{Border, BorderProperties},
        button::Button,
        fill::{FillDirection, FillParent},
        label::{Label, LabelConstructor, LabelProperties},
        spacing::Spacing,
        Widget, WidgetDataHolder, WidgetWrapper,
    },
    BoundingBox, Canvas, MeasuredSize, WidgetRenderer,
};

trait ToRectangle {
    fn to_rectangle(self) -> Rectangle;
}

impl ToRectangle for BoundingBox {
    fn to_rectangle(self) -> Rectangle {
        let top_left = Point::new(self.position.x, self.position.y);
        let size = Size::new(self.size.width, self.size.height);

        Rectangle::new(top_left, size)
    }
}

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

    fn size(&self) -> MeasuredSize {
        let size = self.target.bounding_box().size;

        MeasuredSize {
            width: size.width,
            height: size.height,
        }
    }
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
    for Label<EgCanvas<C, D>, LabelStyle<F>, NoData>
where
    F: TextRenderer,
    C: PixelColor,
    LabelStyle<F>: Default,
    D: DrawTarget<Color = C>,
{
    fn new(text: &'static str) -> WidgetWrapper<Self, NoData> {
        WidgetWrapper {
            widget: Label {
                text,
                label_properties: LabelStyle::default(),
                bounds: BoundingBox::default(),
                _marker: PhantomData,
            },
            data_holder: WidgetDataHolder::default(),
        }
    }
}

pub struct BorderStyle<W, C>
where
    W: Widget,
    C: PixelColor,
{
    style: PrimitiveStyle<C>,
    _marker: PhantomData<W>,
}

impl<W> Default for BorderStyle<W, BinaryColor>
where
    W: Widget,
{
    fn default() -> Self {
        Self {
            style: PrimitiveStyleBuilder::new()
                .stroke_alignment(StrokeAlignment::Inside)
                .stroke_color(BinaryColor::On)
                .stroke_width(1)
                .build(),
            _marker: PhantomData,
        }
    }
}

impl<W, C> BorderProperties for BorderStyle<W, C>
where
    W: Widget,
    C: PixelColor,
{
    fn get_border_width(&self) -> u32 {
        self.style.stroke_width
    }
}

impl<F, C, DT, D> WidgetRenderer<EgCanvas<C, DT>> for Label<EgCanvas<C, DT>, LabelStyle<F>, D>
where
    F: TextRenderer<Color = C>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.label_properties
            .renderer
            .draw_string(
                self.text,
                Point::new(self.bounds.position.x, self.bounds.position.y),
                &mut canvas.target,
            )
            .map(|_| ())
    }
}

// TODO: draw target should be clipped to widget's bounds, so this can be restored to Border
impl<W, C, DT, D> WidgetRenderer<EgCanvas<C, DT>>
    for WidgetWrapper<Border<W, BorderStyle<W, C>, D>, D>
where
    W: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
    BorderStyle<W, C>: BorderProperties,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        let bounds = self.bounding_box();
        let border = bounds
            .to_rectangle()
            .into_styled(self.widget.border_properties.style);

        self.widget.inner.draw(canvas)?;
        border.draw(&mut canvas.target)
    }
}

impl<C, DT, I, D> WidgetRenderer<EgCanvas<C, DT>> for Button<I, D>
where
    I: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.inner.draw(canvas)
    }
}

impl<C, DT, I, D> WidgetRenderer<EgCanvas<C, DT>> for Spacing<I, D>
where
    I: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    D: WidgetData,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.inner.draw(canvas)
    }
}

impl<C, DT, W, FD> WidgetRenderer<EgCanvas<C, DT>> for FillParent<W, FD>
where
    FD: FillDirection,
    W: Widget + WidgetRenderer<EgCanvas<C, DT>>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
{
    fn draw(&self, canvas: &mut EgCanvas<C, DT>) -> Result<(), DT::Error> {
        self.inner.draw(canvas)
    }
}
