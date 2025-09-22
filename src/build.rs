use std::{env, fs, path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=assets/logo_colorum_wordmark.svg");
    println!("cargo:rerun-if-changed=assets/logo_colorum_stacked.svg");
    println!("cargo:rerun-if-changed=assets/logo_colorum_mark.svg");

    if !cfg!(feature = "outlined-logos") { return; }

    let inkscape = env::var("INKSCAPE").unwrap_or_else(|_| "inkscape".to_string());
    let inputs = [
        "assets/logo_colorum_wordmark.svg",
        "assets/logo_colorum_stacked.svg",
        "assets/logo_colorum_mark.svg",
    ];

    for input in inputs {
        let out = input.replace(".svg", "_outlined.svg");
        if let Some(parent) = Path::new(&out).parent() { fs::create_dir_all(parent).ok(); }
        let ok = Command::new(&inkscape)
            .args(&[
                input, "--export-type=svg", "--export-plain-svg",
                "--export-text-to-path", "--export-filename", &out,
            ])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            println!("cargo:warning=Inkscape missing or failed; using non-outlined SVG for {input}");
        }
    }
}