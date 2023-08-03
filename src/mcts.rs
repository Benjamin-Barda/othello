use crate::bitboard::*;
use std::cell::RefCell;
use std::rc::Rc;

use tch::{Tensor, CModule, IValue, Kind}; 

type BoardState = [u64; 2];

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

    fn new(state : BoardState) -> Self {
        Node {
            state, 
            prob : 0.0, 
            children: Vec::new(), 
            parent: Vec::new(), 
            action_taken: None, 

            visit_count: 0, 
            value: 0.0
        }        
    }

    fn is_leaf(&self) -> bool {
        return self.children.is_empty();

    }

    fn get_state(&self) -> Tensor {
        let s1 = bb2tensor(self.state[0]); 
        let s2 = bb2tensor(self.state[1]); 

        Tensor::stack(&[s1, s2], 1)

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
            true => {}, 
            false => {
                self.parent.first().unwrap().take().backpropagate(0.0 - self.value);
            }
        };


    }
}



pub struct MCTS {
    model: CModule
}



impl MCTS {
    pub fn new(model_file : String) -> Self {
        let model =  match CModule::load(model_file) {
            Ok(model) => model,
            Err(_) => panic!("Failed To Load Model") ,
        };
        return MCTS {
            model
        };
    }

    fn eval(&self, state : Tensor) -> Result<IValue, tch::TchError> {
        return self.model.forward_is(&[
                IValue::Tensor(state) 
        ]);
    }

    pub fn search(&self, state: BoardState, num_searches : usize) -> Vec<u32> {
        let root = Node::new(state); 

        for _ in 0..num_searches {
            let mut node = root.clone();    

            while !node.is_leaf() {
                node = node.select().as_ref().take();
            }

            let is_game_finished =  is_game_ended(node.state[0], node.state[1]);

            let mut policy : Tensor;
            let value : f64;

            if !is_game_finished {
                let state = node.get_state(); 
                let out = self.eval(state).unwrap();
                (policy, value) = match out {
                    IValue::Tuple(ivals) => match &ivals[..] {
                        [IValue::Tensor(pol), IValue::Double(val)] =>(pol.shallow_clone(), *val as f64), 
                        _ => panic!("Something went wrong in model forward")
                    
                    }, 
                    _ => panic!("Something went wrong in model forward")
                };


                policy = policy.softmax(-1, Kind::Float);
                policy *= bb2tensor(generate_legal_moves(node.state[0], node.state[1]));
                policy /= policy.sum(Kind::Float);

                node.expand(policy
                    .iter::<f64>()
                    .unwrap()
                    .collect::<Vec<f64>>()
                    )
            } else {
                value = match game_result(node.state[0], node.state[1]) {
                    GameResult::WIN => 1.0,
                    GameResult::DRAW => 0.0,
                    GameResult::LOSS => 0.0,
                };
            }

            node.backpropagate(value)

        }
        // return action prob for each of the candidates move
        let action_probs : Vec<u32> = root.children.iter()
            .map(|child| child.as_ref().take().visit_count)
            .collect();
        return action_probs;
    }

}
