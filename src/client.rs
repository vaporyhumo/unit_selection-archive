use sdl2::{render::Canvas, video::Window, EventPump, Sdl, VideoSubsystem};

use crate::{GAME_HEIGHT, GAME_WIDTH, WHITE};

pub struct Client {
  pub canvas: Canvas<Window>,
  pub event_pump: EventPump,
}

impl Client {
  pub fn new() -> Result<Client, String> {
    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;
    let window: Window = video_subsystem
      .window("unit selection", GAME_WIDTH as u32, GAME_HEIGHT as u32)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;
    let mut canvas: Canvas<Window> = window
      .into_canvas()
      .present_vsync()
      .build()
      .map_err(|e| e.to_string())?;
    let event_pump: EventPump = sdl_context.event_pump()?;

    canvas.set_draw_color(WHITE);
    canvas.clear();

    Ok(Client { canvas, event_pump })
  }
}
