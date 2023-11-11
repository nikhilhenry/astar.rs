use std::cmp::Ordering;
pub enum NodeType {
    Obstacle,
    Traversable,
    Traversed,
}

pub struct Node {
    pub node_type: NodeType,
    h_cost: usize,
    g_cost: usize,
    f_cost: usize,
    pub index: usize,
}

impl Node {
    pub fn set_obstacle(&mut self) {
        self.node_type = NodeType::Obstacle
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            node_type: NodeType::Traversable,
            h_cost: 0,
            g_cost: 0,
            f_cost: 0,
            index: 0,
        }
    }
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost
            .cmp(&self.f_cost)
            .then_with(|| self.index.cmp(&other.index))
    }
}
