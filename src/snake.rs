use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  // return opposite of current direction
  pub fn opposite(&self) -> Direction {
    match *self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

#[derive(Debug, Clone)]
struct Block {
  x: i32,
  y: i32,
}

pub struct Snake {
  direction: Direction,
  body: LinkedList<Block>,
  tail: Option<Block>,
}

impl Snake {
  // create new instance of snake
  pub fn new(x: i32, y: i32) -> Snake {
    // Create snake body and append 3 blocks
    let mut body: LinkedList<Block> = LinkedList::new();
    body.push_back(Block {
      x: x + 2,
      y,
    });
    body.push_back(Block {
      x: x + 1,
      y,
    });
    body.push_back(Block {
      x,
      y,
    });

    // return new instance of snake
    Snake {
      direction: Direction::Right,
      body,
      tail: None,
    }
  }

  // draw snake
  pub fn draw(&self, con: &Context, g: &mut G2d) {
    // Loop through the blocks in the body and draw
    for block in &self.body {
      draw_block(SNAKE_COLOR, block.x, block.y, con, g);
    }
  }

  // Get head position
  pub fn head_position(&self) -> (i32, i32) {
    let head_block = self.body.front().unwrap();
    (head_block.x, head_block.y)
  }

  // Move 
  pub fn move_forward(&mut self, dir: Option<Direction>) {
    match dir {
      Some(d) => self.direction = d,
      None => (),
    }

    // Get last head position and move it 1 in the direction
    let (last_x, last_y): (i32, i32) = self.head_position();

    let new_block = match self.direction {
      Direction::Up => Block {
        x: last_x,
        y: last_y - 1,
      },
      Direction::Down => Block {
        x: last_x,
        y: last_y + 1,
      },
      Direction::Left => Block {
        x: last_x - 1,
        y: last_y,
      },
      Direction::Right => Block {
        x: last_x + 1,
        y: last_y,
      },
    };
    // Add a new block in the direction and remove it from the end
    self.body.push_front(new_block);
    let removed_block = self.body.pop_back().unwrap();
    self.tail = Some(removed_block);
  }

  // Return direction of the snake
  pub fn head_direction(&self) -> Direction {
    self.direction
  }

  // return next position of the snake
  pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
    // get current postion
    let (head_x, head_y): (i32, i32) = self.head_position();

    // set current direction, change if a new direction given
    let mut moving_dir = self.direction;
    match dir {
      Some(d) => moving_dir = d,
      None => {}
    }

    // return new position based on the direction
    match moving_dir {
      Direction::Up => (head_x, head_y - 1),
      Direction::Down => (head_x, head_y + 1),
      Direction::Left => (head_x - 1, head_y),
      Direction::Right => (head_x + 1, head_y),
    }
  }

  // Remove block from tail and add to the front
  pub fn restore_tail(&mut self) {
    let block = self.tail.clone().unwrap();
    self.body.push_back(block);
  }

  // check if snake is overlapping
  pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
    let mut ch = 0;
    // TODO:
    for block in &self.body {
      if x == block.x && y == block.y {
        return true;
      }

      ch += 1;
      if ch == self.body.len() - 1 {
        break;
      }
    }
    return false;
  }
}