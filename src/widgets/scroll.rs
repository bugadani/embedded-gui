use crate::{
    data::{NoData, WidgetData},
    geometry::{BoundingBox, Position, PositionDelta},
    input::{
        controller::InputContext,
        event::{InputEvent, PointerEvent, ScrollEvent},
    },
    widgets::{ParentHolder, UpdateHandler, Widget, WidgetDataHolder, WidgetStateHolder},
    MeasureConstraint, MeasureSpec, MeasuredSize, WidgetState,
};

pub struct ScrollData {
    /// The current scroll position.
    pub offset: i32,

    /// The largest possible value of `offset`.
    pub maximum_offset: i32,

    /// The Scroll widget's main size (height of Vertical, width of Horizontal).
    pub viewport_size: i32,
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

pub trait FlingController {
    fn update(&mut self);
    fn start_fling(&mut self);
    fn stop_fling(&mut self);
    fn fling_delta(&self) -> i32;
    fn set_fling_delta(&mut self, delta: i32);
}

pub struct NoFling;

pub struct PointerFling {
    next_delta: i32,
    delta: i32,
    divisor: i32,
    friction: i32,
}

impl PointerFling {
    fn new() -> Self {
        Self {
            delta: 0,
            next_delta: 0,
            divisor: 1,
            friction: 1,
        }
    }

    pub fn friction(&mut self, friction: i32) -> &mut Self {
        self.friction = friction;
        self
    }

    pub fn divisor(&mut self, divisor: i32) -> &mut Self {
        self.divisor = divisor;
        self
    }
}

impl FlingController for NoFling {
    fn start_fling(&mut self) {}
    fn stop_fling(&mut self) {}

    fn update(&mut self) {}

    fn fling_delta(&self) -> i32 {
        0
    }

    fn set_fling_delta(&mut self, _delta: i32) {}
}

impl FlingController for PointerFling {
    fn start_fling(&mut self) {
        // multiplication because a friction value of 1 might be too big
        self.delta = self.next_delta * self.divisor;
        self.next_delta = 0;
    }

    fn stop_fling(&mut self) {
        self.delta = 0;
    }

    fn update(&mut self) {
        if self.delta != 0 {
            self.delta = if self.delta < 0 {
                (self.delta + self.friction).min(0)
            } else {
                (self.delta - self.friction).max(0)
            };
        }
    }

    fn fling_delta(&self) -> i32 {
        self.delta / self.divisor
    }

    fn set_fling_delta(&mut self, delta: i32) {
        self.next_delta = delta;
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
    pub offset_target: Option<i32>,
    pub scroll_time: u32,
}

impl<W, SD, D> ScrollFields<W, SD, D> {
    pub fn scroll_to(&mut self, offset: i32) {
        self.offset_target = Some(offset);
    }

    pub fn scroll_time(&mut self, time: u32) -> &mut Self {
        self.scroll_time = time;
        self
    }
}

impl Scroll<(), (), NoData> {
    const STATE_IDLE: u32 = 0;
    const STATE_HOVERED: u32 = 1;
}

pub struct Scroll<W, SD, D = NoData, F = PointerFling>
where
    D: WidgetData,
{
    pub fields: ScrollFields<W, SD, D::Data>,
    data_holder: WidgetDataHolder<ScrollFields<W, SD, D::Data>, D>,
    fling_controller: F,
}

impl<W> Scroll<W, Horizontal, NoData, PointerFling>
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
                offset_target: None,
                scroll_time: 6,
            },
            fling_controller: PointerFling::new(),
            data_holder: WidgetDataHolder::default(),
        }
    }
}

impl<W> Scroll<W, Vertical, NoData, PointerFling>
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
                offset_target: None,
                scroll_time: 6,
            },
            fling_controller: PointerFling::new(),
            data_holder: WidgetDataHolder::default(),
        }
    }
}

impl<W, SD, D> Scroll<W, SD, D, PointerFling>
where
    W: Widget,
    D: WidgetData,
{
    /// Sets the friction value.
    ///
    /// A higher value results in a shorter fling.
    pub fn friction(mut self, friction: i32) -> Self {
        self.fling_controller.friction(friction);
        self
    }

    /// Sets the friction divisor.
    ///
    /// A higher value results in a smaller overall friction value, i.e. a longer fling.
    /// Used to fine-tune the friction or to allow for smaller friction values.
    pub fn friction_divisor(mut self, divisor: i32) -> Self {
        self.fling_controller.divisor(divisor);
        self
    }

    /// Determines the time it takes for `scroll_to` to reach its target.
    ///
    /// The bigger the `time` parameter, the slower the scrolling speed. Does not affect manual
    /// scrolling speed.
    pub fn scroll_time(mut self, time: u32) -> Self {
        self.fields.scroll_time(time);
        self
    }
}

impl<W, SD, F> Scroll<W, SD, NoData, F>
where
    W: Widget,
{
    pub fn bind<D>(self, data: D) -> Scroll<W, SD, D, F>
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
                offset_target: self.fields.offset_target,
                scroll_time: self.fields.scroll_time,
            },
            fling_controller: self.fling_controller,
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
    pub fn disable_fling(self) -> Scroll<W, SD, D, NoFling> {
        Scroll {
            fields: self.fields,
            fling_controller: NoFling,
            data_holder: self.data_holder,
        }
    }

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

        let max_offset_x = child_size.width.saturating_sub(own_size.width) as i32;
        let max_offset_y = child_size.height.saturating_sub(own_size.height) as i32;

        let offset = PositionDelta {
            x: dx.min(max_offset_x),
            y: dy.min(max_offset_y),
        };

        // Apply clamping.
        self.fields.direction.override_offset(offset);

        // Fire callback
        let scroll_data = ScrollData {
            maximum_offset: SD::scroll_direction(max_offset_x, max_offset_y),
            offset: SD::scroll_direction(offset.x, offset.y),
            viewport_size: SD::scroll_direction(own_size.width as i32, own_size.height as i32),
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
            InputEvent::Cancel => {
                self.fields.inner.test_input(InputEvent::Cancel);
                None
            }

            InputEvent::PointerEvent(position, PointerEvent::Hover) => {
                // Need to keep track of hovered state because a Scroll event has no position
                if self.bounding_box().contains(position) {
                    self.change_state(Scroll::STATE_HOVERED);
                    self.fields.inner.test_input(event).map(|idx| idx + 1)
                } else {
                    self.change_state(Scroll::STATE_IDLE);
                    self.fields.inner.test_input(InputEvent::Cancel);
                    None
                }
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

            InputEvent::PointerEvent(position, PointerEvent::Down) => {
                // Pointer down = start drag-scrolling
                if self.bounding_box().contains(position) {
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
                self.fling_controller.stop_fling();

                // Cancel ongoing scroll_to
                self.fields.offset_target = None;
            }

            InputEvent::ScrollEvent(ScrollEvent::VerticalScroll(dy)) => {
                self.change_offset(PositionDelta { x: 0, y: -dy });
                self.fling_controller.stop_fling();

                // Cancel ongoing scroll_to
                self.fields.offset_target = None;
            }

            InputEvent::PointerEvent(position, evt) if hovered => {
                self.fields.last_pointer_pos = match evt {
                    PointerEvent::Down => {
                        self.fling_controller.stop_fling();

                        // Cancel ongoing scroll_to
                        self.fields.offset_target = None;

                        Some(position)
                    }

                    PointerEvent::Drag => {
                        self.fling_controller.stop_fling();
                        let delta = if let Some(last) = self.fields.last_pointer_pos {
                            let delta = last - position;
                            self.change_offset(last - position);
                            SD::scroll_direction(delta.x, delta.y)
                        } else {
                            0
                        };
                        self.fling_controller.set_fling_delta(delta);

                        Some(position)
                    }

                    PointerEvent::Up => {
                        self.fling_controller.start_fling();

                        None
                    }

                    _ => return false,
                };
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

        if let Some(target) = self.fields.offset_target {
            let current_offset = SD::scroll_direction(
                self.fields.direction.offset().x,
                self.fields.direction.offset().y,
            );

            if target == current_offset {
                self.fields.offset_target = None;
                return;
            }

            let delta = target - current_offset;
            let frames = self.fields.scroll_time as i32;

            let delta = if delta / frames == 0 {
                if target > current_offset {
                    1
                } else {
                    -1
                }
            } else {
                delta / frames
            };

            let (x, y) = SD::merge_directions(delta, 0);
            self.change_offset(PositionDelta { x, y });

            self.fling_controller.stop_fling();
        } else {
            self.fling_controller.update();
            let delta = self.fling_controller.fling_delta();
            if delta != 0 {
                let (x, y) = SD::merge_directions(delta, 0);
                self.change_offset(PositionDelta { x, y });
            }
        }
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
