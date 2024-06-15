struct Node {
    parent: usize,
    rank: usize,
}

pub struct SepSet {
    nodes: Vec<Option<Node>>,
}

impl SepSet {
    pub fn empty(size: usize) -> Self {
        SepSet {
            nodes: (0..size).map(|_| None).collect(),
        }
    }

    pub fn full(size: usize) -> Self {
        SepSet {
            nodes: (0..size).map(|i| Some(Node { parent: i, rank: 0 })).collect(),
        }
    }

    pub fn make_set(&mut self, id: usize) {
        self.nodes[id] = Some(Node { parent: id, rank: 0 });
    }

    pub fn find(&mut self, id: usize) -> usize {
        let node = self.nodes[id].as_ref().unwrap();
        if node.parent != id {
            self.find(node.parent)
        } else {
            id
        }
    }

    pub(crate) fn union(&mut self, id1: usize, id2: usize) {
        let root1 = self.find(id1);
        let root2 = self.find(id2);
        if root1 != root2 {
            let node1 = self.nodes[root1].as_ref().unwrap().rank;
            let node2 = self.nodes[root2].as_ref().unwrap().rank;
            if node1 < node2 {
                self.nodes[root1].as_mut().unwrap().parent = root2;
            } else if node1 > node2 {
                self.nodes[root2].as_mut().unwrap().parent = root1;
            } else {
                self.nodes[root2].as_mut().unwrap().parent = root1;
                self.nodes[root1].as_mut().unwrap().rank += 1;
            }
        }
    }
}