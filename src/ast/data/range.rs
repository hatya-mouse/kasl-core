#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, serde::Serialize)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Default for Range {
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl Range {
    pub fn n(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    pub fn zero() -> Self {
        Range { start: 0, end: 0 }
    }
}
