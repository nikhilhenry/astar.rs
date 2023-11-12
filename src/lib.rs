pub mod node;
mod position;

use crate::node::Node;
use crate::position::Position;
use std::cell::{Ref, RefCell};
use std::collections::{BinaryHeap, HashMap};
use std::ops::Deref;

pub struct Grid {
    height: usize,
    width: usize,
    nodes: HashMap<Position, Node>,
    goal: Option<Position>,
    start: Option<Position>,
}

const OFFSETS: [Position; 4] = [
    Position::new(1, 0),
    Position::new(-1, 0),
    Position::new(0, 1),
    Position::new(0, -1),
];

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let mut nodes = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let pos = Position::new(x as i32, y as i32);
                let mut node = Node::default();
                node.index = height * y + x;
                nodes.insert(pos, node);
            }
        }
        Grid {
            height,
            width,
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

    fn is_valid_pos(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    fn get_neighbours(&self, me: &Position) -> Vec<Position> {
        OFFSETS
            .iter()
            .map(|offset| me + offset)
            .filter(|pos| self.is_valid_pos(&pos))
            .collect()
    }

    pub fn set_obstacle(&mut self, x: usize, y: usize) {
        let pos = Position::new(x as i32, y as i32);
        self.nodes.entry(pos).and_modify(|node| node.set_obstacle());
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
