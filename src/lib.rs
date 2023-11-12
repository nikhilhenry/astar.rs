pub mod node;
mod position;
#[cfg(test)]
mod test;

use crate::node::{Node, NodeType};
use crate::position::Position;
use std::cell::{Ref, RefCell};
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

pub struct Grid {
    height: usize,
    width: usize,
    nodes: HashMap<Position, Rc<RefCell<Node>>>,
    goal: Option<Position>,
    start: Option<Position>,
}

const OFFSETS: [Position; 4] = [
    Position::new(1, 0),  // right
    Position::new(-1, 0), // left
    Position::new(0, -1), // top
    Position::new(0, 1),  // bottom
];

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let mut nodes = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let pos = Position::new(x as i32, y as i32);
                let mut node = Node::default();
                node.index = height * y + x;
                nodes.insert(pos, Rc::new(RefCell::new(node)));
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

    pub fn is_ready(&self) -> bool {
        self.goal.is_some() && self.start.is_some()
    }

    pub fn get_node_at(&self, x: usize, y: usize) -> Ref<Node> {
        let pos = Position::new(x as i32, y as i32);
        self.nodes
            .get(&pos)
            .expect(&*format!("{:?} is invalid", pos))
            .borrow()
    }

    fn is_valid_pos(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    fn get_neighbours(&self, me: &Position) -> Vec<Position> {
        OFFSETS
            .iter()
            .map(|offset| *&me + offset)
            .filter(|pos| self.is_valid_pos(&pos))
            .collect()
    }

    fn get_index_from_pos(&self, pos: &Position) -> usize {
        self.height * (pos.y as usize) + (pos.x as usize)
    }

    fn get_pos_from_index(&self, idx: usize) -> Position {
        let row = idx / self.width;
        let col = idx - self.height * row;
        Position::new(col as i32, row as i32)
    }

    pub fn set_obstacle(&mut self, x: usize, y: usize) {
        let pos = Position::new(x as i32, y as i32);
        self.nodes
            .entry(pos)
            .and_modify(|node| node.borrow_mut().set_obstacle());
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

    fn trace_path(&self, position: Position) {
        let node = self.nodes.get(&position).unwrap();
        let parent = node.borrow().parent.clone();
        node.borrow_mut().node_type = NodeType::Obstacle;

        if let Some(pos) = parent {
            self.trace_path(pos)
        }
    }

    pub fn solve(&mut self) {
        let mut iters = 0;
        let start_pos = self.start.clone();
        let Some(start_pos) = start_pos else {
            panic!("no start position");
        };
        let goal_pos = self.goal.clone();
        let Some(goal_pos) = goal_pos else {
            panic!("no goal position")
        };

        let goal = self.get_index_from_pos(&goal_pos);

        let mut open_set = BinaryHeap::new();
        let start_node = self
            .nodes
            .get(&start_pos)
            .expect("must be valid start position");
        start_node.borrow_mut().h_cost = 0;
        start_node.borrow_mut().g_cost = 0;
        start_node.borrow_mut().f_cost = 0;

        // start_node.node_type = NodeType::Traversed;
        open_set.push(start_node.clone());

        while let Some(current_node) = open_set.pop() {
            iters += 1;
            if current_node.borrow().index == goal {
                println!("solution found in {iters}");
                self.trace_path(
                    current_node
                        .borrow()
                        .parent
                        .clone()
                        .expect("need to have a parent"),
                );
                break;
            }
            let current_pos = self.get_pos_from_index(current_node.borrow().index);
            for pos in self.get_neighbours(&current_pos) {
                let neighbour = self.nodes.get(&pos).expect("invalid position");
                let temp_g_cost = current_node.borrow().g_cost + 10;
                if temp_g_cost > neighbour.borrow().g_cost {
                    continue; // this way would have been a worse path
                }
                let g_cost = temp_g_cost;
                let h_cost = position::euclid_distance(&current_pos, &pos);
                let f_cost = g_cost + h_cost;
                neighbour.borrow_mut().g_cost = g_cost;
                neighbour.borrow_mut().h_cost = h_cost;
                neighbour.borrow_mut().f_cost = f_cost;
                neighbour.borrow_mut().parent = Some(current_pos.clone());
                if neighbour.borrow().node_type == NodeType::Traversed {
                    continue;
                }
                neighbour.borrow_mut().node_type = NodeType::Traversed;
                open_set.push(neighbour.clone());
            }
        }
    }
}
