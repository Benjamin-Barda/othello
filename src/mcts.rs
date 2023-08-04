use crate::bitboard::*;
use std::cell::RefCell;
use std::io::prelude::*;
use std::net::TcpStream;
use std::rc::Rc;

type BoardState = [u64; 2];
type Payload = [u8; 16];

#[derive(Clone)]
struct Node {
    state: BoardState,
    prob: f64,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Vec<Rc<RefCell<Node>>>,
    action_taken: Option<usize>,

    visit_count: u32,
    value: f64,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            state: [get_initial_black_bitboard(), get_initial_white_bitboard()],
            prob: 0.0,
            children: Vec::new(),
            parent: Vec::new(),
            action_taken: None,

            visit_count: 0,
            value: 0.0,
        }
    }
}

impl Node {
    fn new(state: BoardState) -> Self {
        Node {
            state,
            prob: 0.0,
            children: Vec::new(),
            parent: Vec::new(),
            action_taken: None,

            visit_count: 0,
            value: 0.0,
        }
    }

    fn is_leaf(&self) -> bool {
        return self.children.is_empty();
    }

    fn get_payload_to_send(&self) -> [u8; 16] {
        let s1 = self.state[0].to_ne_bytes();
        let s2 = self.state[1].to_ne_bytes();

        let mut out: [u8; 16] = [0; 16];

        s1.iter().chain(s2.iter()).enumerate().for_each(|(i, b)| {
            out[i] = *b;
        });

        return out;
    }

    fn get_ucb(&self, child: &Rc<RefCell<Node>>) -> f64 {
        let q_value: f64;
        if child.borrow().visit_count == 0 {
            q_value = 0.0;
        } else {
            q_value = child.borrow().value / (child.borrow().visit_count as f64);
        }

        // ADD Missing hyperparameter C !!!
        return q_value
            + ((self.visit_count as f64).ln() / (child.borrow().visit_count as f64)).sqrt();
    }

    fn select(&self) -> &Rc<RefCell<Node>> {
        let mut best_ucb: f64 = f64::NEG_INFINITY;
        let mut best_child_index: usize = 0;

        self.children
            .iter()
            .map(|child| self.get_ucb(child))
            .enumerate()
            .for_each(|(i, ucb)| {
                if ucb > best_ucb {
                    best_ucb = ucb;
                    best_child_index = i;
                }
            });
        return self.children.get(best_child_index).unwrap();
    }

    fn expand(&mut self, policy: Vec<f64>) {
        let iter = policy.iter().enumerate().filter(|(_, prob)| **prob > 0.0);
        for (action, prob) in iter {
            println!("{}", action);
            let i_action: u64 = 1 << action;
            let new_state: BoardState = resolve_move(i_action, self.state[0], self.state[1]);
            self.children.push(Rc::new(RefCell::new(Node {
                state: [new_state[1], new_state[0]],
                prob: *prob,
                children: Vec::new(),
                parent: vec![Rc::new(RefCell::new(self.clone()))],
                action_taken: Some(action),

                value: 0.0,
                visit_count: 0,
            })));
        }
    }

    fn backpropagate(&mut self, value: f64) {
        self.value += value;
        self.visit_count += 1;

        match self.parent.is_empty() {
            true => {}
            false => {
                self.parent
                    .first()
                    .unwrap()
                    .take()
                    .backpropagate(0.0 - self.value);
            }
        };
    }
}

pub struct MCTS {
    pub stream: TcpStream,
}

impl MCTS {
    fn eval(&mut self, state: Payload) -> (Vec<f64>, f64) {
        self.stream.write(&state).unwrap();
        return (vec![0.0; 63], 0.0);
    }

    pub fn search(&mut self, state: BoardState, num_searches: usize) -> Vec<u32> {
        let mut root = Node::new(state);

        for _ in 0..num_searches {
            let mut node = root.clone();

            while !node.is_leaf() {
                node = node.select().as_ref().take();
            }

            let is_game_finished = is_game_ended(node.state[0], node.state[1]);

            let policy: Vec<f64>;
            let value: f64;

            if !is_game_finished {
                let state = node.get_payload_to_send();
                let ev = self.eval(state);
                (policy, value) = ev;

                node.expand(policy);
            } else {
                value = match game_result(node.state[0], node.state[1]) {
                    GameResult::WIN => 1.0,
                    GameResult::DRAW => 0.0,
                    GameResult::LOSS => 0.0,
                };
            }

            node.backpropagate(value);
            root = node;
        }

        // return action prob for each of the candidates move
        let action_probs: Vec<u32> = root
            .children
            .iter()
            .map(|child| child.as_ref().take().visit_count)
            .collect();

        println!("{}", root.children.is_empty());
        return action_probs;
    }
}
