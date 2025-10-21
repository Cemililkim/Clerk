fn main() {
  // Set the manifest directory as an environment variable for runtime use
  let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  println!("cargo:rustc-env=TAURI_MANIFEST_DIR={}", manifest_dir);
  
  tauri_build::build()
}
