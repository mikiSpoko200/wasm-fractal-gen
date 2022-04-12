pub struct Linspace {
    start: f64,
    end: f64,
    size: usize,
    step:  f64,
    iter:  usize,
    biter: usize,
}

impl Iterator for Linspace {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.iter < self.size { 
            let new = self.iter as f64 * self.step + self.start;
            self.iter += 1;
            Some(new)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for Linspace {
    fn next_back(&mut self) -> Option<f64> {
        return if self.biter < self.size {
            let new = self.end - self.step * self.biter as f64;
            self.biter += 1;
            Some(new)
        } else {
            None
        }
    }
}

impl Linspace {
    pub fn new(start: f64, end: f64, size: usize) -> Self {
        assert!(start < end, "start needs to be smaller than end in Linspace::new.");
        Self{start, end, size, step: (end - start) / (size as f64 - 1.0), iter: 0, biter: 0}
    }
}
