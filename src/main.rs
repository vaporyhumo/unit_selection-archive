mod client;
use std::time::Duration;

use client::Client;
use sdl2::{pixels::Color, keyboard::Keycode, event::Event, mouse::MouseButton};

const GAME_WIDTH: usize = 800;
const GAME_HEIGHT: usize = 600;
const UNIT_SIZE: usize = 20;

const WHITE: Color = Color::RGBA(255, 255, 255, 255);
const BLACK: Color = Color::RGBA(0, 0, 0, 255);
const RED: Color = Color::RGBA(100, 0, 0, 255);

struct Unit {
  pos: (usize, usize),
  selected: bool,
}

impl Unit {
  fn tick(self) -> Self {
    Unit { ..self }
  }
}

// function that takes the position of a unit and the position of a click
// and determines whether the unit was clicked
fn unit_clicked(unit_pos: (usize, usize), click_pos: (i32, i32)) -> bool {
  let (x, y): (i32, i32) = (unit_pos.0 as i32, unit_pos.1 as i32);
  let (w, h): (u32, u32) = (UNIT_SIZE as u32, UNIT_SIZE as u32);
  let (click_x, click_y): (i32, i32) = click_pos;
  click_x >= x && click_x <= x + w as i32 && click_y >= y && click_y <= y + h as i32
}

fn main() -> Result<(), String> {
  let mut client: Client = Client::new()?;
  let mut unit: Unit = Unit {
    pos: (0, 0),
    selected: false,
  };
  let mut left_down_pos: (i32, i32) = (0, 0);
  let mut left_click_down: bool = false;
  let mut left_up_pos: (i32, i32) = (0, 0);

  draw_unit(&mut client, &unit)?;
  client.canvas.present();

  'mainloop: loop {
    client.canvas.set_draw_color(WHITE);
    client.canvas.clear();

    for event in client.event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'mainloop,
        Event::MouseButtonDown { mouse_btn, x, y, .. } => {
          if mouse_btn == MouseButton::Left {
            left_down_pos = (x, y);
            left_click_down = true;
          }
        },
        Event::MouseButtonUp { mouse_btn, x, y, .. } => {
          if mouse_btn == MouseButton::Left {
            left_up_pos = (x, y);
            left_click_down = false;

            if unit_clicked(unit.pos, left_up_pos) {
              unit.selected = true;
            } else {
              unit.selected = false;
            }
          }
        },
        _ => {},
      }
    }

    unit = unit.tick();

    draw_unit(&mut client, &unit)?;

    client.canvas.present();
    std::thread::sleep(Duration::from_millis(100));
  }
  Ok(())
}

fn draw_unit(client: &mut Client, unit: &Unit) -> Result<(), String> {
  let (x, y): (i32, i32) = (unit.pos.0 as i32, unit.pos.1 as i32);
  let (w, h): (u32, u32) = (UNIT_SIZE as u32, UNIT_SIZE as u32);
  if unit.selected {
    client.canvas.set_draw_color(RED);
  } else {
    client.canvas.set_draw_color(BLACK);
  }
  client.canvas.fill_rect(sdl2::rect::Rect::new(
    x as i32, y as i32, w as u32, h as u32,
  ))?;

  Ok(())
}
