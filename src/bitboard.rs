const A_FILE: u64 = 0x0101010101010101;
const H_FILE: u64 = 0x8080808080808080;

enum Direction {
    N,
    S,
    W,
    E,
    NW,
    NE,
    SE,
    SW,
}

pub enum GameResult {
    WIN,
    DRAW,
    LOSS,
}

fn shift(x: u64, y: isize) -> u64 {
    if y < 0 {
        x >> -y
    } else {
        x << y
    }
}

fn generate_along_dir(gen: u64, prop: u64, dir: Direction) -> u64 {
    let mut p = prop
        & match dir {
            Direction::N | Direction::S => !0,
            Direction::W | Direction::NW | Direction::SW => !H_FILE,
            Direction::E | Direction::NE | Direction::SE => !A_FILE,
        };

    let dir = match dir {
        Direction::NE => 9,
        Direction::N => 8,
        Direction::NW => 7,
        Direction::E => 1,
        Direction::W => -1,
        Direction::SE => -7,
        Direction::S => -8,
        Direction::SW => -9,
    };

    let mut g = gen | p & shift(gen, dir);
    p &= shift(p, dir);

    let mut d = dir * 2;
    g = g | p & shift(g, d);
    p &= shift(p, d);

    d *= 2;
    g = g | p & shift(g, d);
    shift(g ^ gen, dir)
}

pub fn generate_legal_moves(my_bb: u64, opp_bb: u64) -> u64 {
    let mut moves = generate_along_dir(my_bb, opp_bb, Direction::NE);
    moves |= generate_along_dir(my_bb, opp_bb, Direction::N);
    moves |= generate_along_dir(my_bb, opp_bb, Direction::NW);
    moves |= generate_along_dir(my_bb, opp_bb, Direction::E);
    moves |= generate_along_dir(my_bb, opp_bb, Direction::W);
    moves |= generate_along_dir(my_bb, opp_bb, Direction::SE);
    moves |= generate_along_dir(my_bb, opp_bb, Direction::S);
    moves |= generate_along_dir(my_bb, opp_bb, Direction::SW);

    moves & !my_bb & !opp_bb
}

fn resolve_along_dir(mov: u64, opp_bb: u64, dir: Direction) -> u64 {
    let mut p = opp_bb
        & match dir {
            Direction::N | Direction::S => !0,
            Direction::W | Direction::NW | Direction::SW => !H_FILE,
            Direction::E | Direction::NE | Direction::SE => !A_FILE,
        };

    let dir = match dir {
        Direction::NE => 9,
        Direction::N => 8,
        Direction::NW => 7,
        Direction::E => 1,
        Direction::W => -1,
        Direction::SE => -7,
        Direction::S => -8,
        Direction::SW => -9,
    };

    let mut mov = mov;
    mov |= p & shift(mov, dir);
    p &= shift(p, dir);

    let mut d = dir * 2;
    mov |= p & shift(mov, d);
    p &= shift(p, d);

    d *= 2;
    mov |= p & shift(mov, d);
    mov
}

pub fn resolve_move(mov: u64, my_bb: u64, opp_bb: u64) -> [u64; 2] {
    let mut flipped = 0u64;
    flipped |= resolve_along_dir(mov, opp_bb, Direction::NE);
    flipped |= resolve_along_dir(mov, opp_bb, Direction::N);
    flipped |= resolve_along_dir(mov, opp_bb, Direction::NW);
    flipped |= resolve_along_dir(mov, opp_bb, Direction::E);
    flipped |= resolve_along_dir(mov, opp_bb, Direction::W);
    flipped |= resolve_along_dir(mov, opp_bb, Direction::SE);
    flipped |= resolve_along_dir(mov, opp_bb, Direction::S);
    flipped |= resolve_along_dir(mov, opp_bb, Direction::SW);

    [my_bb | flipped | mov, opp_bb & !flipped]
}

pub fn get_initial_white_bitboard() -> u64 {
    0x0000001008000000
}

pub fn get_initial_black_bitboard() -> u64 {
    0x0000000810000000
}

pub fn has_legal_moves(my_bb: u64, opp_bb: u64) -> bool {
    return generate_legal_moves(my_bb, opp_bb) != 0;
}

pub fn is_game_ended(my_bb: u64, opp_bb: u64) -> bool {
    return !(has_legal_moves(my_bb, opp_bb) | has_legal_moves(opp_bb, my_bb));
}

pub fn game_result(my_bb: u64, opp_bb: u64) -> GameResult {
    let my_score = my_bb.count_ones();
    let opp_score = opp_bb.count_ones();

    if my_score > opp_score {
        return GameResult::WIN;
    } else if my_score == opp_score {
        return GameResult::DRAW;
    } else {
        return GameResult::LOSS;
    }
}

pub fn bb2vec(bb: u64) -> Vec<f64> {
    let mut bool_vector: Vec<f64> = Vec::new();
    for i in 0..64 {
        bool_vector.push(match (bb >> i) & 1 == 1 {
            true => 1.0,
            false => 0.0,
        });
    }
    return bool_vector;
}

pub fn render(w_bb: u64, b_bb: u64, legal_moves: u64) -> String {
    let mut board: String = String::new();

    board.push_str("    A B C D E F G H\n");
    board.push_str("    ---------------");

    let mut w_bb = w_bb;
    let mut b_bb = b_bb;
    let mut legal_moves = legal_moves;
    let mut col = 0;

    for i in 0..64 {
        if i % 8 == 0 {
            board.push_str("\n");
            board.push_str(&String::from(col.to_string()));
            col += 1;
            board.push_str(" | ");
        }
        let mask: u64 = 1;
        if mask & w_bb == 1 {
            board.push_str("W ");
        } else if mask & b_bb == 1 {
            board.push_str("B ");
        } else if mask & legal_moves == 1 {
            board.push_str("+ ");
        } else {
            board.push_str(". ");
        }
        w_bb = w_bb >> 1;
        b_bb = b_bb >> 1;
        legal_moves = legal_moves >> 1;
    }
    return board;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_legal_moves() {
        let w_bb: u64 = get_initial_white_bitboard();
        let b_bb: u64 = get_initial_black_bitboard();
        let res: u64 = 0x0000102004080000;

        assert_eq!(res.count_ones(), 4);
        assert_eq!(generate_legal_moves(b_bb, w_bb), res);
    }

    fn test_resolve_move() {
        let w_bb: u64 = get_initial_white_bitboard();
        let b_bb: u64 = get_initial_black_bitboard();
        let mov: u64 = 0x0000100000000000;
    }
}
