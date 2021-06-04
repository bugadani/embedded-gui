use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::PixelColor,
    prelude::Point,
    text::{renderer::TextRenderer, Baseline},
};
use embedded_gui::{
    geometry::MeasuredSize,
    widgets::label::{Label, LabelProperties},
    WidgetRenderer,
};

use crate::EgCanvas;

pub struct LabelStyle<T>
where
    T: TextRenderer,
{
    renderer: T,
}

impl<'a, C> LabelStyle<MonoTextStyle<'a, C>>
where
    C: PixelColor,
{
    /// Customize the text color
    pub fn text_color(&mut self, text_color: C) {
        self.renderer = MonoTextStyleBuilder::from(&self.renderer)
            .text_color(text_color)
            .build();
    }

    /// Customize the font
    pub fn font<'a2>(self, font: &'a2 MonoFont<'a2>) -> LabelStyle<MonoTextStyle<'a2, C>> {
        LabelStyle {
            renderer: MonoTextStyleBuilder::from(&self.renderer)
                .font(font)
                .build(),
        }
    }
}

impl<F, C> LabelProperties for LabelStyle<F>
where
    F: TextRenderer<Color = C>,
    C: PixelColor,
{
    fn measure_text(&self, text: &str) -> MeasuredSize {
        let metrics = self.renderer.measure_string(
            text,
            Point::zero(),
            embedded_graphics::text::Baseline::Top,
        );

        MeasuredSize {
            width: metrics.bounding_box.size.width,
            height: metrics.bounding_box.size.height,
        }
    }
}

pub trait LabelStyling<S>: Sized {
    type Color;

    fn text_color(mut self, color: Self::Color) -> Self {
        self.set_text_color(color);
        self
    }

    fn set_text_color(&mut self, color: Self::Color);

    fn text_renderer<T: TextRenderer>(self, renderer: T) -> Label<S, LabelStyle<T>>;

    fn style<P>(self, props: P) -> Label<S, P>
    where
        P: LabelProperties;
}

impl<'a, C, S> LabelStyling<S> for Label<S, LabelStyle<MonoTextStyle<'a, C>>>
where
    S: AsRef<str>,
    C: PixelColor,
{
    type Color = C;

    fn set_text_color(&mut self, color: Self::Color) {
        self.label_properties.text_color(color);
    }

    fn text_renderer<T: TextRenderer>(self, renderer: T) -> Label<S, LabelStyle<T>> {
        self.style(LabelStyle { renderer })
    }

    fn style<P>(self, props: P) -> Label<S, P>
    where
        P: LabelProperties,
    {
        Label {
            parent_index: self.parent_index,
            text: self.text,
            bounds: self.bounds,
            label_properties: props,
            on_state_changed: |_, _| (),
        }
    }
}

/// Font settings specific to `MonoFont`'s renderer.
pub trait MonoFontLabelStyling<C, S>: Sized
where
    S: AsRef<str>,
    C: PixelColor,
{
    fn font<'a>(self, font: &'a MonoFont<'a>) -> Label<S, LabelStyle<MonoTextStyle<'a, C>>>;
}

impl<'a, C, S> MonoFontLabelStyling<C, S> for Label<S, LabelStyle<MonoTextStyle<'a, C>>>
where
    S: AsRef<str>,
    C: PixelColor,
{
    fn font<'a2>(self, font: &'a2 MonoFont<'a2>) -> Label<S, LabelStyle<MonoTextStyle<'a2, C>>> {
        let renderer = MonoTextStyleBuilder::from(&self.label_properties.renderer)
            .font(font)
            .build();

        self.style(LabelStyle { renderer })
    }
}

impl<S, F, C, DT> WidgetRenderer<EgCanvas<DT>> for Label<S, LabelStyle<F>>
where
    S: AsRef<str>,
    F: TextRenderer<Color = C>,
    C: PixelColor,
    DT: DrawTarget<Color = C>,
{
    fn draw(&self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.label_properties
            .renderer
            .draw_string(
                self.text.as_ref(),
                Point::new(self.bounds.position.x, self.bounds.position.y),
                Baseline::Top,
                &mut canvas.target,
            )
            .map(|_| ())
    }
}

macro_rules! label_for_charset {
    ($charset:ident) => {
        pub mod $charset {
            use embedded_graphics::{
                mono_font::{$charset, MonoTextStyle},
                pixelcolor::PixelColor,
            };
            use embedded_gui::{geometry::BoundingBox, widgets::label::Label};

            use crate::{themes::Theme, widgets::label::LabelStyle};

            pub trait LabelConstructor<'a, S, C>
            where
                S: AsRef<str>,
                C: PixelColor,
            {
                fn new(text: S) -> Label<S, LabelStyle<MonoTextStyle<'a, C>>>;
            }

            impl<'a, C, S> LabelConstructor<'a, S, C> for Label<S, LabelStyle<MonoTextStyle<'a, C>>>
            where
                S: AsRef<str>,
                C: PixelColor + Theme,
            {
                fn new(text: S) -> Self {
                    Label {
                        parent_index: 0,
                        text,
                        label_properties: LabelStyle {
                            renderer: MonoTextStyle::new(
                                &$charset::FONT_6X10,
                                <C as Theme>::TEXT_COLOR,
                            ),
                        },
                        bounds: BoundingBox::default(),
                        on_state_changed: |_, _| (),
                    }
                }
            }
        }
    };
}

label_for_charset!(ascii);
label_for_charset!(iso_8859_1);
label_for_charset!(iso_8859_10);
label_for_charset!(iso_8859_13);
label_for_charset!(iso_8859_14);
label_for_charset!(iso_8859_15);
label_for_charset!(iso_8859_16);
label_for_charset!(iso_8859_2);
label_for_charset!(iso_8859_3);
label_for_charset!(iso_8859_4);
label_for_charset!(iso_8859_5);
label_for_charset!(iso_8859_7);
label_for_charset!(iso_8859_9);
label_for_charset!(jis_x0201);
