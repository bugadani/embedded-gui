use core::marker::PhantomData;

use crate::{
    data::WidgetData,
    input::event::InputEvent,
    widgets::{
        NoDataHolder, ParentHolder, Widget, WidgetDataHolder, WidgetDataHolderTrait,
        WidgetStateHolder, WidgetWrapper,
    },
    BoundingBox, Canvas, MeasureSpec, MeasuredSize, Position, WidgetRenderer, WidgetState,
};

#[derive(Default, Clone, Copy)]
pub struct SpacingSpec {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

pub struct Spacing<W, C> {
    pub inner: W,
    pub spacing: SpacingSpec,
    _marker: PhantomData<C>,
}

impl<W, C> Spacing<W, C>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
{
    pub fn new(inner: W) -> WidgetWrapper<Self, NoDataHolder<Self>> {
        WidgetWrapper::new(Spacing {
            spacing: SpacingSpec::default(),
            inner,
            _marker: PhantomData,
        })
    }
}

impl<W, C> Spacing<W, C> {
    pub fn set_left(&mut self, space: u32) {
        self.spacing.left = space;
    }
    pub fn set_right(&mut self, space: u32) {
        self.spacing.right = space;
    }
    pub fn set_top(&mut self, space: u32) {
        self.spacing.top = space;
    }
    pub fn set_bottom(&mut self, space: u32) {
        self.spacing.bottom = space;
    }
}

impl<W, C> WidgetWrapper<Spacing<W, C>, NoDataHolder<Spacing<W, C>>>
where
    W: Widget,
{
    pub fn bind<D>(
        self,
        data: D,
    ) -> WidgetWrapper<Spacing<W, C>, WidgetDataHolder<Spacing<W, C>, D>>
    where
        D: WidgetData,
    {
        WidgetWrapper {
            parent_index: self.parent_index,
            widget: self.widget,
            data_holder: NoDataHolder::<Spacing<W, C>>::default().bind(data),
            on_state_changed: |_, _| (),
            state: WidgetState::default(),
        }
    }
}

impl<W, C, D, DH> WidgetWrapper<Spacing<W, C>, DH>
where
    W: Widget,
    D: WidgetData,
    DH: WidgetDataHolderTrait<Data = D, Owner = Spacing<W, C>>,
{
    pub fn left(mut self, space: u32) -> Self {
        self.widget.set_left(space);
        self
    }

    pub fn right(mut self, space: u32) -> Self {
        self.widget.set_right(space);
        self
    }

    pub fn top(mut self, space: u32) -> Self {
        self.widget.set_top(space);
        self
    }

    pub fn bottom(mut self, space: u32) -> Self {
        self.widget.set_bottom(space);
        self
    }

    pub fn all(mut self, space: u32) -> Self {
        self.widget.set_left(space);
        self.widget.set_right(space);
        self.widget.set_top(space);
        self.widget.set_bottom(space);

        self
    }
}

impl<W, C, D, DH> WidgetStateHolder for WidgetWrapper<Spacing<W, C>, DH>
where
    W: Widget,
    D: WidgetData,
    DH: WidgetDataHolderTrait<Data = D, Owner = Spacing<W, C>>,
{
    fn change_state(&mut self, state: u32) {
        // propagate state to child widget
        self.widget.inner.change_state(state);

        // apply state
        if self.state.change_state(state) {
            (self.on_state_changed)(&mut self.widget, self.state);
        }
    }

    fn change_selection(&mut self, _state: bool) {}

    fn is_selectable(&self) -> bool {
        false
    }
}

impl<W, C, D, DH> Widget for WidgetWrapper<Spacing<W, C>, DH>
where
    W: Widget,
    D: WidgetData,
    DH: WidgetDataHolderTrait<Data = D, Owner = Spacing<W, C>>,
{
    fn attach(&mut self, parent: usize, self_index: usize) {
        self.set_parent(parent);
        self.widget.inner.attach(self_index, self_index + 1);
    }

    fn arrange(&mut self, position: Position) {
        let spacing = self.widget.spacing;

        self.widget.inner.arrange(Position {
            x: position.x + spacing.left as i32,
            y: position.y + spacing.top as i32,
        });
    }

    fn bounding_box(&self) -> BoundingBox {
        let spacing = self.widget.spacing;
        let bounds = self.widget.inner.bounding_box();

        BoundingBox {
            position: Position {
                x: bounds.position.x - spacing.left as i32,
                y: bounds.position.y - spacing.top as i32,
            },
            size: MeasuredSize {
                width: bounds.size.width + spacing.left + spacing.right,
                height: bounds.size.height + spacing.top + spacing.bottom,
            },
        }
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        let spacing = self.widget.spacing;

        self.widget.inner.measure(MeasureSpec {
            width: measure_spec.width.shrink(spacing.left + spacing.right),
            height: measure_spec.height.shrink(spacing.top + spacing.bottom),
        });
    }

    fn children(&self) -> usize {
        1 + self.widget.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.widget.inner
        } else {
            self.widget.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.widget.inner
        } else {
            self.widget.inner.get_mut_child(idx - 1)
        }
    }

    fn update(&mut self) {
        self.data_holder.update(&mut self.widget);
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        // We just relay whatever the child desires
        self.widget.inner.test_input(event).map(|i| i + 1)
    }
}

impl<C, W> WidgetRenderer<C> for Spacing<W, C>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.inner.draw(canvas)
    }
}
