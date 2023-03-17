use embedded_canvas::CCanvasAt;
use embedded_graphics::{
    prelude::{Dimensions, DrawTarget, PixelColor, Point, Size},
    Drawable,
};
use embedded_gui::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
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

use crate::{themes::Theme, EgCanvas};

pub trait CanvasProperties {
    type Color;
    type Canvas: Drawable<Color = Self::Color>;

    fn canvas(&mut self) -> &mut Self::Canvas;
    fn measure(&self) -> MeasuredSize;
    fn move_canvas(&mut self, pos: Position);
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

    fn measure(&self) -> MeasuredSize {
        MeasuredSize {
            width: W as u32,
            height: H as u32,
        }
    }

    fn move_canvas(&mut self, pos: Position) {
        self.canvas.top_left = Point { x: pos.x, y: pos.y };
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

    fn measure(&self) -> MeasuredSize {
        MeasuredSize {
            width: self.canvas.bounding_box().size.width,
            height: self.canvas.bounding_box().size.height,
        }
    }

    fn move_canvas(&mut self, pos: Position) {
        self.canvas.top_left = Point { x: pos.x, y: pos.y };
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

pub struct Canvas<P, H: FnMut(InputContext, InputEvent) -> bool> {
    pub bounds: BoundingBox,
    pub parent_index: usize,
    pub canvas_properties: P,
    pub state: WidgetState,
    handler: Option<H>,
}

impl Canvas<(), fn(InputContext, InputEvent) -> bool> {
    pub const STATE_SELECTED: Selected = Selected;
    pub const STATE_UNSELECTED: Unselected = Unselected;
}

impl<P> Canvas<P, fn(InputContext, InputEvent) -> bool> {
    pub fn new() -> Canvas<P, fn(InputContext, InputEvent) -> bool>
    where
        P: Default,
    {
        Self {
            parent_index: 0,
            bounds: BoundingBox::default(),
            canvas_properties: P::default(),
            state: WidgetState::default(),
            handler: None,
        }
    }

    pub fn with_properties(properties: P) -> Canvas<P, fn(InputContext, InputEvent) -> bool>
    where
        P: Default,
    {
        Self {
            parent_index: 0,
            bounds: BoundingBox::default(),
            canvas_properties: properties,
            state: WidgetState::default(),
            handler: None,
        }
    }
}

impl<P, H> Canvas<P, H>
where
    H: FnMut(InputContext, InputEvent) -> bool,
{
    pub fn canvas(&mut self) -> &mut P::Canvas
    where
        P: CanvasProperties,
    {
        self.canvas_properties.canvas()
    }

    pub fn with_input_handler<H2>(self, handler: H2) -> Canvas<P, H2>
    where
        H2: FnMut(InputContext, InputEvent) -> bool,
    {
        Canvas {
            parent_index: self.parent_index,
            bounds: self.bounds,
            canvas_properties: self.canvas_properties,
            state: self.state,
            handler: Some(handler),
        }
    }
}

impl<P, H> WrapperBindable for Canvas<P, H>
where
    P: CanvasProperties,
    H: FnMut(InputContext, InputEvent) -> bool,
{
}

impl<P, H> Widget for Canvas<P, H>
where
    P: CanvasProperties,
    H: FnMut(InputContext, InputEvent) -> bool,
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

            InputEvent::PointerEvent(position, PointerEvent::Drag | PointerEvent::Hover) => {
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

impl<C, DT, P, H> WidgetRenderer<EgCanvas<DT>> for Canvas<P, H>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
    P: CanvasProperties<Color = C>,
    H: FnMut(InputContext, InputEvent) -> bool,
{
    fn draw(&mut self, canvas: &mut EgCanvas<DT>) -> Result<(), DT::Error> {
        self.canvas_properties.move_canvas(self.bounds.position);
        self.canvas().draw(&mut canvas.target)?;

        Ok(())
    }
}
