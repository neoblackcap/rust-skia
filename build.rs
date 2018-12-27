extern crate bindgen;
extern crate cc;

use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use cc::Build;

fn main() {
  Command::new("git")
    .arg("submodule")
    .arg("init")
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .unwrap();

  Command::new("git")
    .args(&["submodule", "update", "--recursive"])
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .unwrap();

  Command::new("python")
    .arg("skia/tools/git-sync-deps")
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .unwrap();

  let output = Command::new("bin/gn")
    .args(&[
      "gen",
      "out/Static",
      r#"--args=is_official_build=true skia_use_system_expat=false skia_use_system_icu=false skia_use_system_libjpeg_turbo=false skia_use_system_libpng=false skia_use_system_libwebp=false skia_use_system_zlib=false cc="clang" cxx="clang++""#
    ])
    .envs(env::vars())
    .current_dir(PathBuf::from("./skia"))
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()
    .expect("gn error");

  if output.status.code() != Some(0) {
    panic!("{:?}", String::from_utf8(output.stdout).unwrap());
  }

  Command::new("ninja")
    .current_dir(PathBuf::from("./skia"))
    .args(&["-C", "out/Static"])
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .expect("ninja error");

  let current_dir = env::current_dir().unwrap();
  let current_dir_name = current_dir.to_str().unwrap();

  println!("cargo:include={}/skia/include/c", &current_dir_name);
  println!(
    "cargo:rustc-link-search={}/skia/out/Static",
    &current_dir_name
  );
  println!("cargo:rustc-link-lib=static=skia");
  println!("cargo:rustc-link-lib=static=skiabinding");

  let target = env::var("TARGET").unwrap();
  if target.contains("unknown-linux-gnu") {
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-link-lib=GL");
  } else if target.contains("eabi") {
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=GLESv2");
  } else if target.contains("apple-darwin") {
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=framework=OpenGL");
    println!("cargo:rustc-link-lib=framework=ApplicationServices");
  } else if target.contains("windows") {
    if target.contains("gnu") {
      println!("cargo:rustc-link-lib=stdc++");
    }
    println!("cargo:rustc-link-lib=usp10");
    println!("cargo:rustc-link-lib=ole32");
  }

  if env::var("INIT_SKIA").is_ok() {
    bindgen_gen(&current_dir_name);
  }
}

fn bindgen_gen(current_dir_name: &str) {
  let mut builder = bindgen::Builder::default()
    .generate_inline_functions(true)
    .whitelist_function("SkiaCreateCanvas")
    .whitelist_function("SkiaCreateRect")
    .whitelist_function("SkiaClearCanvas")
    .whitelist_function("SkiaGetSurfaceData")
    .whitelist_var("SK_ColorWHITE")
    .whitelist_var("SK_ColorBLUE")
    .clang_arg("-std=c++14");

  let mut cc_build = Build::new();

  builder = builder.header("src/bindings.hpp");

  for include_dir in read_dir("skia/include").expect("Unable to read skia/include") {
    let dir = include_dir.unwrap();
    let include_path = format!("{}/{}", &current_dir_name, &dir.path().to_str().unwrap());
    builder = builder.clang_arg(format!("-I{}", &include_path));
    cc_build.include(&include_path);
  }

  cc_build
    .cpp(true)
    .flag("-std=c++14")
    .file("src/bindings.cc")
    .out_dir("skia/out/Static")
    .compile("skiabinding");

  let bindings = builder.generate().expect("Unable to generate bindings");

  let out_path = PathBuf::from("src");
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
