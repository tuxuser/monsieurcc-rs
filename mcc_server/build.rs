use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=\"frontend\"");

    let mut frontend_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    frontend_path.push("frontend");

    let result = Command::new("npm")
        .args(["run", "build"])
        .current_dir(frontend_path)
        .output()
        .expect("npm build failed to execute");

    eprintln!("{:?}", result);
}
