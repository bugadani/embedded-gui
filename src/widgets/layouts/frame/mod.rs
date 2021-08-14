//! Frame layout.
//!
//! The Frame layout places widgets on top of each other.

use object_chain::Chain;

use crate::{geometry::BoundingBox, widgets::layouts::frame::layout::FrameLayout};

mod layout;

pub struct Frame;
impl Frame {
    /// Creates a new, empty frame layout.
    pub fn new() -> Self {
        Self
    }

    /// Adds a new widget on top of the previous layers.
    pub fn add_layer<W>(self, inner: W) -> FrameLayout<Chain<W>> {
        FrameLayout {
            widgets: Chain::new(inner),
            bounds: BoundingBox::default(),
        }
    }
}
