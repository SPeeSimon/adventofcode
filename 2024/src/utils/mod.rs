
pub mod grid;
pub mod position;


#[derive(Debug, Eq, PartialEq, Hash)]
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