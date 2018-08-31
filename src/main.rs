// https://www.youtube.com/watch?v=DnT_7M7L7vo
extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
  let (width, height) = (20, 20);

  // Create new window, exit on 'Esc'
  let mut window: PistonWindow = WindowSettings::new(
    "Snake",
    [to_coord_u32(width), to_coord_u32(height)],
  )
  .exit_on_esc(true)
  .build()
  .unwrap();

  // Create new game
  let mut game = Game::new(width, height);
  // Event loop
  while let Some(event) = window.next() {
    // passed pressed key to game
    if let Some(Button::Keyboard(key)) = event.press_args() {
      game.key_pressed(key);
    }
    // Clear screen and draw scene
    // | | are anonymous functions
    window.draw_2d(&event, |c, g| {
      clear(BACK_COLOR, g);
      game.draw(&c, g);
    });

    // Update game
    event.update(|arg| {
      game.update(arg.dt);
    });
  }
}