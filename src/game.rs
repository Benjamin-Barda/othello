use pyo3::prelude::*; 
use crate::bitboard::*;

enum Turn {
    W, 
    B
}

#[pyclass]
pub struct Othello {
    white_bb : u64, 
    black_bb : u64, 
    turn : Turn

}


#[pymethods]
impl Othello {

    #[new]
    fn new() -> Self {
        Othello {
            white_bb : get_initial_white_bitboard(), 
            black_bb : get_initial_black_bitboard(),
            turn : Turn::B
        }
    }

    fn get_white_bb(&self) -> PyResult<Vec<bool>>{
        self.bb2vec(self.white_bb)

    }

    fn get_black_bb(&self) -> PyResult<Vec<bool>>{
        self.bb2vec(self.black_bb)

    }

    fn bb2vec(&self, bb : u64) -> PyResult<Vec<bool>> {
        let mut vec_bb = vec![];
        for i in 0..64 {
            match  (bb >> i) & 1 == 0 {
                true => vec_bb.push(false), 
                false => vec_bb.push(true)
            }; 

        }

        Ok(vec_bb)
    }

    fn vec2bb(&self, vector : Vec<bool>) -> PyResult<u64>{
        let mut bb : u64 = 0;
        vector
            .iter()
            .for_each(|bit| {
                match bit {
                    true => {bb &= 1; bb = bb << 1;} 
                    false => {bb = bb << 1;}

                }
            });
        Ok(bb)
    }

    fn get_legal_moves(&self) -> PyResult<Vec<bool>> {
        match self.turn {
            Turn::B => self.bb2vec(generate_legal_moves(self.black_bb, self.white_bb)), 
            Turn::W => self.bb2vec(generate_legal_moves(self.white_bb, self.black_bb))
        }
    }

    fn has_legal_moves(&self) -> PyResult<bool> {
        Ok(self.get_legal_moves().unwrap().contains(&true))
    }

    fn make_move(&mut self, selected_move : Vec<bool>) {
        match self.turn {
            Turn::B => {
                resolve_move(self.vec2bb(selected_move).unwrap(), self.black_bb, self.white_bb);
                self.turn = Turn::W;
            },
            Turn::W => {
                resolve_move(self.vec2bb(selected_move).unwrap(), self.white_bb, self.black_bb);
                self.turn = Turn::B;
            }
        };
    }

    fn pass_turn(&mut self) {
        match self.turn {
            Turn::B => {self.turn = Turn::W;},
            Turn::W => {self.turn = Turn::B;}
        }
    }

    fn is_game_ended(&mut self) -> PyResult<bool> {
        match self.has_legal_moves().unwrap() {
            true => Ok(false), 
            false => {
                self.pass_turn(); 
                let is_ended = !self.has_legal_moves().unwrap();

                // Pass turn back
                self.pass_turn();
                Ok(is_ended)
            }
        }
    }


    fn __str__(&self) -> String  {
        render(self.white_bb, self.black_bb, 0u64)
    }
}

