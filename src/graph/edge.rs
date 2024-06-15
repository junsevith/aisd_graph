use std::cmp::Ordering;

#[derive(Debug)]
pub struct Edge<W> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
}

impl<W> Edge<W> {
    pub fn new(from: usize, to: usize, weight: W) -> Self {
        Edge { from, to, weight }
    }
}


impl<W: PartialOrd> Eq for Edge<W> {}

impl<W: PartialOrd> PartialEq<Self> for Edge<W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<W: PartialOrd> PartialOrd<Self> for Edge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<W: PartialOrd> Ord for Edge<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(&other.weight).unwrap()
    }
}
