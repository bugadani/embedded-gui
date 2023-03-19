use embedded_canvas::CCanvasAt;
use embedded_graphics::{
    draw_target::Cropped,
    prelude::{Dimensions, DrawTarget, DrawTargetExt, PixelColor, Point, Size},
    Drawable,
};
use embedded_gui::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize},
    input::{
        controller::InputContext,
        event::{InputEvent, PointerEvent},
    },
    prelude::WrapperBindable,
    state::{
        selection::{Selected, Unselected},
        WidgetState,
    },
    widgets::Widget,
    WidgetRenderer,
};

use crate::{themes::Theme, EgCanvas, ToRectangle};

pub trait CanvasProperties {
    type Color;
    type Canvas: DrawTarget<Color = Self::Color> + DrawTargetExt + Drawable<Color = Self::Color>;

    fn canvas(&mut self) -> &mut Self::Canvas;
    fn clear_color(&mut self) -> Self::Color;
    fn measure(&self) -> MeasuredSize;
}

pub struct CCanvasStyle<C, const W: usize, const H: usize>
where
    C: PixelColor,
{
    pub clear_color: C,
    pub canvas: CCanvasAt<C, W, H>,
}

impl<C, const W: usize, const H: usize> CanvasProperties for CCanvasStyle<C, W, H>
where
    C: PixelColor,
{
    type Color = C;
    type Canvas = CCanvasAt<C, W, H>;

    fn canvas(&mut self) -> &mut Self::Canvas {
        &mut self.canvas
    }

    fn clear_color(&mut self) -> Self::Color {
        self.clear_color
    }

    fn measure(&self) -> MeasuredSize {
        MeasuredSize {
            width: W as u32,
            height: H as u32,
        }
    }
}

impl<C, const W: usize, const H: usize> Default for CCanvasStyle<C, W, H>
where
    C: Theme,
{
    fn default() -> Self {
        Self {
            clear_color: C::BACKGROUND_COLOR,
            canvas: CCanvasAt::new(Point::zero()),
        }
    }
}

//#[cfg(feature = "std")]
use embedded_canvas::CanvasAt;

//#[cfg(feature = "std")]
pub struct CanvasStyle<C>
where
    C: PixelColor,
{
    pub clear_color: C,
    pub canvas: CanvasAt<C>,
}

//#[cfg(feature = "std")]
impl<C> CanvasStyle<C>
where
    C: Theme,
{
    pub fn new(size: Size) -> Self {
        Self {
            clear_color: C::BACKGROUND_COLOR,
            canvas: CanvasAt::new(Point::zero(), size),
        }
    }
}

//#[cfg(feature = "std")]
impl<C> CanvasProperties for CanvasStyle<C>
where
    C: PixelColor,
{
    type Color = C;
    type Canvas = CanvasAt<C>;

    fn canvas(&mut self) -> &mut Self::Canvas {
        &mut self.canvas
    }

    fn clear_color(&mut self) -> Self::Color {
        self.clear_color
    }

    fn measure(&self) -> MeasuredSize {
        MeasuredSize {
            width: self.canvas.bounding_box().size.width,
            height: self.canvas.bounding_box().size.height,
        }
    }
}

//#[cfg(feature = "std")]
impl<C> Default for CanvasStyle<C>
where
    C: Theme,
{
    fn default() -> Self {
        Self {
            clear_color: C::BACKGROUND_COLOR,
            canvas: CanvasAt::new(Point::zero(), Size::zero()),
        }
    }
}

pub struct Canvas<P, H, D> {
    pub bounds: BoundingBox,
    pub parent_index: usize,
    pub canvas_properties: P,
    pub state: WidgetState,
    handler: Option<H>,
    on_draw: D,
    draw: bool,
}

impl Canvas<(), (), ()> {
    pub const STATE_SELECTED: Selected = Selected;
    pub const STATE_UNSELECTED: Unselected = Unselected;
}

impl<P> Canvas<P, (), ()>
where
    P: CanvasProperties,
{
    pub fn new() -> Canvas<P, fn(InputContext, InputEvent) -> bool, fn(&mut Cropped<'_, P::Canvas>)>
    where
        P: Default,
    {
        Canvas {
            parent_index: 0,
            bounds: BoundingBox::default(),
            canvas_properties: P::default(),
            state: WidgetState::default(),
            handler: None,
            on_draw: |_| {},
            draw: true,
        }
    }

    pub fn with_properties(
        properties: P,
    ) -> Canvas<P, fn(InputContext, InputEvent) -> bool, fn(&mut Cropped<'_, P::Canvas>)>
    where
        P: Default,
    {
        Canvas {
            parent_index: 0,
            bounds: BoundingBox::default(),
            canvas_properties: properties,
            state: WidgetState::default(),
            handler: None,
            on_draw: |_| {},
            draw: true,
        }
    }
}

impl<P, H, D> Canvas<P, H, D>
where
    P: CanvasProperties,
    H: FnMut(InputContext, InputEvent) -> bool,
    D: FnMut(&mut Cropped<'_, P::Canvas>),
{
    pub fn invalidate(&mut self) {
        self.draw = true;
    }

    pub fn with_input_handler<H2>(self, handler: H2) -> Canvas<P, H2, D>
    where
        H2: FnMut(InputContext, InputEvent) -> bool,
    {
        Canvas {
            parent_index: self.parent_index,
            bounds: self.bounds,
            canvas_properties: self.canvas_properties,
            state: self.state,
            handler: Some(handler),
            on_draw: self.on_draw,
            draw: self.draw,
        }
    }

    pub fn with_on_draw<D2>(self, on_draw: D2) -> Canvas<P, H, D2>
    where
        P: CanvasProperties,
        D2: FnMut(&mut Cropped<'_, P::Canvas>),
    {
        Canvas {
            parent_index: self.parent_index,
            bounds: self.bounds,
            canvas_properties: self.canvas_properties,
            state: self.state,
            handler: self.handler,
            on_draw,
            draw: self.draw,
        }
    }
}

impl<P, H, D> WrapperBindable for Canvas<P, H, D>
where
    P: CanvasProperties,
    H: FnMut(InputContext, InputEvent) -> bool,
    D: FnMut(&mut Cropped<'_, P::Canvas>),
{
}

impl<P, H, D> Widget for Canvas<P, H, D>
where
    P: CanvasProperties,
    H: FnMut(InputContext, InputEvent) -> bool,
    D: FnMut(&mut Cropped<'_, P::Canvas>),
{
    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let canvas_size = self.canvas_properties.measure();

        let width = measure_spec.width.apply_to_measured(canvas_size.width);
        let height = measure_spec.height.apply_to_measured(canvas_size.height);

        self.bounds.size = MeasuredSize { width, height };
    }

    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }

    fn on_state_changed(&mut self, _: WidgetState) {}

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        let bounds = self.bounding_box();

        let state = &mut self.state;
        self.handler.as_mut().and_then(|_| match event {
            InputEvent::Cancel => {
                state.set_state(Canvas::STATE_UNSELECTED);
                None
            }

            InputEvent::PointerEvent(position, PointerEvent::Down) => {
                if bounds.contains(position) {
                    Some(0)
                } else {
                    // Allow a potentially clicked widget to handle the event.
                    state.set_state(Canvas::STATE_UNSELECTED);
                    None
                }
            }

            // We want controls drawn above the Canvas to get input events.
            InputEvent::PointerEvent(_, PointerEvent::Hover) => None,

            InputEvent::PointerEvent(position, PointerEvent::Drag) => {
                if bounds.contains(position) {
                    Some(0)
                } else {
                    None
                }
            }

            InputEvent::KeyEvent(_) => {
                if state.has_state(Canvas::STATE_SELECTED) {
                    Some(0)
                } else {
                    None
                }
            }

            _ => Some(0),
        })
    }

    fn handle_input(&mut self, ctxt: InputContext, event: InputEvent) -> bool {
        let state = &mut self.state;
        self.handler
            .as_mut()
            .map(|handler| {
                match event {
                    InputEvent::Cancel => {
                        state.set_state(Canvas::STATE_UNSELECTED);
                    }
                    InputEvent::PointerEvent(_, PointerEvent::Down) => {
                        state.set_state(Canvas::STATE_SELECTED);
                    }
                    _ => {}
                }

                handler(ctxt, event)
            })
            .unwrap_or(false)
    }

    fn is_selectable(&self) -> bool {
        self.handler.is_some()
    }
}

impl<C, DT, P, H, D> WidgetRenderer<EgCanvas<DT>> for Canvas<P, H, D>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    P: CanvasProperties<Color = C>,
    H: FnMut(InputContext, InputEvent) -> bool,
    D: FnMut(&mut Cropped<'_, P::Canvas>),
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        if self.draw {
            self.draw = false;
            let bounds = self.bounding_box().to_rectangle();
            let clear_color = self.canvas_properties.clear_color();
            let mut canvas = self.canvas_properties.canvas().cropped(&bounds);

            _ = canvas.clear(clear_color);

            (self.on_draw)(&mut canvas);
        }

        let bounds = self.bounding_box().to_rectangle();
        self.canvas_properties
            .canvas()
            .draw(&mut canvas.target.clipped(&bounds))?;

        Ok(())
    }
}
