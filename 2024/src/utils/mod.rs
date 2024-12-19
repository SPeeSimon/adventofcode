use std::ops::{Index, IndexMut};


pub mod grid;
pub mod position;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Move {
    TOP,
    TOP_RIGHT,
    RIGHT,
    DOWN_RIGHT,
    DOWN,
    DOWN_LEFT,
    LEFT,
    TOP_LEFT,
}


impl Move {

    pub fn from_char(movement: &char) -> Option<Move> {
        match movement {
            '^' => Some(Move::TOP),
            '>' => Some(Move::RIGHT),
            'v' => Some(Move::DOWN),
            '<' => Some(Move::LEFT),
            _ => None
        }
    }


    pub fn direct_sides() -> [Move;4] {
        [Move::TOP, Move::RIGHT, Move::DOWN, Move::LEFT]
    }


    pub fn move_90(&self) -> Move {
        match self {
            Move::TOP => Move::RIGHT,
            Move::TOP_RIGHT => Move::DOWN_RIGHT,
            Move::RIGHT => Move::DOWN,
            Move::DOWN_RIGHT => Move::DOWN_LEFT,
            Move::DOWN => Move::LEFT,
            Move::DOWN_LEFT => Move::TOP_LEFT,
            Move::LEFT => Move::TOP,
            Move::TOP_LEFT => Move::TOP_RIGHT,
        }
    }
    

    pub fn turn_left(&self) -> Move {
        match self {
            Move::TOP => Move::LEFT,
            Move::TOP_RIGHT => Move::TOP_LEFT,
            Move::RIGHT => Move::TOP,
            Move::DOWN_RIGHT => Move::TOP_RIGHT,
            Move::DOWN => Move::RIGHT,
            Move::DOWN_LEFT => Move::DOWN_RIGHT,
            Move::LEFT => Move::DOWN,
            Move::TOP_LEFT => Move::DOWN_LEFT,
        }
    }


    fn move_x(&self, x: usize, amount: usize) -> Option<usize> {
        match *self {
            Move::TOP => Some(x),
            Move::DOWN => Some(x),
            Move::RIGHT => Some(x + amount),
            Move::TOP_RIGHT => Some(x + amount),
            Move::DOWN_RIGHT => Some(x + amount),
            Move::DOWN_LEFT => x.checked_sub(amount),
            Move::TOP_LEFT => x.checked_sub(amount),
            Move::LEFT => x.checked_sub(amount),
        }
    }

    fn move_y(&self, y: usize, amount: usize) -> Option<usize> {
        match *self {
            Move::TOP => y.checked_sub(amount),
            Move::TOP_RIGHT => y.checked_sub(amount),
            Move::TOP_LEFT => y.checked_sub(amount),
            Move::DOWN_RIGHT => Some(y + amount),
            Move::DOWN_LEFT => Some(y + amount),
            Move::DOWN => Some(y + amount),
            Move::RIGHT => Some(y),
            Move::LEFT => Some(y),
        }
    }

}


impl<T> Index<Move> for Vec<T> {
    type Output = T;

    fn index(&self, mv: Move) -> &Self::Output {
        let i = mv as usize;
        &self[i]
    }
}

impl<T> Index<Move> for [T;8] {
    type Output = T;

    fn index(&self, mv: Move) -> &Self::Output {
        let i = mv as usize;
        &self[i]
    }
}

impl<T> Index<Move> for [T;4] {
    type Output = T;

    fn index(&self, mv: Move) -> &Self::Output {
        match mv {
            Move::TOP => &self[0],
            Move::RIGHT => &self[1],
            Move::DOWN => &self[2],
            Move::LEFT => &self[3],
            Move::TOP_RIGHT => unimplemented!(),
            Move::TOP_LEFT => unimplemented!(),
            Move::DOWN_RIGHT => unimplemented!(),
            Move::DOWN_LEFT => unimplemented!(),
        }
    }
}

impl<T> IndexMut<Move> for [T;4] {
    // type Output = T;

    fn index_mut(&mut self, mv: Move) -> &mut Self::Output {
        match mv {
            Move::TOP => &mut self[0],
            Move::RIGHT => &mut self[1],
            Move::DOWN => &mut self[2],
            Move::LEFT => &mut self[3],
            Move::TOP_RIGHT => unimplemented!(),
            Move::TOP_LEFT => unimplemented!(),
            Move::DOWN_RIGHT => unimplemented!(),
            Move::DOWN_LEFT => unimplemented!(),
        }
    }
}
