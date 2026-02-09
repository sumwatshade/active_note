//! Build script for Embassy nRF52 project
//! Handles linking of defmt and memory.x

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
