#![feature(stmt_expr_attributes)]
pub mod frame_history;
pub mod node;
mod position;
#[cfg(test)]
mod test;

use crate::node::{Node, NodeType};
use crate::position::Position;
use std::cell::{Ref, RefCell};
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
pub enum Heuristic {
    Manhattan,
    Diagonal,
    Euclidean,
}

pub struct Grid {
    height: usize,
    width: usize,
    nodes: HashMap<Position, Rc<RefCell<Node>>>,
    goal: Option<Position>,
    start: Option<Position>,
    pub allow_diagonal: bool,
    path: Option<Vec<Position>>,
    pub duration: Option<Duration>,
}

const OFFSETS: [Position; 4] = [
    Position::new(1, 0),  // right
    Position::new(-1, 0), // left
    Position::new(0, -1), // top
    Position::new(0, 1),  // bottom
];

const DIAG_OFFSETS: [Position; 4] = [
    Position::new(1, -1),  // top right
    Position::new(-1, -1), // top left
    Position::new(1, 1),   // bottom right
    Position::new(-1, 1),  // bottom left
];

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let mut nodes = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let pos = Position::new(x as i32, y as i32);
                let node = Node {
                    index: width * y + x,
                    ..Default::default()
                };
                nodes.insert(pos, Rc::new(RefCell::new(node)));
            }
        }
        Grid {
            height,
            width,
            nodes,
            goal: None,
            start: None,
            allow_diagonal: true,
            path: None,
            duration: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.goal.is_some() && self.start.is_some()
    }

    pub fn get_node_at(&self, x: usize, y: usize) -> Ref<Node> {
        let pos = Position::new(x as i32, y as i32);
        self.nodes
            .get(&pos)
            .unwrap_or_else(|| panic!("{:?} is invalid", pos))
            .borrow()
    }

    fn is_valid_pos(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    fn get_neighbours(&self, me: &Position) -> Vec<Position> {
        OFFSETS
            .iter()
            .map(|offset| me + offset)
            .filter(|pos| self.is_valid_pos(pos))
            .collect()
    }
    fn get_neighbours_diag(&self, me: &Position) -> Vec<Position> {
        DIAG_OFFSETS
            .iter()
            .map(|offset| me + offset)
            .filter(|pos| self.is_valid_pos(pos))
            .collect()
    }
    // returns the adjacent neighbours with cost(10)
    fn get_neighbours_cost(&self, me: &Position) -> Vec<(Position, usize)> {
        self.get_neighbours(me)
            .iter()
            .map(|pos| (pos.clone(), 10))
            .collect()
    }
    // returns the adjacent and diagonal neighbours with cost (14)
    fn get_neighbours_diag_cost(&self, me: &Position) -> Vec<(Position, usize)> {
        let mut diag_cost: Vec<(Position, usize)> = self
            .get_neighbours_diag(me)
            .iter()
            .map(|pos| (pos.clone(), 14))
            .collect();
        let mut adjacent_cost = self.get_neighbours_cost(me);
        adjacent_cost.append(&mut diag_cost);
        adjacent_cost
    }

    fn get_index_from_pos(&self, pos: &Position) -> usize {
        self.width * (pos.y as usize) + (pos.x as usize)
    }

    fn get_pos_from_index(&self, idx: usize) -> Position {
        let row = idx / self.width;
        let col = idx % self.width;
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

    fn trace_path(&mut self, position: Position) {
        let mut path_pos: Vec<Position> = Vec::new();
        path_pos.push(position.clone());
        let mut current_pos = position;
        while let Some(parent_node) = self.nodes.get(&current_pos) {
            if let Some(parent_pos) = parent_node.borrow().parent.clone() {
                current_pos = parent_pos;
                path_pos.push(current_pos.clone());
            } else {
                break;
            }
        }
        self.path = Some(path_pos);
    }

    pub fn solve(&mut self, heuristic: &Heuristic) {
        let start_pos = self.start.clone();
        let Some(start_pos) = start_pos else {
            panic!("no start position");
        };
        let goal_pos = self.goal.clone();
        let Some(goal_pos) = goal_pos else {
            panic!("no goal position")
        };

        let heuristic: fn(&Position, &Position) -> usize = match heuristic {
            Heuristic::Manhattan => position::manhattan_distance,
            Heuristic::Euclidean => position::euclidean_distance,
            Heuristic::Diagonal => position::diagonal_distance,
        };

        let goal = self.get_index_from_pos(&goal_pos);

        let mut open_set = BinaryHeap::new();
        let start_node = self
            .nodes
            .get(&start_pos)
            .expect("must be valid start position");
        let start_h_cost = heuristic(&start_pos, &goal_pos);
        start_node.borrow_mut().h_cost = start_h_cost;
        start_node.borrow_mut().g_cost = 0;
        start_node.borrow_mut().f_cost = start_h_cost;

        // start_node.node_type = NodeType::Traversed;
        open_set.push(start_node.clone());

        #[cfg(not(target_arch = "wasm32"))]
        let now = Instant::now();

        while let Some(current_node) = open_set.pop() {
            #[cfg(not(target_arch = "wasm32"))]
            let duration = Some(now.elapsed());
            #[cfg(target_arch = "wasm32")]
            let duration = None;
            self.duration = duration;

            if current_node.borrow().index == goal {
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
            let neighbours = if self.allow_diagonal {
                self.get_neighbours_diag_cost(&current_pos)
            } else {
                self.get_neighbours_cost(&current_pos)
            };
            for (pos, cost) in neighbours {
                let neighbour = self.nodes.get(&pos).expect("invalid position");
                if neighbour.borrow().node_type == NodeType::Obstacle {
                    continue;
                }
                let temp_g_cost = current_node.borrow().g_cost + cost;
                if temp_g_cost > neighbour.borrow().g_cost {
                    continue; // this way would have been a worse path
                }
                let g_cost = temp_g_cost;
                let h_cost = heuristic(&pos, &goal_pos);
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
        if let Some(path) = &self.path {
            // set all the nodes to path
            path.iter().for_each(|pos| {
                self.nodes
                    .get(pos)
                    .expect("node should exist")
                    .borrow_mut()
                    .node_type = NodeType::Path;
            })
        }
    }
}
