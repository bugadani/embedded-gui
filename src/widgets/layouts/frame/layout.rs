use object_chain::{Chain, ChainElement, Link};

use crate::{
    geometry::{measurement::MeasureSpec, BoundingBox, MeasuredSize, Position},
    input::event::InputEvent,
    state::WidgetState,
    widgets::Widget,
    Canvas, WidgetRenderer,
};

pub struct FrameLayout<CE> {
    pub widgets: CE,
    pub bounds: BoundingBox,
}

impl<CE> FrameLayout<CE>
where
    CE: ChainElement,
{
    pub fn add_layer<W>(self, inner: W) -> FrameLayout<Link<W, CE>> {
        FrameLayout {
            widgets: self.widgets.append(inner),
            bounds: self.bounds,
        }
    }
}

pub trait FrameLayoutChainElement {
    fn at(&self, index: usize) -> &dyn Widget;

    fn at_mut(&mut self, index: usize) -> &mut dyn Widget;

    fn test_input(&mut self, event: InputEvent) -> Option<usize>;

    fn count_widgets(&self) -> usize;

    fn measure(&mut self, spec: MeasureSpec) -> MeasuredSize;

    fn arrange(&mut self, position: Position);

    fn on_state_changed(&mut self, state: WidgetState);

    fn update(&mut self);
}

impl<W> FrameLayoutChainElement for Chain<W>
where
    W: Widget,
{
    fn at(&self, index: usize) -> &dyn Widget {
        debug_assert!(index == 0);

        &self.object
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn Widget {
        debug_assert!(index == 0);

        &mut self.object
    }

    fn measure(&mut self, spec: MeasureSpec) -> MeasuredSize {
        self.object.measure(spec);
        self.object.bounding_box().size
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        self.object.test_input(event)
    }

    fn count_widgets(&self) -> usize {
        self.object.children() + 1
    }

    fn arrange(&mut self, position: Position) {
        self.object.arrange(position);
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        self.object.on_state_changed(state);
    }

    fn update(&mut self) {
        self.object.update();
    }
}

impl<W, CE> FrameLayoutChainElement for Link<W, CE>
where
    W: Widget,
    CE: FrameLayoutChainElement + ChainElement,
{
    fn at(&self, index: usize) -> &dyn Widget {
        if index == Link::len(self) - 1 {
            return &self.object;
        }

        self.parent.at(index)
    }

    fn at_mut(&mut self, index: usize) -> &mut dyn Widget {
        if index == Link::len(self) - 1 {
            return &mut self.object;
        }

        self.parent.at_mut(index)
    }

    fn measure(&mut self, spec: MeasureSpec) -> MeasuredSize {
        let inner_size = self.parent.measure(spec);
        self.object.measure(spec);
        let size = self.object.bounding_box().size;

        MeasuredSize {
            width: inner_size.width.max(size.width),
            height: inner_size.height.max(size.height),
        }
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        self.parent.test_input(event).or_else(|| {
            self.object
                .test_input(event)
                .map(|idx| idx + self.parent.count_widgets())
        })
    }

    fn count_widgets(&self) -> usize {
        self.object.children() + 1 + self.parent.count_widgets()
    }

    fn arrange(&mut self, position: Position) {
        self.parent.arrange(position);
        self.object.arrange(position);
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        self.object.on_state_changed(state);
        self.parent.on_state_changed(state);
    }

    fn update(&mut self) {
        self.object.update();
        self.parent.update();
    }
}

impl<C, W> WidgetRenderer<C> for Chain<W>
where
    W: Widget + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.object.draw(canvas)
    }
}

impl<C, W, CE> WidgetRenderer<C> for Link<W, CE>
where
    W: Widget + WidgetRenderer<C>,
    CE: FrameLayoutChainElement + ChainElement + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.parent.draw(canvas)?;
        self.object.draw(canvas)
    }
}

impl<CE> FrameLayout<CE>
where
    CE: ChainElement + FrameLayoutChainElement,
{
    fn locate(&self, mut idx: usize) -> Option<(usize, usize)> {
        let children = self.widgets.len();

        for i in 0..children {
            let child = self.widgets.at(i);
            let grandchildren = child.children();
            if idx <= grandchildren {
                return Some((i, idx));
            }

            idx -= grandchildren + 1;
        }

        None
    }
}

impl<CE> Widget for FrameLayout<CE>
where
    CE: FrameLayoutChainElement + ChainElement,
{
    fn attach(&mut self, parent: usize, index: usize) {
        debug_assert!(index == 0 || parent != index);
        let mut children = index;

        for i in 0..self.widgets.len() {
            let widget = self.widgets.at_mut(i);

            widget.attach(parent, children + i);
            children += widget.children();
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        &mut self.bounds
    }

    fn measure(&mut self, measure_spec: MeasureSpec) {
        self.bounds.size = self.widgets.measure(measure_spec);
    }

    fn arrange(&mut self, position: Position) {
        self.bounds.position = position;

        self.widgets.arrange(position);
    }

    fn children(&self) -> usize {
        self.widgets.count_widgets()
    }

    fn get_child(&self, idx: usize) -> &dyn Widget {
        let (child, grandchild) = self.locate(idx).unwrap();

        let widget = self.widgets.at(child);
        if grandchild == 0 {
            widget
        } else {
            widget.get_child(grandchild - 1)
        }
    }

    fn get_mut_child(&mut self, idx: usize) -> &mut dyn Widget {
        let (child, grandchild) = self.locate(idx).unwrap();

        let widget = self.widgets.at_mut(child);
        if grandchild == 0 {
            widget
        } else {
            widget.get_mut_child(grandchild - 1)
        }
    }

    fn parent_index(&self) -> usize {
        unimplemented!()
    }

    fn set_parent(&mut self, _index: usize) {}

    fn update(&mut self) {
        self.widgets.update();
    }

    fn test_input(&mut self, event: InputEvent) -> Option<usize> {
        self.widgets.test_input(event).map(|idx| idx + 1)
    }

    fn on_state_changed(&mut self, state: WidgetState) {
        self.widgets.on_state_changed(state);
    }

    fn is_selectable(&self) -> bool {
        false
    }
}

impl<C, CE> WidgetRenderer<C> for FrameLayout<CE>
where
    CE: FrameLayoutChainElement + ChainElement + WidgetRenderer<C>,
    C: Canvas,
{
    fn draw(&self, canvas: &mut C) -> Result<(), C::Error> {
        self.widgets.draw(canvas)
    }
}
