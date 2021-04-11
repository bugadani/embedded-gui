use crate::{
    data::{NoData, WidgetData},
    input::{
        controller::InputContext,
        event::{InputEvent, PointerEvent, ScrollEvent},
    },
    widgets::{ParentHolder, UpdateHandler, Widget, WidgetDataHolder, WidgetStateHolder},
    BoundingBox, MeasureConstraint, MeasureSpec, MeasuredSize, Position, PositionDelta,
    WidgetState,
};

pub struct ScrollData {
    /// The current scroll position.
    pub offset: u32,

    /// The largest possible value of `offset`.
    pub maximum_offset: u32,

    /// The Scroll widget's main size (height of Vertical, width of Horizontal).
    pub viewport_size: u32,
}

pub trait ScrollDirection {
    fn offset(&self) -> PositionDelta;
    fn change_offset(&mut self, delta: PositionDelta);
    fn override_offset(&mut self, offset: PositionDelta);
    fn scroll_direction<T>(x: T, y: T) -> T;
    fn cross_direction<T>(x: T, y: T) -> T;
    fn merge_directions<T>(main: T, cross: T) -> (T, T);
}

pub struct Horizontal {
    offset: u32,
}

impl ScrollDirection for Horizontal {
    fn offset(&self) -> PositionDelta {
        PositionDelta {
            x: self.offset as i32,
            y: 0,
        }
    }

    fn change_offset(&mut self, delta: PositionDelta) {
        self.offset = (self.offset as i32 + delta.x).max(0) as u32;
    }

    fn override_offset(&mut self, offset: PositionDelta) {
        self.offset = Self::scroll_direction(offset.x, offset.y) as u32;
    }

    fn scroll_direction<T>(x: T, _y: T) -> T {
        x
    }

    fn cross_direction<T>(_x: T, y: T) -> T {
        y
    }

    fn merge_directions<T>(main: T, cross: T) -> (T, T) {
        (main, cross)
    }
}

pub struct Vertical {
    offset: u32,
}

impl ScrollDirection for Vertical {
    fn offset(&self) -> PositionDelta {
        PositionDelta {
            x: 0,
            y: self.offset as i32,
        }
    }

    fn change_offset(&mut self, delta: PositionDelta) {
        self.offset = (self.offset as i32 + delta.y).max(0) as u32;
    }

    fn override_offset(&mut self, offset: PositionDelta) {
        self.offset = Self::scroll_direction(offset.x, offset.y) as u32;
    }

    fn scroll_direction<T>(_x: T, y: T) -> T {
        y
    }

    fn cross_direction<T>(x: T, _y: T) -> T {
        x
    }

    fn merge_directions<T>(main: T, cross: T) -> (T, T) {
        (cross, main)
    }
}

// Need separation because scroll change listeners need the fields.
pub struct ScrollFields<W, SD, D> {
    pub parent_index: usize,
    pub inner: W,
    pub bounds: BoundingBox,
    pub on_state_changed: fn(&mut Self, WidgetState),
    pub direction: SD,
    pub state: WidgetState,
    pub last_pointer_pos: Option<Position>,
    pub on_scroll_changed: fn(&mut D, ScrollData),
}

impl Scroll<(), (), NoData> {
    const STATE_IDLE: u32 = 0;
    const STATE_HOVERED: u32 = 1;
}

pub struct Scroll<W, SD, D = NoData>
where
    D: WidgetData,
{
    pub fields: ScrollFields<W, SD, D::Data>,
    data_holder: WidgetDataHolder<ScrollFields<W, SD, D::Data>, D>,
}

impl<W> Scroll<W, Horizontal, NoData>
where
    W: Widget,
{
    pub fn horizontal(inner: W) -> Self {
        Scroll {
            fields: ScrollFields {
                parent_index: 0,
                inner,
                bounds: BoundingBox::default(),
                on_state_changed: |_, _| (),
                direction: Horizontal { offset: 0 },
                state: WidgetState::default(),
                last_pointer_pos: None,
                on_scroll_changed: |_, _| (),
            },
            data_holder: WidgetDataHolder::default(),
        }
    }
}

impl<W> Scroll<W, Vertical, NoData>
where
    W: Widget,
{
    pub fn vertical(inner: W) -> Self {
        Scroll {
            fields: ScrollFields {
                parent_index: 0,
                inner,
                bounds: BoundingBox::default(),
                on_state_changed: |_, _| (),
                direction: Vertical { offset: 0 },
                state: WidgetState::default(),
                last_pointer_pos: None,
                on_scroll_changed: |_, _| (),
            },
            data_holder: WidgetDataHolder::default(),
        }
    }
}

impl<W, SD> Scroll<W, SD, NoData>
where
    W: Widget,
{
    pub fn bind<D>(self, data: D) -> Scroll<W, SD, D>
    where
        D: WidgetData,
    {
        Scroll {
            fields: ScrollFields {
                parent_index: self.fields.parent_index,
                inner: self.fields.inner,
                bounds: self.fields.bounds,
                on_state_changed: |_, _| (),
                direction: self.fields.direction,
                state: self.fields.state,
                last_pointer_pos: self.fields.last_pointer_pos,
                on_scroll_changed: |_, _| (),
            },
            data_holder: WidgetDataHolder::new(data),
        }
    }
}

impl<W, SD, D> Scroll<W, SD, D>
where
    W: Widget,
    D: WidgetData,
    SD: ScrollDirection,
{
    pub fn on_data_changed(
        mut self,
        callback: fn(&mut ScrollFields<W, SD, D::Data>, &D::Data),
    ) -> Self
    where
        D: WidgetData,
    {
        self.data_holder.on_data_changed = callback;
        self
    }

    pub fn on_scroll_changed(mut self, callback: fn(&mut D::Data, ScrollData)) -> Self
    where
        D: WidgetData,
    {
        self.fields.on_scroll_changed = callback;
        self
    }

    fn change_offset(&mut self, offset: PositionDelta) {
        self.fields.direction.change_offset(offset);

        // Clamp the offset.
        let child_size = self.fields.inner.bounding_box().size;
        let own_size = self.bounding_box().size;

        let PositionDelta { x: dx, y: dy } = self.fields.direction.offset();

        let max_offset_x = child_size.width.saturating_sub(own_size.width);
        let max_offset_y = child_size.height.saturating_sub(own_size.height);

        let offset = PositionDelta {
            x: dx.min(max_offset_x as i32),
            y: dy.min(max_offset_y as i32),
        };

        // Apply clamping.
        self.fields.direction.override_offset(offset);

        // Fire callback
        let scroll_data = ScrollData {
            maximum_offset: SD::scroll_direction(max_offset_x, max_offset_y),
            offset: SD::scroll_direction(offset.x as u32, offset.y as u32),
            viewport_size: SD::scroll_direction(own_size.width, own_size.height),
        };

        let callback = self.fields.on_scroll_changed;
        self.data_holder
            .data
            .update(|data| callback(data, scroll_data));
    }
}

impl<W, SD, D> WidgetStateHolder for Scroll<W, SD, D>
where
    W: Widget,
    D: WidgetData,
{
    fn change_state(&mut self, state: u32) {
        self.fields.state.change_state(state);
    }

    fn change_selection(&mut self, state: bool) {
        self.fields.state.change_selection(state);
    }

    fn is_selectable(&self) -> bool {
        false
    }
}

impl<W, SD, D> Widget for Scroll<W, SD, D>
where
    W: Widget,
    SD: ScrollDirection,
    D: WidgetData,
{
    fn attach(&mut self, parent: usize, self_index: usize) {
        self.set_parent(parent);
        self.fields.inner.attach(self_index, self_index + 1);
    }

    fn arrange(&mut self, position: Position) {
        let offset = self.fields.direction.offset();

        self.fields.inner.arrange(position - offset);
        self.bounding_box_mut().position = position;
    }

    fn bounding_box(&self) -> BoundingBox {
        self.fields.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.fields.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let (width_spec, height_spec) = SD::merge_directions(
            MeasureConstraint::Unspecified,
            SD::cross_direction(measure_spec.width, measure_spec.height),
        );
        self.fields.inner.measure(MeasureSpec {
            width: width_spec,
            height: height_spec,
        });

        // Scroll is as big as parent lets it to be, children are (depending on the direction)
        // as big as they want to be. If parent gives us an unspecified dimension, scroll will take
        // up as much space as the child.

        let child_size = self.fields.inner.bounding_box().size;

        let main_size = SD::scroll_direction(measure_spec.width, measure_spec.height)
            .largest()
            .unwrap_or(SD::scroll_direction(child_size.width, child_size.height));
        let cross_size = SD::cross_direction(child_size.width, child_size.height);

        let (width, height) = SD::merge_directions(main_size, cross_size);

        self.set_measured_size(MeasuredSize { width, height });
    }

    fn children(&self) -> usize {
        1 + self.fields.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.fields.inner
        } else {
            self.fields.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.fields.inner
        } else {
            self.fields.inner.get_mut_child(idx - 1)
        }
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        match event {
            InputEvent::PointerEvent(position, PointerEvent::Hover) => {
                // Need to keep track of hovered state because a Scroll event has no position
                self.change_state(if self.bounding_box().contains(position) {
                    Scroll::STATE_HOVERED
                } else {
                    Scroll::STATE_IDLE
                });

                self.fields.inner.test_input(event).map(|idx| idx + 1)
            }

            InputEvent::ScrollEvent(_) => self
                .fields
                .inner
                .test_input(event)
                .map(|idx| idx + 1)
                .or_else(|| {
                    if self.fields.state.state() == Scroll::STATE_HOVERED {
                        Some(0)
                    } else {
                        None
                    }
                }),

            InputEvent::PointerEvent(_position, PointerEvent::Down) => {
                // Pointer down = start drag-scrolling
                if let Some(idx) = self.fields.inner.test_input(event) {
                    // we give priority to our child
                    Some(idx + 1)
                } else if self.fields.state.state() == Scroll::STATE_HOVERED {
                    // Avoid jumping when some events were handled by children.
                    self.fields.last_pointer_pos = None;
                    Some(0)
                } else {
                    None
                }
            }

            // We only get Up if we handled Down
            InputEvent::PointerEvent(_, PointerEvent::Drag | PointerEvent::Up) => {
                if self.fields.state.state() == Scroll::STATE_HOVERED {
                    Some(0)
                } else {
                    None
                }
            }

            _ => self.fields.inner.test_input(event).map(|idx| idx + 1),
        }
        // TODO: multiple interaction modes:
        // touch should drag the inner widgets, mouse probably shouldn't
    }

    fn handle_input(&mut self, _ctxt: InputContext, event: InputEvent) -> bool {
        let hovered = self.fields.state.state() == Scroll::STATE_HOVERED;
        match event {
            InputEvent::ScrollEvent(ScrollEvent::HorizontalScroll(dx)) => {
                self.change_offset(PositionDelta { x: -dx, y: 0 });
            }

            InputEvent::ScrollEvent(ScrollEvent::VerticalScroll(dy)) => {
                self.change_offset(PositionDelta { x: 0, y: -dy });
            }

            InputEvent::PointerEvent(position, evt) if hovered => {
                let last = self.fields.last_pointer_pos;
                self.fields.last_pointer_pos = match evt {
                    PointerEvent::Drag => Some(position),
                    PointerEvent::Up => None,
                    _ => return false,
                };

                if let Some(last) = last {
                    self.change_offset(last - position);
                }
            }

            _ => return false,
        };

        true
    }
}

impl<W, SD, D> UpdateHandler for Scroll<W, SD, D>
where
    W: Widget,
    SD: ScrollDirection,
    D: WidgetData,
{
    fn update(&mut self) {
        self.data_holder.update(&mut self.fields);
    }
}

impl<W, SD, D> ParentHolder for Scroll<W, SD, D>
where
    W: Widget,
    D: WidgetData,
{
    fn parent_index(&self) -> usize {
        self.fields.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.fields.parent_index = index;
    }
}
