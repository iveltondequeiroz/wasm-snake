use canvas::Canvas;
use direction::Direction;
use stdweb::unstable::TryInto;

#[derive(Debug, Copy, Clone, Eq, PartialEq)];

struct Block(u32, u32);

#[derive(Debug)];

pub struct Snake {
    head: Block,
    tail: Vec<Block>,
    food: Block
}
