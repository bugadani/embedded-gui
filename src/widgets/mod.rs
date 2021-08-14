//! Base widgets
//! ============
//!
//! *Note*: this list is not exhaustive as backend crates may also define custom widgets.
//!
//! Combining widgets
//! -----------------
//!
//! The set of widgets have been designed to do as little as necessary. This means that you have
//! different widgets to specify background color, border color and thickness, and so on.
//!
//! You are free to combine these widgets. As an example, you can layer two `Border` widgets and
//! a `Spacing` to create a double border effect.
//!
//! Backend crates might define certain arrangements to simplify usage.
//!
//! Helper functions
//! ----------------
//!
//! Sometimes we want to reuse a particular combination of widgets. To do this, we can create
//! functions that returns the nested structure we describe.
//!
//! Due to how `embedded-gui` is built, we must specify the exact type of the nested widget
//! structure. Thankfully, if we did our part well, the compiler can help us with this.
//!
//! Consider the following example, where we want to create function that constructs a
//! `TextBlock` with a border. We can implement our function like this, purposefully omitting the
//! return type:
//!
//! ```rust,ignore
//! fn button_with_style<W: Widget>(inner: W) -> _ {
//!     Button::new(
//!         // we need to help the compiler a bit
//!         Background::<_, BackgroundStyle<BinaryColor>>::new(
//!             Border::<_, BorderStyle<BinaryColor>>::new(
//!                 FillParent::both(inner)
//!                     .align_horizontal(Center)
//!                     .align_vertical(Center),
//!             )
//!             .border_color(BinaryColor::Off),
//!         )
//!         .background_color(BinaryColor::Off),
//!     )
//! }
//! ```
//!
//! If we try to build this, the compiler will give us the correct return type:
//!
//! ```text
//! error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
//!    --> examples\hello_world.rs:108:46
//!     |
//! 108 | fn button_with_style<W: Widget>(inner: W) -> _ {
//!     |                                              ^
//!     |                                              |
//!     |                                              not allowed in type signatures
//!     |                                              help: replace with the correct return type: `Button<Background<Border<FillParent<W, HorizontalAndVertical, embedded_gui::widgets::fill::Center, embedded_gui::widgets::fill::Center>, BorderStyle<BinaryColor>>, BackgroundStyle<BinaryColor>>>`
//! ```
//!
//! If we have supplied the compiler with enough information, it will be able to tell us the exact
//! return type we need to paste in place of the `_`.
//!

use crate::{
    geometry::{measurement::MeasureSpec, BoundingBox, Position},
    input::{controller::InputContext, event::InputEvent},
    state::WidgetState,
};

pub mod background;
pub mod border;
pub mod button;
pub mod fill;
pub mod graphical;
pub mod label;
pub mod layouts;
pub mod scroll;
pub mod slider;
pub mod spacing;
pub mod text_block;
pub mod text_box;
pub mod toggle;
pub mod utils;
pub mod visibility;

pub trait Widget {
    fn attach(&mut self, parent: usize, index: usize) {
        debug_assert!(index == 0 || parent != index);
        debug_assert!(
            self.children() == 0,
            "Attach must be implemented by non-leaf widgets"
        );
        self.set_parent(parent);
    }

    fn bounding_box(&self) -> BoundingBox;

    fn bounding_box_mut(&mut self) -> &mut BoundingBox {
        unimplemented!()
    }

    fn children(&self) -> usize {
        0
    }

    fn get_child(&self, _idx: usize) -> &dyn Widget {
        unimplemented!()
    }

    fn get_mut_child(&mut self, _idx: usize) -> &mut dyn Widget {
        unimplemented!()
    }

    fn measure(&mut self, measure_spec: MeasureSpec);

    fn arrange(&mut self, position: Position) {
        debug_assert!(
            self.children() == 0,
            "Arrange must be implemented by non-leaf widgets"
        );
        self.bounding_box_mut().position = position;
    }

    fn update(&mut self) {}

    fn parent_index(&self) -> usize;

    fn set_parent(&mut self, _index: usize) {}

    fn test_input(&mut self, _event: InputEvent) -> Option<usize> {
        None
    }

    fn handle_input(&mut self, _ctxt: InputContext, _event: InputEvent) -> bool {
        false
    }

    fn on_state_changed(&mut self, state: WidgetState);

    fn is_selectable(&self) -> bool {
        false
    }
}
