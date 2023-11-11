pub mod node;
mod position;

use crate::node::Node;
use crate::position::Position;
use std::collections::HashMap;
pub struct Grid {
    nodes: HashMap<Position, Node>,
    goal: Option<Position>,
    start: Option<Position>,
}

impl Grid {
    pub fn new(height: usize, length: usize) -> Self {
        let mut nodes = HashMap::new();
        for y in 0..height {
            for x in 0..length {
                let pos = Position::new(x as i32, y as i32);
                let mut node = Node::default();
                node.index = height * y + x;
                nodes.insert(pos, node);
            }
        }
        Grid {
            nodes,
            goal: None,
            start: None,
        }
    }

    pub fn get_node_at(&self, x: usize, y: usize) -> &Node {
        let pos = Position::new(x as i32, y as i32);
        let node = self.nodes.get(&pos);
        node.expect(&*format!("{:?} is invalid", pos))
    }

    pub fn set_obstacle(&mut self, x: usize, y: usize) {
        let pos = Position::new(x as i32, y as i32);
        let node = self.nodes.entry(pos).and_modify(|node| node.set_obstacle());
    }

    pub fn set_start(&mut self, x: usize, y: usize) {
        let pos = Position::new(x as i32, y as i32);
        self.start = Some(pos);
    }

    pub fn set_goal(&mut self, x: usize, y: usize) {
        let pos = Position::new(x as i32, y as i32);
        self.goal = Some(pos);
    }

    pub fn is_goal(&self, x: usize, y: usize) -> bool {
        let pos = Position::new(x as i32, y as i32);
        Some(pos) == self.goal
    }

    pub fn is_start(&self, x: usize, y: usize) -> bool {
        let pos = Position::new(x as i32, y as i32);
        Some(pos) == self.start
    }
}
