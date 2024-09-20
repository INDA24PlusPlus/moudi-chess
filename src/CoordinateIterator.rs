use std::iter::Iterator;

pub struct CoordinateIterator {
    current: (usize, usize),
    end: (usize, usize),
    dx: isize,
    dy: isize,
}

impl CoordinateIterator {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self { 
        CoordinateIterator {
            current: start,
            end,
            dx: (end.0 as isize - start.0 as isize).signum(),
            dy: (end.1 as isize - start.1 as isize).signum(),
        }
    }

    pub fn contains(&mut self, coord: (usize, usize)) -> bool {
        self.find(|(x, y)| *x == coord.0 && *y == coord.1) != None
    }
}

impl Iterator for CoordinateIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.dx != 0 && self.current.0 == self.end.0) || (self.dy != 0 && self.current.1 == self.end.1) || self.current == self.end {
            return None;
        }
        
        self.current.0 = (self.current.0 as isize + self.dx) as usize;
        self.current.1 = (self.current.1 as isize + self.dy) as usize;

        Some(self.current)
    }
}
