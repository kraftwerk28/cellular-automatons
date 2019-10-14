use sdl2::{
  event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, EventPump,
};
use std::time::Duration;
use std::{
  sync::{Arc, Mutex},
  thread,
};

const CELL_SIZE: u32 = 8;
const GRID_SIZE: u32 = 75;

#[derive(Copy, Clone)]
pub enum Direction {
  N,
  E,
  S,
  W,
}

pub enum CellColor {
  None,
  Red,
  White,
}

pub enum Turn {
  Left,
  Right,
}

fn apply_dir(d: Direction, t: Turn) -> Direction {
  use Direction::*;
  use Turn::*;
  match t {
    Left => match d {
      N => W,
      E => N,
      S => E,
      W => S,
    },
    Right => match d {
      N => E,
      E => S,
      S => W,
      W => N,
    },
  }
}

pub struct Ant {
  pub grid: Grid,
  pub direction: Direction,
  pub pos: (u32, u32),
}

impl Ant {
  pub fn new() -> Ant {
    Ant {
      pos: (GRID_SIZE / 2, GRID_SIZE / 2),
      grid: Grid::generate(GRID_SIZE, GRID_SIZE),
      direction: Direction::N,
    }
  }
  pub fn tick(&mut self) -> () {
    use CellColor::*;
    use Direction::*;
    match self.grid.get_cell(self.pos) {
      None => {
        self.grid.paint(self.pos, CellColor::Red);
        self.direction = apply_dir(self.direction, Turn::Left);
      }
      White => {
        self.grid.paint(self.pos, CellColor::Red);
        self.direction = apply_dir(self.direction, Turn::Left);
      }
      Red => {
        self.grid.paint(self.pos, CellColor::White);
        self.direction = apply_dir(self.direction, Turn::Right);
      }
    }
    match self.direction {
      N => self.pos.1 -= 1,
      E => self.pos.0 += 1,
      S => self.pos.1 += 1,
      W => self.pos.0 -= 1,
    }
  }
}

pub struct Grid {
  canvas: WindowCanvas,
  event_pump: EventPump,
  body: Vec<Vec<CellColor>>,
  w: u32,
  h: u32,
  pub is_running: bool,
  pub is_updating: bool,
}

fn make_grid(_w: u32, _h: u32) -> Vec<Vec<CellColor>> {
  (0.._h)
    .map(|_| (0.._w).map(|_| CellColor::None).collect())
    .collect::<Vec<Vec<CellColor>>>()
}

impl Grid {
  fn generate(_w: u32, _h: u32) -> Grid {
    let gl = sdl2::init().unwrap();
    let vid = gl.video().unwrap();
    let window = vid
      .window(
        "Ant automation",
        _w as u32 * CELL_SIZE,
        _h as u32 * CELL_SIZE,
      )
      .build()
      .unwrap();

    Grid {
      canvas: window.into_canvas().build().unwrap(),
      event_pump: gl.event_pump().unwrap(),
      body: make_grid(_w, _h),
      w: _w,
      h: _h,
      is_running: true,
      is_updating: false,
    }
  }
  fn get_cell(&self, coord: (u32, u32)) -> &CellColor {
    &self.body[coord.1 as usize][coord.0 as usize]
  }
  fn paint(&mut self, coord: (u32, u32), cl: CellColor) -> () {
    self.body[coord.1 as usize][coord.0 as usize] = cl;
  }
  pub fn draw(&mut self) {
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();

    let gr_size = self.w as usize * self.h as usize;
    let mut red_rects = Vec::with_capacity(gr_size);
    let mut white_rects = Vec::with_capacity(gr_size);

    (0usize..self.w as usize).for_each(|x| {
      (0usize..self.h as usize).for_each(|y| {
        let rect = Rect::new(
          x as i32 * CELL_SIZE as i32,
          y as i32 * CELL_SIZE as i32,
          CELL_SIZE,
          CELL_SIZE,
        );
        match self.body[y][x] {
          CellColor::Red => red_rects.push(rect),
          CellColor::White => white_rects.push(rect),
          _ => {}
        }
      })
    });

    let err_msg = "Failed to draw rectangle";
    self.canvas.set_draw_color(Color::RGB(255, 0, 0));
    self.canvas.fill_rects(red_rects.as_slice()).expect(err_msg);
    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
    self
      .canvas
      .fill_rects(white_rects.as_slice())
      .expect(err_msg);
    self.canvas.present();
  }
  pub fn collect_events(&mut self) {
    for evt in self.event_pump.poll_iter() {
      use Keycode::*;
      match evt {
        Event::Quit { .. } => {
          self.is_running = false;
        }
        Event::KeyDown { keycode, .. } => match keycode.unwrap() {
          Q | Escape => {
            self.is_running = false;
          }
          Space => {
            self.is_updating = !self.is_updating;
          }
          _ => (),
        },
        _ => (),
      }
    }
  }
}

pub fn init() {
  let cnt = Arc::new(Mutex::new(0));

  let cnt_ = cnt.clone();
  thread::spawn(move || loop {
    thread::sleep(Duration::from_millis(1000));
    let mut cnt = cnt_.lock().unwrap();
    println!("{}", *cnt);
    *cnt = 0;
  });

  let mut ant = Ant::new();
  loop {
    if ant.grid.is_updating {
      ant.tick();
    }
    ant.grid.draw();
    ant.grid.collect_events();
    let mut cnt = cnt.lock().unwrap();
    *cnt += 1;
    thread::sleep(Duration::from_millis(1000 / 50));
    if !ant.grid.is_running {
      break;
    }
  }
}
