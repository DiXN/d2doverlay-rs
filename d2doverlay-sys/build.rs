use std::env;

fn main() {
  let mut build = cc::Build::new();

  let target = env::var("TARGET").unwrap();

  build
    .include("overlay.h")
    .flag_if_supported("-std=c++11");

  build.file("src\\source.cpp");

  for &lib in &["d2d1", "dwmapi", "dwrite"] {
    println!("cargo:rustc-link-lib={}", lib);
  }

  build.compile("D2DOverlay");
}