use rand::random;
use sdl2::{
  event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, EventPump,
};

const CELL_SIZE: u32 = 2;
const GRID_SIZE: u32 = 350;

struct Grid {
  canvas: WindowCanvas,
  event_pump: EventPump,
  body: Vec<Vec<bool>>,
  w: u32,
  h: u32,
  is_running: bool,
  is_updating: bool,
}

fn make_grid(_w: u32, _h: u32) -> Vec<Vec<bool>> {
  (0.._h)
    .map(|_| (0.._w).map(|_| random::<u8>() < 64).collect())
    .collect::<Vec<Vec<bool>>>()
}

impl Grid {
  fn generate(_w: u32, _h: u32) -> Grid {
    let gl = sdl2::init().unwrap();
    let vid = gl.video().unwrap();
    let window = vid
      .window("Convay's GoL", _w as u32 * CELL_SIZE, _h as u32 * CELL_SIZE)
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
  fn update(&mut self) {
    let h = self.w as usize - 2;
    let w = self.h as usize - 2;
    let mut _b = self.body.clone();
    (0..self.w).for_each(|y| {
      (0..self.h).for_each(|x| {
        let b = &mut self.body;
        let x = x as usize;
        let y = y as usize;
        let mut nbs: u8 = 0;
        if x > 0 {
          nbs += b[x - 1][y] as u8;
        }
        if x < w {
          nbs += b[x + 1][y] as u8;
        }
        if y > 0 {
          nbs += b[x][y - 1] as u8;
        }
        if y < h {
          nbs += b[x][y + 1] as u8;
        }

        if x > 0 && y > 0 {
          nbs += b[x - 1][y - 1] as u8;
        }
        if x > 0 && y < h {
          nbs += b[x - 1][y + 1] as u8;
        }
        if x < w && y > 0 {
          nbs += b[x + 1][y - 1] as u8;
        }
        if x < w && y < h {
          nbs += b[x + 1][y + 1] as u8;
        }

        if nbs < 2 || nbs > 3 {
          _b[x][y] = false;
        }
        if !b[x][y] && nbs == 3 {
          _b[x][y] = true;
        }
      })
    });
    self.body = _b;
  }
  fn draw(&mut self) {
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
    let mut cell_rects: Vec<Rect> = Vec::with_capacity((self.w * self.h) as usize);
    (0..self.w).for_each(|x| {
      (0..self.h).for_each(|y| {
        if self.body[y as usize][x as usize] {
          cell_rects.push(Rect::new(
            x as i32 * CELL_SIZE as i32,
            y as i32 * CELL_SIZE as i32,
            CELL_SIZE,
            CELL_SIZE,
          ));
        }
      })
    });
    self.canvas.fill_rects(cell_rects.as_slice()).unwrap();
    self.canvas.present();
  }
  fn collect_events(&mut self) {
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
          R => {
            self.body = make_grid(self.w, self.h);
          }
          _ => (),
        },
        _ => (),
      }
    }
  }
  //  fn scale(&mut self) {
  //    let bsize = self.w * self.h;
  //    let mut res: Vec<Vec<CellColor>> = Vec::with_capacity(bsize as usize);
  //    res.push(Vec::with_capacity(self.w as usize));
  //    res.
  //  }
}

pub fn init() {
  let mut game = Grid::generate(GRID_SIZE, GRID_SIZE);
  loop {
    if game.is_updating {
      game.update();
    }
    game.draw();
    std::thread::sleep(std::time::Duration::from_millis(1000 / 50));
    game.collect_events();

    if !game.is_running {
      break;
    }
  }
}
