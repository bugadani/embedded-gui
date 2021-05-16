//! Frame layout.
//!
//! The Frame layout places widgets on top of each other.

use object_chain::Chain;

use crate::{geometry::BoundingBox, widgets::layouts::frame::layout::FrameLayout};

pub mod layout;

pub struct Frame;
impl Frame {
    pub fn new() -> Self {
        Self
    }

    pub fn add_layer<W>(self, inner: W) -> FrameLayout<Chain<W>> {
        FrameLayout {
            widgets: Chain::new(inner),
            bounds: BoundingBox::default(),
        }
    }
}
