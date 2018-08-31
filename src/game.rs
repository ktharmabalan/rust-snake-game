// Import piston
use piston_window::*;
use piston_window::types::Color;
// Import random number generator
use rand::{thread_rng, Rng};
// Import snake/draw
use snake::{Direction, Snake};
use draw::{draw_block, draw_rectangle};

// Colors for food/border/gameover
const FOOD_COLOR : Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR : Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR : Color = [0.90, 0.00, 0.00, 0.5];
// Moving period/restart time
const MOVING_PERIOD: f64 = 0.1; // lower for faster update, greater for slower update
const RESTART_TIME: f64 = 1.0;  // wait time in seconds

// Struct for game
pub struct Game {
  snake: Snake,
  food_exists: bool,
  food_x: i32,
  food_y: i32,
  width: i32,
  height: i32,
  game_over: bool,
  waiting_time: f64,
}

impl Game {
  // Associative method to create a new game
  pub fn new(width: i32, height: i32) -> Game {
    Game {
      snake: Snake::new(2, 2),
      waiting_time: 0.0,
      food_exists: true,
      food_x: 6,
      food_y: 4,
      width,
      height,
      game_over: false,
    }
  }

  // Handle key press
  pub fn key_pressed(&mut self, key: Key) {
    if self.game_over {
      return;
    }

    let dir = match key {
      Key::Up => Some(Direction::Up),
      Key::Down => Some(Direction::Down),
      Key::Left => Some(Direction::Left),
      Key::Right => Some(Direction::Right),
      _ => None
    };

    if dir.unwrap() == self.snake.head_direction().opposite() {
      return;
    }

    // Update snake
    self.update_snake(dir);
  }

  // draw snake, food, border, and game over screen
  pub fn draw(&self, con: &Context, g: &mut G2d) {
    // draw snake
    self.snake.draw(con, g);

    // draw food
    if self.food_exists {
      draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
    }

    // draw borders
    draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
    draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
    draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
    draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

    // draw game over
    if self.game_over {
      draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
    }
  }

  // Update
  pub fn update(&mut self, delta_time: f64) {
    self.waiting_time += delta_time;
    // restart game if game over and wait time is over
    if self.game_over {
      if self.waiting_time > RESTART_TIME {
        self.restart();
      }
      return;
    }

    // add food if none exist
    if !self.food_exists {
      self.add_food();
    }

    // update snake if update time is greater than moving preiod
    if self.waiting_time > MOVING_PERIOD {
      self.update_snake(None);
    }
  }

  // Check if snake is eating
  fn check_eating(&mut self) {
    // Get snake head position and see if it matches the food position
    let (head_x, head_y) : (i32, i32) = self.snake.head_position();
    if self.food_exists && self.food_x == head_x && self.food_y == head_y {
      // remove food
      self.food_exists = false;
      // add a block to snake
      self.snake.restore_tail();
    }
  }

  // Check if snake is alive
  fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
    // Get next position
    let (next_x, next_y) = self.snake.next_head(dir);

    // check if snake overlaps tail
    if self.snake.overlap_tail(next_x, next_y) {
      return false;
    }
    // return next position
    next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
  }

  // Add food
  fn add_food(&mut self) {
    // thread random number generator
    let mut rng = thread_rng();
    // generate new x and y
    let mut new_x = rng.gen_range(1, self.width - 1);
    let mut new_y = rng.gen_range(1, self.height - 1);
    // generate next x and y until it doesn't overlap the snake
    while self.snake.overlap_tail(new_x, new_y) {
      new_x = rng.gen_range(1, self.width - 1);
      new_y = rng.gen_range(1, self.height - 1);
    }
    // set new food
    self.food_x = new_x;
    self.food_y = new_y;
    self.food_exists = true;
  }

  // Update snake
  fn update_snake(&mut self, dir: Option<Direction>) {
    // move snake if its alive and check if it is eating
    if self.check_if_snake_alive(dir) {
      self.snake.move_forward(dir);
      self.check_eating();
    } else {
      // set game over if snake is dead
      self.game_over = true;
    }
    // reset wait time
    self.waiting_time = 0.0;
  }

  // Restart game
  fn restart(&mut self) {
    self.snake = Snake::new(2, 2);
    self.waiting_time = 0.0;
    self.food_exists = true;
    self.food_x = 6;
    self.food_y = 4;
    self.game_over = false;
  }
}