//! Helper for handling horizontal and vertical orientations.

pub trait AxisOrder {
    fn main_axis<V>(x: V, y: V) -> V;
    fn cross_axis<V>(x: V, y: V) -> V;
    fn merge<V>(main: V, cross: V) -> (V, V);
}

pub struct Horizontal;
impl AxisOrder for Horizontal {
    fn main_axis<V>(x: V, _y: V) -> V {
        x
    }

    fn cross_axis<V>(_x: V, y: V) -> V {
        y
    }

    fn merge<V>(main: V, cross: V) -> (V, V) {
        (main, cross)
    }
}

pub struct Vertical;
impl AxisOrder for Vertical {
    fn main_axis<V>(_x: V, y: V) -> V {
        y
    }

    fn cross_axis<V>(x: V, _y: V) -> V {
        x
    }

    fn merge<V>(main: V, cross: V) -> (V, V) {
        (cross, main)
    }
}
