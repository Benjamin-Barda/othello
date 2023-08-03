use crate::bitboard::*;
use std::cell::RefCell;
use std::rc::Rc;

type BoardState = [u64; 2];

#[derive(Clone)]
struct Node {
    state: BoardState,
    prob: f32,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Vec<Rc<RefCell<Node>>>,
    action_taken: Option<usize>,

    visit_count: u32,
    value: f32,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            state: [get_initial_white_bitboard(), get_initial_black_bitboard()],
            prob: 0 as f32,
            children: Vec::new(),
            parent: Vec::new(),
            action_taken: None,

            visit_count: 0,
            value: 0.0,
        }
    }
}

impl Node {

    fn is_leaf(&self) -> bool {
        return self.children.is_empty();
    
    }

    fn get_ucb(&self, child: &Rc<RefCell<Node>>) -> f32 {
        let q_value: f32;
        if child.borrow().visit_count == 0 {
            q_value = 0.0;
        } else {
            q_value = child.borrow().value / (child.borrow().visit_count as f32);
        }

        // ADD Missing hyperparameter C !!!
        return q_value
            + ((self.visit_count as f32).ln() / (child.borrow().visit_count as f32)).sqrt();
    }

    fn select(&self) -> &Rc<RefCell<Node>> {
        let mut best_ucb: f32 = f32::NEG_INFINITY;
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

    fn expand(&mut self, policy: Vec<f32>) {
        let iter = policy.iter().enumerate().filter(|(_, prob)| **prob > 0.0);
        for (action, prob) in iter {
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

    fn backpropagate(&mut self, value: f32) {
        
        self.value += value; 
        self.visit_count += 1; 
        
        match self.parent.is_empty() {
            true => {}, 
            false => {
               self.parent.first().unwrap().take().backpropagate(0.0 - self.value);
            }
        };
        

    }
}