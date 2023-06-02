use manifest::InstanceIdRange;
use std::ops::RangeInclusive;

pub struct IdAllocator {
    current: usize,
    end: usize,
}

impl IdAllocator {
    pub fn new(range: InstanceIdRange) -> Self {
        Self {
            current: *range.range().start(),
            end: *range.range().end(),
        }
    }

    pub fn next(&mut self) -> Option<usize> {
        let id = self.current;
        if id + 1 > self.end {
            return None;
        }
        self.current += 1;
        Some(id)
    }
}
