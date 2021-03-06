use d2doverlay_sys::{
  overlay_setup_with_process_sys,
  overlay_options_sys, draw_ellipse_sys,
  draw_circle_sys,
  draw_box_sys,
  draw_line_sys,
  draw_string_sys
};

use std::ffi::CString;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
enum Draw {
  Box(f32, f32, f32, f32, f32, f32, f32, f32, f32, bool),
  Line(f32, f32, f32, f32, f32, f32, f32, f32, f32),
  Ellipse(f32, f32, f32, f32, f32, f32, f32, f32, f32, bool),
  Circle(f32, f32, f32, f32, f32, f32, f32, f32, bool),
  Text(String, f32, f32, f32, f32, f32, f32, f32)
}

const THICKNESS: f32 = 1.0;
const R: f32 = 0.0;
const G: f32 = 0.0;
const B: f32 = 0.0;
const A: f32 = 1.0;
const FILLED: bool = false;

lazy_static! {
  static ref DRAW_CALLS: Mutex<Vec<Draw>> = Mutex::new(Vec::new());
}

extern "C" fn draw_loop(_: u32, _: u32) {
  for draw in DRAW_CALLS.lock().unwrap().iter() {
     match draw {
      Draw::Ellipse(x, y, width, height, thickness, r, g, b, a, filled) => unsafe { draw_ellipse_sys(*x, *y, *width, *height, *thickness, *r, *g, *b, *a, *filled) },
      Draw::Box(x, y, width, height, thickness, r, g, b, a, filled) => unsafe { draw_box_sys(*x, *y, *width, *height, *thickness, *r, *g, *b, *a, *filled) },
      Draw::Circle(x, y, radius, thickness, r, g, b, a, filled) => unsafe { draw_circle_sys(*x, *y, *radius, *thickness, *r, *g, *b, *a, *filled) },
      Draw::Line(x1, y1, x2, y2, thickness, r, g, b, a) => unsafe { draw_line_sys(*x1, *y1, *x2, *y2, *thickness, *r, *g, *b, *a) },
      Draw::Text(text, font_size, x, y, r, g, b, a) => unsafe { draw_string_sys(CString::new(text.to_owned()).unwrap().as_ptr(), *font_size, *x,* y, *r, *g, *b, *a) },
    }
  }
}

//box

pub fn draw_box(x: f32, y: f32, width: f32, height: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Box(x, y, width, height, THICKNESS, R, G, B, A, FILLED));
}

pub fn draw_box_with_border(x: f32, y: f32, width: f32, height: f32, thickness: f32, filled: bool) {
  DRAW_CALLS.lock().unwrap().push(Draw::Box(x, y, width, height, thickness, R, G, B, A, filled));
}

pub fn draw_box_with_rbga(x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Box(x, y, width, height, THICKNESS, r, g, b, a, FILLED));
}

pub fn draw_box_full(x: f32, y: f32, width: f32, height: f32, thickness: f32, r: f32, g: f32, b: f32, a: f32, filled: bool) {
  DRAW_CALLS.lock().unwrap().push(Draw::Box(x, y, width, height, thickness, r, g, b, a, filled));
}

//ellipse

pub fn draw_ellipse(x: f32, y: f32, width: f32, height: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Ellipse(x, y, width, height, THICKNESS, R, G, B, A, FILLED));
}

pub fn  draw_ellips_with_border(x: f32, y: f32, width: f32, height: f32, thickness: f32, filled: bool) {
  DRAW_CALLS.lock().unwrap().push(Draw::Ellipse(x, y, width, height, thickness, R, G, B, A, filled));
}

pub fn draw_ellips_with_rbga(x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Ellipse(x, y, width, height, THICKNESS, r, g, b, a, FILLED));
}

pub fn draw_ellips_full(x: f32, y: f32, width: f32, height: f32, thickness: f32, r: f32, g: f32, b: f32, a: f32, filled: bool) {
  DRAW_CALLS.lock().unwrap().push(Draw::Ellipse(x, y, width, height, thickness, r, g, b, a, filled));
}

//line

pub fn draw_line(x: f32, y: f32, width: f32, height: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Line(x, y, width, height, THICKNESS, R, G, B, A));
}

pub fn draw_line_with_rbga(x: f32, y: f32, width: f32, height: f32, r: f32, g: f32, b: f32, a: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Line(x, y, width, height, THICKNESS, r, g, b, a));
}

pub fn draw_line_full(x: f32, y: f32, width: f32, height: f32, thickness: f32, r: f32, g: f32, b: f32, a: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Line(x, y, width, height, thickness, r, g, b, a));
}

//circle

pub fn draw_circle(x: f32, y: f32, radius: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Circle(x, y, radius, THICKNESS, R, G, B, A, FILLED));
}

pub fn draw_circle_with_border(x: f32, y: f32, radius: f32, thickness: f32, filled: bool) {
  DRAW_CALLS.lock().unwrap().push(Draw::Circle(x, y, radius, thickness, R, G, B, A, filled));
}

pub fn draw_circle_with_rbga(x: f32, y: f32, radius: f32, r: f32, g: f32, b: f32, a: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Circle(x, y, radius, THICKNESS, r, g, b, a, FILLED));
}

pub fn draw_circle_full(x: f32, y: f32, radius: f32, thickness: f32, r: f32, g: f32, b: f32, a: f32, filled: bool) {
  DRAW_CALLS.lock().unwrap().push(Draw::Circle(x, y, radius, thickness, r, g, b, a, filled));
}

//text

pub fn draw_text(text: &str, font_size: f32, x: f32, y: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Text(text.to_owned(), font_size, x, y, R, G, B, A));
}

pub fn draw_text_with_rbga(text: &str, font_size: f32, x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) {
  DRAW_CALLS.lock().unwrap().push(Draw::Text(text.to_owned(), font_size, x, y, r, g, b, a));
}

pub fn redraw() {
  DRAW_CALLS.lock().unwrap().clear();
}

pub fn overlay_with_process(process: &str, options: Option<u32>) {
  let process_nstr = CString::new(process.to_owned()).unwrap();
  unsafe { overlay_setup_with_process_sys(draw_loop, process_nstr.as_ptr()); }

  if let Some(option) = options {
    unsafe { overlay_options_sys(option); }
  }
}
