#[derive(Copy, Clone, Debug)]
pub enum MeasureConstraint {
    AtMost(u32),
    Exactly(u32),
    Unspecified,
}

impl MeasureConstraint {
    pub fn shrink(self, by: u32) -> MeasureConstraint {
        match self {
            MeasureConstraint::AtMost(size) => MeasureConstraint::AtMost(size.saturating_sub(by)),
            MeasureConstraint::Exactly(size) => MeasureConstraint::Exactly(size.saturating_sub(by)),
            MeasureConstraint::Unspecified => MeasureConstraint::Unspecified,
        }
    }

    pub fn apply_to_measured(self, measured: u32) -> u32 {
        match self {
            MeasureConstraint::AtMost(constraint) => constraint.min(measured),
            MeasureConstraint::Exactly(constraint) => constraint,
            MeasureConstraint::Unspecified => measured,
        }
    }

    pub fn to_at_most(self) -> MeasureConstraint {
        match self {
            MeasureConstraint::AtMost(size) => MeasureConstraint::AtMost(size),
            MeasureConstraint::Exactly(size) => MeasureConstraint::AtMost(size),
            MeasureConstraint::Unspecified => MeasureConstraint::AtMost(u32::MAX),
        }
    }

    pub fn largest(self) -> Option<u32> {
        match self {
            MeasureConstraint::AtMost(size) => Some(size),
            MeasureConstraint::Exactly(size) => Some(size),
            MeasureConstraint::Unspecified => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MeasureSpec {
    pub width: MeasureConstraint,
    pub height: MeasureConstraint,
}
