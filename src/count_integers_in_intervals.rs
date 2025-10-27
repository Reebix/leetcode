// rust
use std::cmp::{max, min};

pub struct CountIntervals {
    intervals: Vec<(i32, i32)>,
    total: i32,
}

impl CountIntervals {
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
            total: 0,
        }
    }

    pub fn add(&mut self, left: i32, right: i32) {
        let mut l = left;
        let mut r = right;

        // insertion pos
        let mut pos = self.intervals.binary_search_by(|&(il, _)| il.cmp(&left)).unwrap_or_else(|p| p);

        // previous interval overlaps
        if pos > 0 && self.intervals[pos - 1].1 >= left - 1 {
            pos -= 1;
        }

        // merge overlapping pos --> j
        let mut j = pos;
        while j < self.intervals.len() && self.intervals[j].0 <= r + 1 {
            l = min(l, self.intervals[j].0);
            r = max(r, self.intervals[j].1);
            j += 1;
        }

        // remove modified area
        if j > pos {
            let removed: i32 = self.intervals[pos..j]
                .iter()
                .map(|(a, b)| *b - *a + 1)
                .sum();
            self.total -= removed;
            // remove overlapping
            self.intervals.drain(pos..j);
        }

        // add interval
        self.intervals.insert(pos, (l, r));
        self.total += r - l + 1;
    }

    pub fn count(&self) -> i32 {
        self.total
    }
}