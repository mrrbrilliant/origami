use std::env;
use std::process::Command;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    #[cfg(not(debug_assertions))]
    Command::new("cargo")
        .args(&[
            "generate",
            "--verbose",
            "--git",
            "https://github.com/mrrbrilliant/origami.git",
            "packages/origami-template",
            "--name",
        ])
        .args(&args)
        .output()
        .expect("Failed to create project");
    #[cfg(debug_assertions)]
    Command::new("cargo")
        .args(&[
            "generate",
            "--verbose",
            "--path",
            "packages/origami-template",
            "--name",
        ])
        .args(&args)
        .output()
        .expect("Failed to create project");
}
