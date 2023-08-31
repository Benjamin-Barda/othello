use crate::bitboard::*;

type BoardState = [u64; 2];
pub struct Arena {
    nodes: Vec<Node>,
}

pub struct NodeId {
    index: usize,
}

pub struct Node {
    id: NodeId,
    parent: Option<NodeId>,
    childrens: Option<Vec<NodeId>>,
    pub data: NodeData,
}

pub struct NodeData {
    visit_count: i32,
    value: f64,
    board_state: BoardState,
}

trait MCTSTree {
    fn get_ucbs(&self, nodeid: NodeId) -> Vec<f64>;
    fn select(&self, nodeid: NodeId) -> &NodeId;
    fn expand(&mut self, nodeid: NodeId, policy: &Vec<f64>);
}

impl Arena {
    pub fn add_node(&mut self, data: NodeData, parent: NodeId) {
        let index = self.nodes.len();
        let new_node = Node {
            id: NodeId { index },
            parent: Some(parent),
            childrens: None,
            data,
        };

        self.nodes.push(new_node);
    }

    pub fn get_node(&self, id: NodeId) -> &Node {
        match self.nodes.get(id.index) {
            Some(n) => n,
            None => panic!("Invalid Node Id received"),
        }
    }
}

impl MCTSTree for Arena {
    fn get_ucbs(&self, nodeid: NodeId) -> Vec<f64> {
        let mut node = self.get_node(nodeid);
        let mut ucbs: Vec<f64> = Vec::new();

        ucbs = node
            .childrens
            .unwrap()
            .iter()
            .map(|child_index| {
                let children = self.nodes.get(child_index.index).unwrap();

                let q_value = children.data.value / (children.data.visit_count as f64);
                return q_value
                    + ((node.data.visit_count as f64).ln() / (children.data.visit_count as f64))
                        .sqrt();
            })
            .collect();

        return ucbs;
    }

    fn select(&self, nodeid: NodeId) -> &NodeId {
        let mut best_ucb = &f64::NEG_INFINITY;
        let mut best_relative_index = 0;

        self.get_ucbs(nodeid)
            .iter()
            .enumerate()
            .for_each(|(index, ucb_score)| {
                if ucb_score > best_ucb {
                    best_ucb = ucb_score;
                    best_relative_index = index;
                }
            });

        let node = self.get_node(nodeid);
        node.childrens.unwrap().get(best_relative_index).unwrap()
    }

    fn expand(&mut self, nodeid: NodeId, policy: &Vec<f64>) {
        let iter = policy.iter().enumerate().filter(|(i, prob)| *prob > &0.0);
        let node = self.get_node(nodeid);

        for (action, prob) in iter {
            let action_taken = 1 << action;
            let new_state = resolve_move(
                action_taken,
                node.data.board_state[0],
                node.data.board_state[1],
            );
            let node_data = NodeData {
                visit_count: 0,
                value: 0.0,
                board_state: resolve_move(action_taken, new_state[1], new_state[0]),
            };
            self.add_node(node_data, nodeid);
        }
    }
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        match self.childrens {
            Some(childrens) => childrens.len() == 0,
            None => true,
        }
    }
}
