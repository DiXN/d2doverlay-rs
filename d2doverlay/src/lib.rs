use d2doverlay_sys::{overlay_setup_with_process, overlay_options, draw_ellipse, draw_circle, draw_box, draw_line, draw_string};

use std::ffi::CString;
use std::sync::mpsc::{Sender, Receiver, channel};
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
enum Draw {
  Box(f32, f32, f32, f32),
  Line(f32, f32, f32, f32),
  Ellipse(f32, f32, f32, f32),
  Circle(f32, f32, f32),
  Text(String, f32, f32, f32)
}

#[derive(Clone)]
pub struct D2dOverlay {
  thickness: f32,
  r: f32,
  g: f32,
  b: f32,
  a: f32,
  filled : bool,
  process: Option<String>,
  window: Option<u32>
}

lazy_static! {
  static ref CHANNEL: (Mutex<Sender<(D2dOverlay, Draw)>>,  Mutex<Receiver<(D2dOverlay, Draw)>>) = {
    let (tx, rx) = channel();
    (Mutex::new(tx), Mutex::new(rx))
  };
}

extern "C" fn draw_loop(_: u32, _: u32) {
  let rx = &CHANNEL.1;
  while let Ok((d2d, draw)) = (*rx.lock().unwrap()).recv() {
    match draw {
      Draw::Ellipse(x, y, width, height) => unsafe { draw_ellipse(x, y, width, height, d2d.thickness, d2d.r, d2d.g, d2d.b, d2d.a, d2d.filled) },
      Draw::Box(x, y, width, height) => unsafe { draw_box(x, y, width, height, d2d.thickness, d2d.r, d2d.g, d2d.b, d2d.a, d2d.filled) },
      Draw::Circle(x, y, radius) => unsafe { draw_circle(x, y, radius, d2d.thickness, d2d.r, d2d.g, d2d.b, d2d.a, d2d.filled) },
      Draw::Line(x1, y1, x2, y2) => unsafe { draw_line(x1, y1, x2, y2, d2d.thickness, d2d.r, d2d.g, d2d.b, d2d.a) },
      Draw::Text(text, font_size, x, y) => unsafe { draw_string(CString::new(text.to_owned()).unwrap().as_ptr(), font_size, x, y, d2d.r, d2d.g, d2d.b, d2d.a) },
    }
  }
}

fn init(overlay: &D2dOverlay, options: Option<u32>) {
  if let Some(process) = &overlay.process {
    let process_nstr = CString::new(process.to_owned()).unwrap();
    unsafe { overlay_setup_with_process(draw_loop, process_nstr.as_ptr()); }

    if let Some(option) = options {
      unsafe { overlay_options(option); }
    }
  }
}


impl D2dOverlay {
  pub fn new_process(process: &str, options: Option<u32>) -> D2dOverlay {
    let overlay = D2dOverlay {
      thickness: 1.0,
      r: 0.0,
      g: 0.0,
      b: 0.0,
      a: 1.0,
      filled: false,
      window: None,
      process: Some(process.to_owned()),
    };

    init(&overlay, options);

    overlay
  }

  pub fn new_window(pid: u32, options: Option<u32>) -> D2dOverlay {
    let overlay = D2dOverlay {
      thickness: 1.0,
      r: 0.0,
      g: 0.0,
      b: 0.0,
      a: 1.0,
      filled: false,
      window: Some(pid),
      process: None
    };

    init(&overlay, options);

    overlay
  }

  pub fn draw_box(&self, x: f32, y: f32, width: f32, height: f32) {
    let tx = &CHANNEL.0;
    tx.lock().unwrap().send((self.clone(), Draw::Box(x, y, width, height))).unwrap();
  }

  pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
    let tx = &CHANNEL.0;
    tx.lock().unwrap().send((self.clone(), Draw::Line(x1, y1, x2, y2))).unwrap();
  }

  pub fn draw_circle(&self, x: f32, y: f32, radius: f32) {
    let tx = &CHANNEL.0;
    tx.lock().unwrap().send((self.clone(), Draw::Circle(x, y, radius))).unwrap();
  }

  pub fn draw_ellipse(&self, x: f32, y: f32, width: f32, height: f32) {
    let tx = &CHANNEL.0;
    tx.lock().unwrap().send((self.clone(), Draw::Ellipse(x, y, width, height))).unwrap();
  }

  pub fn draw_string(&self, text: &str, font_size: f32, x: f32, y: f32) {
    let tx = &CHANNEL.0;
    tx.lock().unwrap().send((self.clone(), Draw::Text(text.to_owned(), font_size, x, y))).unwrap();
  }
}
