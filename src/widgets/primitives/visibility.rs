use crate::{
    data::WidgetData,
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    input::event::InputEvent,
    state::WidgetState,
    widgets::{wrapper::Wrapper, ParentHolder, UpdateHandler, Widget, WidgetStateHolder},
    Canvas, WidgetRenderer,
};

pub struct Visibility<W> {
    pub inner: W,
    pub parent_index: usize,
    pub visibility: bool,
    pub on_state_changed: fn(&mut Self, WidgetState),
}

impl<W> Visibility<W>
where
    W: Widget,
{
    pub fn new(inner: W) -> Visibility<W> {
        Visibility {
            parent_index: 0,
            inner,
            visibility: true,
            on_state_changed: |_, _| (),
        }
    }

    pub fn bind<D>(self, data: D) -> Wrapper<Visibility<W>, D>
    where
        D: WidgetData,
    {
        Wrapper::wrap(self, data)
    }
}

impl<W> Visibility<W> {
    pub fn visible(mut self, visibility: bool) -> Self {
        self.set_visible(visibility);
        self
    }

    pub fn set_visible(&mut self, visibility: bool) {
        self.visibility = visibility;
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Self, WidgetState)) -> Self {
        self.on_state_changed = callback;
        self
    }
}

impl<W, D> Wrapper<Visibility<W>, D>
where
    W: Widget,
    D: WidgetData,
{
    pub fn visible(mut self, visibility: bool) -> Self {
        self.widget.set_visible(visibility);
        self
    }

    pub fn on_state_changed(mut self, callback: fn(&mut Visibility<W>, WidgetState)) -> Self {
        // TODO this should be pulled up
        self.widget.on_state_changed = callback;
        self
    }
}

impl<W> Widget for Visibility<W>
where
    W: Widget,
{
    fn attach(&mut self, parent: usize, self_index: usize) {
        self.set_parent(parent);
        self.inner.attach(self_index, self_index + 1);
    }

    fn arrange(&mut self, position: Position) {
        self.inner.arrange(position);
    }

    fn bounding_box(&self) -> BoundingBox {
        if self.visibility {
            self.inner.bounding_box()
        } else {
            BoundingBox {
                position: self.inner.bounding_box().position,
                size: MeasuredSize {
                    width: 0,
                    height: 0,
                },
            }
        }
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.inner.measure(measure_spec);
    }

    fn children(&self) -> usize {
        1 + self.inner.children()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        if idx == 0 {
            &self.inner
        } else {
            self.inner.get_child(idx - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        if idx == 0 {
            &mut self.inner
        } else {
            self.inner.get_mut_child(idx - 1)
        }
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        // We just relay whatever the child desires
        self.inner.test_input(event).map(|i| i + 1)
    }
}

impl<W> UpdateHandler for Visibility<W>
where
    W: Widget,
{
    fn update(&mut self) {
        self.inner.update();
    }
}

impl<W> ParentHolder for Visibility<W>
where
    W: Widget,
{
    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent(&mut self, index: usize) {
        self.parent_index = index;
    }
}

impl<W> WidgetStateHolder for Visibility<W>
where
    W: Widget,
{
    fn on_state_changed(&mut self, state: WidgetState) {
        (self.on_state_changed)(self, state);
        self.inner.on_state_changed(state);
    }

    fn is_selectable(&self) -> bool {
        false
    }
}

impl<C, W> WidgetRenderer<C> for Visibility<W>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        if self.visibility {
            self.inner.draw(canvas)
        } else {
            Ok(())
        }
    }
}
