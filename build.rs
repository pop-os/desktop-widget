use std::{
    env,
    fs,
    process,
};

fn main() {
    println!("cargo:rerun-if-changed=data");
    for entry_res in fs::read_dir("data").expect("Failed to read data dir") {
        let entry = entry_res.expect("Failed to read data entry");
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    let out_dir = env::var("OUT_DIR").unwrap();
	let status = process::Command::new("glib-compile-resources")
		.arg("--sourcedir=data")
		.arg(format!("--target={}/compiled.gresource", out_dir))
		.arg("data/resources.gresource.xml")
        .status()
        .expect("Failed to run glib-compile-resources");
    if ! status.success() {
        panic!("glib-compile-resources exited with status {}", status);
    }
}