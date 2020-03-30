use winapi::shared::windef::HWND;
use std::os::raw::c_char;

type DirectOverlayCallback = extern "C" fn(u32, u32);

pub const D2DOV_REQUIRE_FOREGROUND: u32 = 1 << 0;
// Draws the FPS of the overlay in the top-right corner
pub const  D2DOV_DRAW_FPS: u32 = 1 << 1;
// Attempts to limit the frametimes so you don't render at 500fps
pub const D2DOV_VSYNC: u32 = 1 << 2;
// Sets the text font to Calibri
pub const  D2DOV_FONT_CALIBRI: u32 = 1 << 3;
// Sets the text font to Arial
pub const  D2DOV_FONT_ARIAL: u32 = 1 << 4;
// Sets the text font to Courier
pub const  D2DOV_FONT_COURIER: u32 = 1 << 5;
// Sets the text font to Gabriola
pub const  D2DOV_FONT_GABRIOLA: u32	= 1 << 6;
// Sets the text font to Impact
pub const  D2DOV_FONT_IMPACT: u32 = 1 << 7;

extern {
  pub fn overlay_options(option : u32);

  pub fn overlay_setup(callback: DirectOverlayCallback);
  pub fn overlay_setup_with_window(callback: DirectOverlayCallback, window: HWND);
  pub fn overlay_setup_with_process(callback: DirectOverlayCallback, process: *const c_char);

  pub fn draw_box(x: f32, y: f32, width: f32, height: f32, thickness: f32, r: f32, g: f32, b: f32, a: f32, filled : bool);

  pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, r: f32, g: f32, b: f32, a: f32);
  // Draws a rectangle on the screen.  Width and height are relative to the coordinates of the box.
  // Use the "filled" bool to make it a solid rectangle; ignore the thickness.
  // To just draw the border around the rectangle, specify a thickness and pass "filled" as false.
  // Draws a circle.  As with the DrawBox, the "filled" bool will make it a solid circle, and thickness is only used when filled=false.
  pub fn draw_circle(x: f32, y: f32, radius: f32, thickness: f32, r: f32, g: f32, b: f32, a: f32, filled: bool);

  // Allows you to draw an elipse.  Same as a circle, except you have two different radii, for width and height.
  pub fn draw_ellipse(x: f32, y: f32, width: f32, height: f32, thickness: f32, r: f32, g: f32, b: f32, a: f32, filled: bool);

  // Draw a string on the screen.  Input is in the form of an std::string.
  pub fn draw_string(str : *const c_char, fontSize: f32, x: f32, y: f32, r: f32, g: f32, b: f32, a: f32);
}
