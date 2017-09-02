use env::Point;
use std::collections::BTreeSet;
use std::fmt;

/// A region is a set of points where, within any given basic block,
/// the points must be continuous. We represent this as a map:
///
///     B -> start..end
///
/// where `B` is a basic block identifier and start/end are indices.
#[derive(Clone, PartialEq, Eq)]
pub struct Region {
    points: BTreeSet<Point>,
    locked: bool,
}

impl Region {
    pub fn new() -> Self {
        Region {
            points: BTreeSet::new(),
            locked: false,
        }
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn add_point(&mut self, point: Point) -> bool {
        if self.locked { return false; }
        self.points.insert(point)
    }

    pub fn may_contain(&self, point: Point) -> bool {
        self.points.contains(&point)
    }
}

impl fmt::Debug for Region {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{{")?;
        for (index, point) in self.points.iter().enumerate() {
            if index > 0 {
                write!(fmt, ", ")?;
            }
            write!(fmt, "{:?}", point)?;
        }
        write!(fmt, "}}")?;
        Ok(())
    }
}
